
//! Autogenerated weights for `pallet_stake_tracker`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-07-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `gpestanas-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// /Users/gpestana/cargo_target/release/substrate-node
// benchmark
// pallet
// --execution
// wasm
// --wasm-execution
// compiled
// --chain
// dev
// --pallet
// pallet-stake-tracker
// --extrinsic
// *
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_stake_tracker`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_stake_tracker::WeightInfo for WeightInfo<T> {
	/// Storage: `TargetList::ListNodes` (r:1 w:1)
	/// Proof: `TargetList::ListNodes` (`max_values`: None, `max_size`: Some(170), added: 2645, mode: `MaxEncodedLen`)
	/// Storage: `StakeTracker::UnsettledTargetScore` (r:1 w:1)
	/// Proof: `StakeTracker::UnsettledTargetScore` (`max_values`: None, `max_size`: Some(57), added: 2532, mode: `MaxEncodedLen`)
	fn settle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `606`
		//  Estimated: `3635`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3635))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
