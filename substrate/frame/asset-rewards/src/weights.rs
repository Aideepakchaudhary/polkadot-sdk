// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_asset_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-07-31, STEPS: `50`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/debug/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=2
// --pallet=pallet-asset-rewards
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/asset-rewards/src/._weights0.rs
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_asset_rewards`.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn stake() -> Weight;
	fn unstake() -> Weight;
	fn harvest_rewards() -> Weight;
	fn set_pool_reward_rate_per_block() -> Weight;
	fn set_pool_admin() -> Weight;
	fn set_pool_expiry_block() -> Weight;
	fn deposit_reward_tokens() -> Weight;
	fn cleanup_pool() -> Weight;
}

/// Weights for `pallet_asset_rewards` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Assets::Asset` (r:2 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::NextPoolId` (r:1 w:1)
	/// Proof: `AssetRewards::NextPoolId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::PoolCost` (r:0 w:1)
	/// Proof: `AssetRewards::PoolCost` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::Pools` (r:0 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495`
		//  Estimated: `6360`
		// Minimum execution time: 708_000_000 picoseconds.
		Weight::from_parts(779_000_000, 6360)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::PoolStakers` (r:1 w:1)
	/// Proof: `AssetRewards::PoolStakers` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:0)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::Freezes` (r:1 w:1)
	/// Proof: `AssetsFreezer::Freezes` (`max_values`: None, `max_size`: Some(105), added: 2580, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::FrozenBalances` (r:1 w:1)
	/// Proof: `AssetsFreezer::FrozenBalances` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	fn stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `852`
		//  Estimated: `3675`
		// Minimum execution time: 441_000_000 picoseconds.
		Weight::from_parts(460_000_000, 3675)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::PoolStakers` (r:1 w:1)
	/// Proof: `AssetRewards::PoolStakers` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::Freezes` (r:1 w:1)
	/// Proof: `AssetsFreezer::Freezes` (`max_values`: None, `max_size`: Some(105), added: 2580, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:0)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::FrozenBalances` (r:1 w:1)
	/// Proof: `AssetsFreezer::FrozenBalances` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	fn unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `904`
		//  Estimated: `3599`
		// Minimum execution time: 480_000_000 picoseconds.
		Weight::from_parts(488_000_000, 3599)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:0)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::PoolStakers` (r:1 w:1)
	/// Proof: `AssetRewards::PoolStakers` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn harvest_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `990`
		//  Estimated: `6208`
		// Minimum execution time: 659_000_000 picoseconds.
		Weight::from_parts(660_000_000, 6208)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn set_pool_reward_rate_per_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3584`
		// Minimum execution time: 121_000_000 picoseconds.
		Weight::from_parts(125_000_000, 3584)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn set_pool_admin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3584`
		// Minimum execution time: 118_000_000 picoseconds.
		Weight::from_parts(149_000_000, 3584)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn set_pool_expiry_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3584`
		// Minimum execution time: 124_000_000 picoseconds.
		Weight::from_parts(133_000_000, 3584)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:0)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn deposit_reward_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `809`
		//  Estimated: `6208`
		// Minimum execution time: 578_000_000 picoseconds.
		Weight::from_parts(578_000_000, 6208)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// TODO: replace with actual weight.
	fn cleanup_pool() -> Weight {
		Weight::MAX
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Assets::Asset` (r:2 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::NextPoolId` (r:1 w:1)
	/// Proof: `AssetRewards::NextPoolId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(211), added: 2686, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::PoolCost` (r:0 w:1)
	/// Proof: `AssetRewards::PoolCost` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::Pools` (r:0 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495`
		//  Estimated: `6360`
		// Minimum execution time: 708_000_000 picoseconds.
		Weight::from_parts(779_000_000, 6360)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::PoolStakers` (r:1 w:1)
	/// Proof: `AssetRewards::PoolStakers` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:0)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::Freezes` (r:1 w:1)
	/// Proof: `AssetsFreezer::Freezes` (`max_values`: None, `max_size`: Some(105), added: 2580, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::FrozenBalances` (r:1 w:1)
	/// Proof: `AssetsFreezer::FrozenBalances` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	fn stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `852`
		//  Estimated: `3675`
		// Minimum execution time: 441_000_000 picoseconds.
		Weight::from_parts(460_000_000, 3675)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::PoolStakers` (r:1 w:1)
	/// Proof: `AssetRewards::PoolStakers` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::Freezes` (r:1 w:1)
	/// Proof: `AssetsFreezer::Freezes` (`max_values`: None, `max_size`: Some(105), added: 2580, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:0)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `AssetsFreezer::FrozenBalances` (r:1 w:1)
	/// Proof: `AssetsFreezer::FrozenBalances` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	fn unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `904`
		//  Estimated: `3599`
		// Minimum execution time: 480_000_000 picoseconds.
		Weight::from_parts(488_000_000, 3599)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:0)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `AssetRewards::PoolStakers` (r:1 w:1)
	/// Proof: `AssetRewards::PoolStakers` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn harvest_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `990`
		//  Estimated: `6208`
		// Minimum execution time: 659_000_000 picoseconds.
		Weight::from_parts(660_000_000, 6208)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn set_pool_reward_rate_per_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3584`
		// Minimum execution time: 121_000_000 picoseconds.
		Weight::from_parts(125_000_000, 3584)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn set_pool_admin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3584`
		// Minimum execution time: 118_000_000 picoseconds.
		Weight::from_parts(149_000_000, 3584)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:1)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	fn set_pool_expiry_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316`
		//  Estimated: `3584`
		// Minimum execution time: 124_000_000 picoseconds.
		Weight::from_parts(133_000_000, 3584)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRewards::Pools` (r:1 w:0)
	/// Proof: `AssetRewards::Pools` (`max_values`: None, `max_size`: Some(119), added: 2594, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn deposit_reward_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `809`
		//  Estimated: `6208`
		// Minimum execution time: 578_000_000 picoseconds.
		Weight::from_parts(578_000_000, 6208)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// TODO: replace with actual weight.
	fn cleanup_pool() -> Weight {
		Weight::MAX
	}
}