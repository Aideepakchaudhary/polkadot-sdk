// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! A pallet that can be used instead of `SovereignPaidRemoteExporter` (or others) in the XCM router
//! configuration. The main feature this pallet offers is a dynamic message fee,
//! which is computed based on the state of the bridge queues. The fee increases exponentially
//! if the queue between this chain and the sibling or child bridge hub becomes congested.
//!
//! All other bridge hub queues offer backpressure mechanisms, so if any of these
//! queues are congested, it will eventually lead to increased queuing on this chain.
//!
//! **Note on Terminology**: When we refer to the bridge hub here, we mean the chain that
//! has the `pallet-bridge-messages` with an `ExportXcm` implementation deployed, e.g., provided by
//! `pallet-xcm-bridge-hub`. Depending on the deployment setup, `T::ToBridgeHubSender` can be
//! configured accordingly - see `T::ToBridgeHubSender` for more documentation.

#![cfg_attr(not(feature = "std"), no_std)]

pub use bp_xcm_bridge_hub_router::XcmChannelStatusProvider;
use codec::Encode;
use frame_support::traits::Get;
use sp_runtime::{FixedPointNumber, FixedU128, Saturating};
use sp_std::vec::Vec;
use xcm::prelude::*;
use xcm_builder::{ExporterFor, InspectMessageQueues};

pub use pallet::*;
pub use weights::WeightInfo;

pub mod benchmarking;
pub mod weights;

mod mock;

/// Minimal delivery fee factor.
pub const MINIMAL_DELIVERY_FEE_FACTOR: FixedU128 = FixedU128::from_u32(1);

/// The factor that is used to increase current message fee factor when bridge experiencing
/// some lags.
const EXPONENTIAL_FEE_BASE: FixedU128 = FixedU128::from_rational(105, 100); // 1.05
/// The factor that is used to increase current message fee factor for every sent kilobyte.
const MESSAGE_SIZE_FEE_BASE: FixedU128 = FixedU128::from_rational(1, 1000); // 0.001

/// Maximal size of the XCM message that may be sent over bridge.
///
/// This should be less than the maximal size, allowed by the messages pallet, because
/// the message itself is wrapped in other structs and is double encoded.
pub const HARD_MESSAGE_SIZE_LIMIT: u32 = 32 * 1024;

