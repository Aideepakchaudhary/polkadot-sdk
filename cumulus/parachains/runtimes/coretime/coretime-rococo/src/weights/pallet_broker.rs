// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_broker`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-dcu62vjg-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("coretime-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_broker
// --chain=coretime-rococo-dev
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/coretime/coretime-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_broker`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_broker::WeightInfo for WeightInfo<T> {
	/// Storage: `Broker::Configuration` (r:0 w:1)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	fn configure() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_717_000 picoseconds.
		Weight::from_parts(1_895_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10888`
		//  Estimated: `13506`
		// Minimum execution time: 21_505_000 picoseconds.
		Weight::from_parts(22_063_000, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	fn unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12090`
		//  Estimated: `13506`
		// Minimum execution time: 20_384_000 picoseconds.
		Weight::from_parts(21_196_000, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_lease() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `466`
		//  Estimated: `1951`
		// Minimum execution time: 10_253_000 picoseconds.
		Weight::from_parts(10_729_000, 0)
			.saturating_add(Weight::from_parts(0, 1951))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:0 w:1)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:60)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn start_sales(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12599`
		//  Estimated: `15065 + n * (1 ±0)`
		// Minimum execution time: 43_067_000 picoseconds.
		Weight::from_parts(126_742_591, 0)
			.saturating_add(Weight::from_parts(0, 15065))
			// Standard Error: 2_215
			.saturating_add(Weight::from_parts(19_257, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(59))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:0 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn purchase() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3593`
		// Minimum execution time: 32_384_000 picoseconds.
		Weight::from_parts(33_354_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AllowedRenewals` (r:1 w:2)
	/// Proof: `Broker::AllowedRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn renew() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `434`
		//  Estimated: `4698`
		// Minimum execution time: 53_960_000 picoseconds.
		Weight::from_parts(57_879_000, 0)
			.saturating_add(Weight::from_parts(0, 4698))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `3551`
		// Minimum execution time: 13_362_000 picoseconds.
		Weight::from_parts(13_915_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Regions` (r:1 w:2)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn partition() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `3551`
		// Minimum execution time: 14_920_000 picoseconds.
		Weight::from_parts(15_454_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::Regions` (r:1 w:3)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn interlace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `3551`
		// Minimum execution time: 16_313_000 picoseconds.
		Weight::from_parts(17_052_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn assign() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `937`
		//  Estimated: `4681`
		// Minimum execution time: 25_406_000 picoseconds.
		Weight::from_parts(26_831_000, 0)
			.saturating_add(Weight::from_parts(0, 4681))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolIo` (r:2 w:2)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:0 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1003`
		//  Estimated: `5996`
		// Minimum execution time: 31_507_000 picoseconds.
		Weight::from_parts(32_208_000, 0)
			.saturating_add(Weight::from_parts(0, 5996))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:3 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[1, 3]`.
	fn claim_revenue(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `652`
		//  Estimated: `6196 + m * (2520 ±0)`
		// Minimum execution time: 56_122_000 picoseconds.
		Weight::from_parts(56_577_577, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			// Standard Error: 43_888
			.saturating_add(Weight::from_parts(1_516_479, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(Weight::from_parts(0, 2520).saturating_mul(m.into()))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn purchase_credit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `322`
		//  Estimated: `3787`
		// Minimum execution time: 63_376_000 picoseconds.
		Weight::from_parts(64_977_000, 0)
			.saturating_add(Weight::from_parts(0, 3787))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn drop_region() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `466`
		//  Estimated: `3551`
		// Minimum execution time: 46_857_000 picoseconds.
		Weight::from_parts(49_768_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn drop_contribution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463`
		//  Estimated: `3533`
		// Minimum execution time: 91_252_000 picoseconds.
		Weight::from_parts(107_603_000, 0)
			.saturating_add(Weight::from_parts(0, 3533))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:1 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn drop_history() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `857`
		//  Estimated: `3593`
		// Minimum execution time: 98_685_000 picoseconds.
		Weight::from_parts(107_482_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AllowedRenewals` (r:1 w:1)
	/// Proof: `Broker::AllowedRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	fn drop_renewal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `957`
		//  Estimated: `4698`
		// Minimum execution time: 40_063_000 picoseconds.
		Weight::from_parts(43_346_000, 0)
			.saturating_add(Weight::from_parts(0, 4698))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 1000]`.
	fn request_core_count(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 20_362_000 picoseconds.
		Weight::from_parts(21_807_717, 0)
			.saturating_add(Weight::from_parts(0, 3539))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::CoreCountInbox` (r:1 w:1)
	/// Proof: `Broker::CoreCountInbox` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn process_core_count(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `266`
		//  Estimated: `1487`
		// Minimum execution time: 5_886_000 picoseconds.
		Weight::from_parts(6_362_864, 0)
			.saturating_add(Weight::from_parts(0, 1487))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: UNKNOWN KEY `0xf308d869daf021a7724e69c557dd8dbe` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xf308d869daf021a7724e69c557dd8dbe` (r:1 w:1)
	/// Storage: `Broker::InstaPoolHistory` (r:1 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn process_revenue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `447`
		//  Estimated: `6196`
		// Minimum execution time: 37_127_000 picoseconds.
		Weight::from_parts(37_843_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:60)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn rotate_sale(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12514`
		//  Estimated: `13506`
		// Minimum execution time: 94_308_000 picoseconds.
		Weight::from_parts(96_312_837, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			// Standard Error: 168
			.saturating_add(Weight::from_parts(573, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(65))
	}
	/// Storage: `Broker::InstaPoolIo` (r:1 w:0)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:0 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn process_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3493`
		// Minimum execution time: 5_843_000 picoseconds.
		Weight::from_parts(6_103_000, 0)
			.saturating_add(Weight::from_parts(0, 3493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workload` (r:1 w:1)
	/// Proof: `Broker::Workload` (`max_values`: None, `max_size`: Some(1212), added: 3687, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn process_core_schedule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1321`
		//  Estimated: `4786`
		// Minimum execution time: 33_622_000 picoseconds.
		Weight::from_parts(34_582_000, 0)
			.saturating_add(Weight::from_parts(0, 4786))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn request_revenue_info_at() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 16_170_000 picoseconds.
		Weight::from_parts(16_990_000, 0)
			.saturating_add(Weight::from_parts(0, 3539))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::CoreCountInbox` (r:0 w:1)
	/// Proof: `Broker::CoreCountInbox` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	fn notify_core_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_549_000 picoseconds.
		Weight::from_parts(1_734_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Status` (r:1 w:1)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::CoreCountInbox` (r:1 w:0)
	/// Proof: `Broker::CoreCountInbox` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xf308d869daf021a7724e69c557dd8dbe` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xf308d869daf021a7724e69c557dd8dbe` (r:1 w:1)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn do_tick_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `432`
		//  Estimated: `3897`
		// Minimum execution time: 12_408_000 picoseconds.
		Weight::from_parts(12_771_000, 0)
			.saturating_add(Weight::from_parts(0, 3897))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::SaleInfo` (r:1 w:0)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Broker::Workplan` (r:0 w:2)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn force_reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11363`
		//  Estimated: `14828`
		// Minimum execution time: 57_465_000 picoseconds.
		Weight::from_parts(59_013_000, 0)
			.saturating_add(Weight::from_parts(0, 14828))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(401), added: 896, mode: `MaxEncodedLen`)
	fn swap_leases() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `470`
		//  Estimated: `1886`
		// Minimum execution time: 6_086_000 picoseconds.
		Weight::from_parts(6_437_000, 0)
			.saturating_add(Weight::from_parts(0, 1886))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
