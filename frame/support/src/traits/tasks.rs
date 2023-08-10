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

//! Contains the [`Task`] trait, which defines a general-purpose way for defining and executing
//! service work, and supporting types.

use codec::FullCodec;
use scale_info::TypeInfo;
use sp_core::blake2_128;
use sp_runtime::DispatchError;
use sp_std::{fmt::Debug, iter::Iterator};
use sp_weights::Weight;

/// A general-purpose trait which defines a type of service work (i.e., work to performed by an
/// off-chain worker) including methods for enumerating, validating, indexing, and running
/// tasks of this type.
pub trait Task: Sized + FullCodec + TypeInfo + Clone + Debug + PartialEq + Eq {
	type Enumeration: Iterator<Item = Self>;

	/// A unique value representing this `Task`. Analogous to `call_index`, but for tasks.
	const TASK_INDEX: u64;

	/// Inspects the pallet's state and enumerates tasks of this type.
	fn enumerate() -> Self::Enumeration;

	/// Checks if a particular instance of this `Task` variant is a valid piece of work.
	fn is_valid(&self) -> bool;

	/// Performs the work for this particular `Task` variant.
	fn run(&self) -> Result<(), DispatchError>;

	/// Returns the weight of executing this `Task`.
	fn weight(&self) -> Weight;

	fn task_index(&self) -> u64 {
		Self::TASK_INDEX
	}

	/// Returns a 64-bit hash code uniquely identifying this task and its inputs and associated
	/// data based on the full 128-bit Blake2 hash code. This is used in the `InvalidTask`
	/// event to differentiate between instances of the same task.
	fn hash_code(&self) -> u64 {
		let full_hash = blake2_128(&self.encode());
		u64::from_le_bytes([
			full_hash[0],
			full_hash[1],
			full_hash[2],
			full_hash[3],
			full_hash[4],
			full_hash[5],
			full_hash[6],
			full_hash[7],
		])
	}
}