/// The target that will be used when publishing logs related to this pallet.
///
/// This doesn't match the pattern used by other bridge pallets (`runtime::bridge-*`). But this
/// pallet has significant differences with those pallets. The main one is that is intended to
/// be deployed at sending chains. Other bridge pallets are likely to be deployed at the separate
/// bridge hub parachain.
pub const LOG_TARGET: &str = "runtime::bridge-xcm-router";

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Benchmarks results from runtime we're plugged into.
		type WeightInfo: WeightInfo;

		/// Universal location of this runtime.
		type UniversalLocation: Get<InteriorLocation>;
		/// Relative location of the supported sibling bridge hub.
		type SiblingBridgeHubLocation: Get<Location>;
		/// The bridged network that this config is for if specified.
		/// Also used for filtering `Bridges` by `BridgedNetworkId`.
		/// If not specified, allows all networks pass through.
		type BridgedNetworkId: Get<Option<NetworkId>>;
		/// Configuration for supported **bridged networks/locations** with **bridge location** and
		/// **possible fee**. Allows to externalize better control over allowed **bridged
		/// networks/locations**.
		type Bridges: ExporterFor;
		/// Checks the XCM version for the destination.
		type DestinationVersion: GetVersion;

		/// The bridge hub may be:
		/// - A system (sibling) bridge hub parachain (or another chain), in which case we need an
		///   implementation for `T::ToBridgeHubSender` that sends `ExportMessage`, e.g.,
		///   `SovereignPaidRemoteExporter`.
		/// - The local chain, in which case we need an implementation for `T::ToBridgeHubSender`
		///   that does not use `ExportMessage` but instead directly calls the `ExportXcm`
		///   implementation.
		type ToBridgeHubSender: SendXcm;
		/// Local XCM channel manager.
		type LocalXcmChannelManager: XcmChannelStatusProvider;

		/// Additional fee that is paid for every byte of the outbound message.
		/// See `calculate_fees` for more details.
		type ByteFee: Get<u128>;
		/// Asset used to pay the `ByteFee`.
		/// If not specified, the `ByteFee` is ignored.
		/// See `calculate_fees` for more details.
		type FeeAsset: Get<Option<AssetId>>;
	}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			// if XCM channel is still congested, we don't change anything
			if T::LocalXcmChannelManager::is_congested(&T::SiblingBridgeHubLocation::get()) {
				return T::WeightInfo::on_initialize_when_congested()
			}

			// if we can't decrease the delivery fee factor anymore, we don't change anything
			let mut delivery_fee_factor = Self::delivery_fee_factor();
			if delivery_fee_factor == MINIMAL_DELIVERY_FEE_FACTOR {
				return T::WeightInfo::on_initialize_when_congested()
			}

			let previous_factor = delivery_fee_factor;
			delivery_fee_factor =
				MINIMAL_DELIVERY_FEE_FACTOR.max(delivery_fee_factor / EXPONENTIAL_FEE_BASE);
			log::info!(
				target: LOG_TARGET,
				"Bridge channel is uncongested. Decreased fee factor from {} to {}",
				previous_factor,
				delivery_fee_factor,
			);
			Self::deposit_event(Event::DeliveryFeeFactorDecreased {
				new_value: delivery_fee_factor,
			});

			DeliveryFeeFactor::<T, I>::put(delivery_fee_factor);

			T::WeightInfo::on_initialize_when_non_congested()
		}
	}

	/// Initialization value for the delivery fee factor.
	#[pallet::type_value]
	pub fn InitialFactor() -> FixedU128 {
		MINIMAL_DELIVERY_FEE_FACTOR
	}

	/// The number to multiply the base delivery fee by.
	///
	/// This factor is shared by all bridges, served by this pallet. For example, if this
	/// chain (`Config::UniversalLocation`) opens two bridges (
	/// `X2(GlobalConsensus(Config::BridgedNetworkId::get()), Parachain(1000))` and
	/// `X2(GlobalConsensus(Config::BridgedNetworkId::get()), Parachain(2000))`), then they
	/// both will be sharing the same fee factor. This is because both bridges are sharing
	/// the same local XCM channel with the child/sibling bridge hub, which we are using
	/// to detect congestion:
	///
	/// ```nocompile
	///  ThisChain --- Local XCM channel --> Sibling Bridge Hub ------
	///                                            |                   |
	///                                            |                   |
	///                                            |                   |
	///                                          Lane1               Lane2
	///                                            |                   |
	///                                            |                   |
	///                                            |                   |
	///                                           \ /                  |
	///  Parachain1  <-- Local XCM channel --- Remote Bridge Hub <------
	///                                            |
	///                                            |
	///  Parachain1  <-- Local XCM channel ---------
	/// ```
	///
	/// If at least one of other channels is congested, the local XCM channel with sibling
	/// bridge hub eventually becomes congested too. And we have no means to detect - which
	/// bridge exactly causes the congestion. So the best solution here is not to make
	/// any differences between all bridges, started by this chain.
	#[pallet::storage]
	#[pallet::getter(fn delivery_fee_factor)]
	pub type DeliveryFeeFactor<T: Config<I>, I: 'static = ()> =
		StorageValue<_, FixedU128, ValueQuery, InitialFactor>;

	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Called when new message is sent (queued to local outbound XCM queue) over the bridge.
		pub(crate) fn on_message_sent_to_bridge(message_size: u32) {
			// if outbound channel is not congested, do nothing
			if !T::LocalXcmChannelManager::is_congested(&T::SiblingBridgeHubLocation::get()) {
				return
			}

			// ok - we need to increase the fee factor, let's do that
			let message_size_factor = FixedU128::from_u32(message_size.saturating_div(1024))
				.saturating_mul(MESSAGE_SIZE_FEE_BASE);
			let total_factor = EXPONENTIAL_FEE_BASE.saturating_add(message_size_factor);
			DeliveryFeeFactor::<T, I>::mutate(|f| {
				let previous_factor = *f;
				*f = f.saturating_mul(total_factor);
				log::info!(
					target: LOG_TARGET,
					"Bridge channel is congested. Increased fee factor from {} to {}",
					previous_factor,
					f,
				);
				Self::deposit_event(Event::DeliveryFeeFactorIncreased { new_value: *f });
				*f
			});
		}

		/// Calculates final fees based on the configuration, supporting two optional features:
		///
		/// 1. Adds an (optional) fee for message size based on `T::ByteFee` and `T::FeeAsset`.
		/// 2. Applies a dynamic fee factor `Self::delivery_fee_factor` to the `actual_cost` (useful for congestion handling).
		///
		/// Parameters:
		/// - `message_size`
		/// - `actual_cost`: the fee for sending a message from this chain to a child, sibling, or local bridge hub, determined by `Config::ToBridgeHubSender`.
		pub(crate) fn calculate_fees(message_size: u32, mut actual_cost: Assets) -> Assets {
			// Apply message size `T::ByteFee/T::FeeAsset` feature (if configured).
			if let Some(asset_id) = T::FeeAsset::get() {
				let message_fee = (message_size as u128).saturating_mul(T::ByteFee::get());
				if message_fee > 0 {
					// Keep in mind that this is only the additional fee for message size.
					actual_cost.push((asset_id, message_fee).into());
				}
			}

			// Apply dynamic fees feature - apply `fee_factor` to the `actual_cost`.
			let fee_factor = Self::delivery_fee_factor();
			let mut new_cost = Assets::new();
			for mut a in actual_cost.into_inner() {
				if let Fungibility::Fungible(ref mut amount) = a.fun {
					*amount = fee_factor.saturating_mul_int(*amount);
				}
				new_cost.push(a);
			}
			new_cost
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Delivery fee factor has been decreased.
		DeliveryFeeFactorDecreased {
			/// New value of the `DeliveryFeeFactor`.
			new_value: FixedU128,
		},
		/// Delivery fee factor has been increased.
		DeliveryFeeFactorIncreased {
			/// New value of the `DeliveryFeeFactor`.
			new_value: FixedU128,
		},
	}
}

