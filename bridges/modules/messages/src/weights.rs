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

//! Autogenerated weights for pallet_bridge_messages
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `serban-ROG-Zephyrus`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/unknown-bridge-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_messages
// --extrinsic=*
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --output=./modules/messages/src/weights.rs
// --template=./.maintain/bridge-weight-template.hbs

#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bridge_messages.
pub trait WeightInfo {
	fn receive_single_message_proof() -> Weight;
	fn receive_n_messages_proof(n: u32) -> Weight;
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight;
	fn receive_single_message_n_kb_proof(n: u32) -> Weight;
	fn receive_delivery_proof_for_single_message() -> Weight;
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight;
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight;
	fn receive_single_message_n_bytes_proof_with_dispatch(n: u32) -> Weight;
}

/// Weights for `pallet_bridge_messages` that are generated using one of the Bridge testnets.
///
/// Those weights are test only and must never be used in production.
pub struct BridgeWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BridgeWeight<T> {
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 34_644 nanoseconds.
		Weight::from_parts(36_135_000, 52645)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	///
	/// The range of component `n` is `[1, 1004]`.
	///
	/// The range of component `n` is `[1, 1004]`.
	fn receive_n_messages_proof(n: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 35_330 nanoseconds.
		Weight::from_parts(27_526_047, 52645)
			// Standard Error: 2_681
			.saturating_add(Weight::from_parts(7_412_923, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 41_123 nanoseconds.
		Weight::from_parts(43_023_000, 52645)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	///
	/// The range of component `n` is `[1, 16]`.
	///
	/// The range of component `n` is `[1, 16]`.
	fn receive_single_message_n_kb_proof(n: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 36_301 nanoseconds.
		Weight::from_parts(37_103_459, 52645)
			// Standard Error: 4_645
			.saturating_add(Weight::from_parts(1_172_720, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `515`
		//  Estimated: `3530`
		// Minimum execution time: 33_941 nanoseconds.
		Weight::from_parts(35_252_000, 3530)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `532`
		//  Estimated: `3530`
		// Minimum execution time: 33_259 nanoseconds.
		Weight::from_parts(34_558_000, 3530)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:2 w:2)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `532`
		//  Estimated: `6070`
		// Minimum execution time: 35_199 nanoseconds.
		Weight::from_parts(36_989_000, 6070)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	///
	/// The range of component `n` is `[128, 2048]`.
	fn receive_single_message_n_bytes_proof_with_dispatch(n: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 75_228 nanoseconds.
		Weight::from_parts(62_255_691, 52645)
			// Standard Error: 2_005
			.saturating_add(Weight::from_parts(353_141, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 34_644 nanoseconds.
		Weight::from_parts(36_135_000, 52645)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeRialtoMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	///
	/// The range of component `n` is `[1, 1004]`.
	///
	/// The range of component `n` is `[1, 1004]`.
	fn receive_n_messages_proof(n: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 35_330 nanoseconds.
		Weight::from_parts(27_526_047, 52645)
			// Standard Error: 2_681
			.saturating_add(Weight::from_parts(7_412_923, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 41_123 nanoseconds.
		Weight::from_parts(43_023_000, 52645)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeRialtoMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	///
	/// The range of component `n` is `[1, 16]`.
	///
	/// The range of component `n` is `[1, 16]`.
	fn receive_single_message_n_kb_proof(n: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 36_301 nanoseconds.
		Weight::from_parts(37_103_459, 52645)
			// Standard Error: 4_645
			.saturating_add(Weight::from_parts(1_172_720, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `515`
		//  Estimated: `3530`
		// Minimum execution time: 33_941 nanoseconds.
		Weight::from_parts(35_252_000, 3530)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:1 w:1)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `532`
		//  Estimated: `3530`
		// Minimum execution time: 33_259 nanoseconds.
		Weight::from_parts(34_558_000, 3530)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages OutboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages OutboundLanes (max_values: Some(1), max_size: Some(44), added:
	/// 539, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRelayers RelayerRewards (r:2 w:2)
	///
	/// Proof: BridgeRelayers RelayerRewards (max_values: None, max_size: Some(65), added: 2540,
	/// mode: MaxEncodedLen)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `532`
		//  Estimated: `6070`
		// Minimum execution time: 35_199 nanoseconds.
		Weight::from_parts(36_989_000, 6070)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeUnknownMessages PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeUnknownMessages PalletOperatingMode (max_values: Some(1), max_size: Some(2),
	/// added: 497, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeUnknownGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeUnknownMessages InboundLanes (r:1 w:1)
	///
	/// Proof: BridgeUnknownMessages InboundLanes (max_values: None, max_size: Some(49180), added:
	/// 51655, mode: MaxEncodedLen)
	///
	/// The range of component `n` is `[128, 2048]`.
	fn receive_single_message_n_bytes_proof_with_dispatch(n: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `52645`
		// Minimum execution time: 75_228 nanoseconds.
		Weight::from_parts(62_255_691, 52645)
			// Standard Error: 2_005
			.saturating_add(Weight::from_parts(353_141, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
