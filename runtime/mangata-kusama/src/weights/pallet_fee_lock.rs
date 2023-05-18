// This file is part of Mangata.

// Copyright (C) 2020-2022 Mangata Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_fee_lock
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama"), DB CACHE: 1024

// Executed Command:
// target/release/mangata-node
// benchmark
// pallet
// -l=info,xyk=error,collective-mangata=warn,bootstrap=warn
// --chain
// kusama
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template
// ./templates/module-weight-template.hbs
// --output
// ./benchmarks/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_fee_lock.
pub trait WeightInfo {
	fn update_fee_lock_metadata() -> Weight;
	fn unlock_fee() -> Weight;
}

/// Weights for pallet_fee_lock using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_fee_lock::WeightInfo for ModuleWeight<T> {
	// Storage: FeeLock FeeLockMetadata (r:1 w:1)
	fn update_fee_lock_metadata() -> Weight {
		(Weight::from_parts(36_270_000, 0))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: FeeLock AccountFeeLockData (r:1 w:1)
	// Storage: FeeLock FeeLockMetadata (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: FeeLock FeeLockMetadataQeueuePosition (r:1 w:1)
	// Storage: FeeLock UnlockQueue (r:1 w:1)
	fn unlock_fee() -> Weight {
		(Weight::from_parts(57_590_000, 0))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: FeeLock FeeLockMetadata (r:1 w:1)
	fn update_fee_lock_metadata() -> Weight {
		(Weight::from_parts(36_270_000, 0))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: FeeLock AccountFeeLockData (r:1 w:1)
	// Storage: FeeLock FeeLockMetadata (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: FeeLock FeeLockMetadataQeueuePosition (r:1 w:1)
	// Storage: FeeLock UnlockQueue (r:1 w:1)
	fn unlock_fee() -> Weight {
		(Weight::from_parts(57_590_000, 0))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
}