// This pallet acts as the `SendXcm` to the sibling/child bridge hub instead of regular
// XCMP/DMP transport. This allows injecting dynamic message fees into XCM programs that
// are going to the bridged network.
impl<T: Config<I>, I: 'static> SendXcm for Pallet<T, I> {
	type Ticket = (u32, <T::ToBridgeHubSender as SendXcm>::Ticket);

	fn validate(
		dest: &mut Option<Location>,
		xcm: &mut Option<Xcm<()>>,
	) -> SendResult<Self::Ticket> {
		log::trace!(target: LOG_TARGET, "validate - msg: {xcm:?}, destination: {dest:?}");

		// In case of success, the `T::ToBridgeHubSender` can modify XCM instructions and consume
		// `dest` / `xcm`, so we retain the clone of original message and the destination for later
		// `DestinationVersion` validation.
		let xcm_to_dest_clone = xcm.clone();
		let dest_clone = dest.clone();

		// First, use the inner exporter to validate the destination to determine if it is even
		// routable. If it is not, return an error. If it is, then the XCM is extended with
		// instructions to pay the message fee at the sibling/child bridge hub. The cost will
		// include both the cost of (1) delivery to the sibling bridge hub (returned by
		// `Config::ToBridgeHubSender`) and (2) delivery to the bridged bridge hub (returned by
		// `Self::exporter_for`).
		match T::ToBridgeHubSender::validate(dest, xcm) {
			Ok((ticket, cost)) => {
				// If the ticket is ok, it means we are routing with this router, so we need to
				// apply more validations to the cloned `dest` and `xcm`, which are required here.
				let xcm_to_dest_clone = xcm_to_dest_clone.ok_or(SendError::MissingArgument)?;
				let dest_clone = dest_clone.ok_or(SendError::MissingArgument)?;

				// We won't have access to `dest` and `xcm` in the `deliver` method, so we need to
				// precompute everything required here. However, `dest` and `xcm` were consumed by
				// `T::ToBridgeHubSender`, so we need to use their clones.
				let message_size = xcm_to_dest_clone.encoded_size() as _;

				// The bridge doesn't support oversized or overweight messages. Therefore, it's
				// better to drop such messages here rather than at the bridge hub. Let's check the
				// message size.
				if message_size > HARD_MESSAGE_SIZE_LIMIT {
					return Err(SendError::ExceedsMaxMessageSize)
				}

				// We need to ensure that the known `dest`'s XCM version can comprehend the current
				// `xcm` program. This may seem like an additional, unnecessary check, but it is
				// not. A similar check is probably performed by the `T::ToBridgeHubSender`, which
				// attempts to send a versioned message to the sibling bridge hub. However, the
				// local bridge hub may have a higher XCM version than the remote `dest`. Once
				// again, it is better to discard such messages here than at the bridge hub (e.g.,
				// to avoid losing funds).
				let destination_version = T::DestinationVersion::get_version_for(&dest_clone)
					.ok_or(SendError::DestinationUnsupported)?;
				let _ = VersionedXcm::from(xcm_to_dest_clone)
					.into_version(destination_version)
					.map_err(|()| SendError::DestinationUnsupported)?;

				// now let's apply fees features
				let cost = Self::calculate_fees(message_size, cost);

				log::info!(
					target: LOG_TARGET,
					"Going to send message to {dest_clone:?} ({message_size:?} bytes) with actual cost: {cost:?}"
				);

				Ok(((message_size, ticket), cost))
			},
			Err(e) => {
				log::trace!(target: LOG_TARGET, "`T::ToBridgeHubSender` validates for dest: {dest_clone:?} with error: {e:?}");
				Err(e)
			},
		}
	}

	fn deliver(ticket: Self::Ticket) -> Result<XcmHash, SendError> {
		// use router to enqueue message to the sibling/child bridge hub. This also should handle
		// payment for passing through this queue.
		let (message_size, ticket) = ticket;
		let xcm_hash = T::ToBridgeHubSender::deliver(ticket)?;

		// increase delivery fee factor if required
		Self::on_message_sent_to_bridge(message_size);

		log::trace!(target: LOG_TARGET, "deliver - message sent, xcm_hash: {xcm_hash:?}");
		Ok(xcm_hash)
	}
}

