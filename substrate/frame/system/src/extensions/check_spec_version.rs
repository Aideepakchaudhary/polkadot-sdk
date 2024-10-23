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

use crate::{Config, Pallet};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
	impl_tx_ext_default, traits::TransactionExtension,
	transaction_validity::TransactionValidityError,
};

/// Ensure the runtime version registered in the transaction is the same as at present.
///
/// # Transaction Validity
///
/// The transaction with incorrect `spec_version` are considered invalid. The validity
/// is not affected in any other way.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckSpecVersion<T: Config + Send + Sync>(core::marker::PhantomData<T>);

impl<T: Config + Send + Sync> core::fmt::Debug for CheckSpecVersion<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "CheckSpecVersion")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
		Ok(())
	}
}

impl<T: Config + Send + Sync> CheckSpecVersion<T> {
	/// Create new `TransactionExtension` to check runtime version.
	pub fn new() -> Self {
		Self(core::marker::PhantomData)
	}
}

impl<T: Config + Send + Sync> TransactionExtension<<T as Config>::RuntimeCall>
	for CheckSpecVersion<T>
{
	const IDENTIFIER: &'static str = "CheckSpecVersion";
	type Implicit = u32;
	fn implicit(&self) -> Result<Self::Implicit, TransactionValidityError> {
		Ok(<Pallet<T>>::runtime_version().spec_version)
	}
	type Val = ();
	type Pre = ();
	fn weight(&self, _: &<T as Config>::RuntimeCall) -> sp_weights::Weight {
		<T::ExtensionsWeightInfo as super::WeightInfo>::check_spec_version()
	}
	impl_tx_ext_default!(<T as Config>::RuntimeCall; validate prepare);
}
impl<T: Config + Send + Sync, Context> TransactionExtension<<T as Config>::RuntimeCall, Context>
	for CheckSpecVersion<T>
{
	type Val = ();
	type Pre = ();
	impl_tx_ext_default!(<T as Config>::RuntimeCall; Context; validate prepare);
}