impl<T: Config<I>, I: 'static> InspectMessageQueues for Pallet<T, I> {
	fn clear_messages() {}

	/// This router needs to implement `InspectMessageQueues` but doesn't have to
	/// return any messages, since it just reuses the `XcmpQueue` router.
	fn get_messages() -> Vec<(VersionedLocation, Vec<VersionedXcm<()>>)> {
		Vec::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::assert_ok;
	use mock::*;

	use frame_support::traits::Hooks;
	use frame_system::{EventRecord, Phase};
	use sp_runtime::traits::One;

	#[test]
	fn initial_fee_factor_is_one() {
		run_test(|| {
			assert_eq!(DeliveryFeeFactor::<TestRuntime, ()>::get(), MINIMAL_DELIVERY_FEE_FACTOR);
		})
	}

	#[test]
	fn fee_factor_is_not_decreased_from_on_initialize_when_xcm_channel_is_congested() {
		run_test(|| {
			DeliveryFeeFactor::<TestRuntime, ()>::put(FixedU128::from_rational(125, 100));
			TestLocalXcmChannelManager::make_congested(&SiblingBridgeHubLocation::get());

			// it should not decrease, because queue is congested
			let old_delivery_fee_factor = XcmBridgeHubRouter::delivery_fee_factor();
			XcmBridgeHubRouter::on_initialize(One::one());
			assert_eq!(XcmBridgeHubRouter::delivery_fee_factor(), old_delivery_fee_factor);

			assert_eq!(System::events(), vec![]);
		})
	}

	#[test]
	fn fee_factor_is_decreased_from_on_initialize_when_xcm_channel_is_uncongested() {
		run_test(|| {
			let initial_fee_factor = FixedU128::from_rational(125, 100);
			DeliveryFeeFactor::<TestRuntime, ()>::put(initial_fee_factor);

			// it shold eventually decreased to one
			while XcmBridgeHubRouter::delivery_fee_factor() > MINIMAL_DELIVERY_FEE_FACTOR {
				XcmBridgeHubRouter::on_initialize(One::one());
			}

			// verify that it doesn't decreases anymore
			XcmBridgeHubRouter::on_initialize(One::one());
			assert_eq!(XcmBridgeHubRouter::delivery_fee_factor(), MINIMAL_DELIVERY_FEE_FACTOR);

			// check emitted event
			let first_system_event = System::events().first().cloned();
			assert_eq!(
				first_system_event,
				Some(EventRecord {
					phase: Phase::Initialization,
					event: RuntimeEvent::XcmBridgeHubRouter(Event::DeliveryFeeFactorDecreased {
						new_value: initial_fee_factor / EXPONENTIAL_FEE_BASE,
					}),
					topics: vec![],
				})
			);
		})
	}

	#[test]
	fn not_applicable_if_destination_is_within_other_network() {
		run_test(|| {
			// unroutable dest
			let dest = Location::new(2, [GlobalConsensus(ByGenesis([0; 32])), Parachain(1000)]);
			let xcm: Xcm<()> = vec![ClearOrigin].into();

			// check that router does not consume when `NotApplicable`
			let mut xcm_wrapper = Some(xcm.clone());
			assert_eq!(
				XcmBridgeHubRouter::validate(&mut Some(dest.clone()), &mut xcm_wrapper),
				Err(SendError::NotApplicable),
			);
			// XCM is NOT consumed and untouched
			assert_eq!(Some(xcm.clone()), xcm_wrapper);

			// check the full `send_xcm`
			assert_eq!(send_xcm::<XcmBridgeHubRouter>(dest, xcm,), Err(SendError::NotApplicable),);
		});
	}

	#[test]
	fn exceeds_max_message_size_if_size_is_above_hard_limit() {
		run_test(|| {
			// routable dest with XCM version
			let dest =
				Location::new(2, [GlobalConsensus(BridgedNetworkId::get()), Parachain(1000)]);
			// oversized XCM
			let xcm: Xcm<()> = vec![ClearOrigin; HARD_MESSAGE_SIZE_LIMIT as usize].into();

			// dest is routable with the inner router
			assert_ok!(<TestRuntime as Config<()>>::ToBridgeHubSender::validate(
				&mut Some(dest.clone()),
				&mut Some(xcm.clone())
			));

			// check for oversized message
			let mut xcm_wrapper = Some(xcm.clone());
			assert_eq!(
				XcmBridgeHubRouter::validate(&mut Some(dest.clone()), &mut xcm_wrapper),
				Err(SendError::ExceedsMaxMessageSize),
			);
			// XCM is consumed by the inner router
			assert!(xcm_wrapper.is_none());

			// check the full `send_xcm`
			assert_eq!(
				send_xcm::<XcmBridgeHubRouter>(dest, xcm,),
				Err(SendError::ExceedsMaxMessageSize),
			);
		});
	}

	#[test]
	fn destination_unsupported_if_wrap_version_fails() {
		run_test(|| {
			// routable dest but we don't know XCM version
			let dest = UnknownXcmVersionForRoutableLocation::get();
			let xcm: Xcm<()> = vec![ClearOrigin].into();

			// dest is routable with the inner router
			assert_ok!(<TestRuntime as Config<()>>::ToBridgeHubSender::validate(
				&mut Some(dest.clone()),
				&mut Some(xcm.clone())
			));

			// check that it does not pass XCM version check
			let mut xcm_wrapper = Some(xcm.clone());
			assert_eq!(
				XcmBridgeHubRouter::validate(&mut Some(dest.clone()), &mut xcm_wrapper),
				Err(SendError::DestinationUnsupported),
			);
			// XCM is consumed by the inner router
			assert!(xcm_wrapper.is_none());

			// check the full `send_xcm`
			assert_eq!(
				send_xcm::<XcmBridgeHubRouter>(dest, xcm,),
				Err(SendError::DestinationUnsupported),
			);
		});
	}

	#[test]
	fn returns_proper_delivery_price() {
		run_test(|| {
			let dest = Location::new(2, [GlobalConsensus(BridgedNetworkId::get())]);
			let xcm: Xcm<()> = vec![ClearOrigin].into();
			let msg_size = xcm.encoded_size();

			// `BASE_FEE + BYTE_FEE * msg_size + HRMP_FEE`
			let base_cost_formula = || BASE_FEE + BYTE_FEE * (msg_size as u128) + HRMP_FEE;

			// initially the base fee is used
			let expected_fee = base_cost_formula();
			assert_eq!(
				XcmBridgeHubRouter::validate(&mut Some(dest.clone()), &mut Some(xcm.clone()))
					.unwrap()
					.1
					.get(0),
				Some(&(BridgeFeeAsset::get(), expected_fee).into()),
			);

			// but when factor is larger than one, it increases the fee, so it becomes:
			// `base_cost_formula() * F`
			let factor = FixedU128::from_rational(125, 100);
			DeliveryFeeFactor::<TestRuntime, ()>::put(factor);
			let expected_fee =
				(FixedU128::saturating_from_integer(base_cost_formula()) * factor)
					.into_inner() / FixedU128::DIV;
			assert_eq!(
				XcmBridgeHubRouter::validate(&mut Some(dest), &mut Some(xcm)).unwrap().1.get(0),
				Some(&(BridgeFeeAsset::get(), expected_fee).into()),
			);
		});
	}

	#[test]
	fn sent_message_doesnt_increase_factor_if_queue_is_uncongested() {
		run_test(|| {
			let old_delivery_fee_factor = XcmBridgeHubRouter::delivery_fee_factor();
			assert_eq!(
				send_xcm::<XcmBridgeHubRouter>(
					Location::new(2, [GlobalConsensus(BridgedNetworkId::get()), Parachain(1000)]),
					vec![ClearOrigin].into(),
				)
				.map(drop),
				Ok(()),
			);

			assert!(TestToBridgeHubSender::is_message_sent());
			assert_eq!(old_delivery_fee_factor, XcmBridgeHubRouter::delivery_fee_factor());

			assert_eq!(System::events(), vec![]);
		});
	}

	#[test]
	fn sent_message_increases_factor_if_xcm_channel_is_congested() {
		run_test(|| {
			TestLocalXcmChannelManager::make_congested(&SiblingBridgeHubLocation::get());

			let old_delivery_fee_factor = XcmBridgeHubRouter::delivery_fee_factor();
			assert_ok!(send_xcm::<XcmBridgeHubRouter>(
				Location::new(2, [GlobalConsensus(BridgedNetworkId::get()), Parachain(1000)]),
				vec![ClearOrigin].into(),
			)
			.map(drop));

			assert!(TestToBridgeHubSender::is_message_sent());
			assert!(old_delivery_fee_factor < XcmBridgeHubRouter::delivery_fee_factor());

			// check emitted event
			let first_system_event = System::events().first().cloned();
			assert!(matches!(
				first_system_event,
				Some(EventRecord {
					phase: Phase::Initialization,
					event: RuntimeEvent::XcmBridgeHubRouter(
						Event::DeliveryFeeFactorIncreased { .. }
					),
					..
				})
			));
		});
	}

	#[test]
	fn get_messages_does_not_return_anything() {
		run_test(|| {
			assert_ok!(send_xcm::<XcmBridgeHubRouter>(
				(Parent, Parent, GlobalConsensus(BridgedNetworkId::get()), Parachain(1000)).into(),
				vec![ClearOrigin].into()
			));
			assert_eq!(XcmBridgeHubRouter::get_messages(), vec![]);
		});
	}
}
