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

//! Autogenerated weights for pallet_vesting_mangata
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/mangata-node
// benchmark
// pallet
// -l=info,xyk=error,collective-mangata=warn,bootstrap=warn
// --chain
// dev
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

/// Weight functions needed for pallet_vesting_mangata.
pub trait WeightInfo {
	fn vest_locked(l: u32, s: u32, ) -> Weight;
	fn vest_unlocked(l: u32, s: u32, ) -> Weight;
	fn vest_other_locked(l: u32, s: u32, ) -> Weight;
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight;
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight;
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight;
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight;
}

/// Weights for pallet_vesting_mangata using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting_mangata::WeightInfo for ModuleWeight<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn vest_locked(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(59_991_857))
			// Standard Error: 6_228
			.saturating_add((Weight::from_ref_time(204_519)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn vest_unlocked(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(68_770_488))
			// Standard Error: 3_643
			.saturating_add((Weight::from_ref_time(118_658)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn vest_other_locked(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(58_757_859))
			// Standard Error: 6_085
			.saturating_add((Weight::from_ref_time(198_617)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_unlocked(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(66_969_543))
			// Standard Error: 3_743
			.saturating_add((Weight::from_ref_time(118_563)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	// Storage: Tokens Locks (r:1 w:1)
	fn force_vested_transfer(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(85_464_533))
			// Standard Error: 5_912
			.saturating_add((Weight::from_ref_time(230_641)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn not_unlocking_merge_schedules(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(59_642_478))
			// Standard Error: 6_355
			.saturating_add((Weight::from_ref_time(205_531)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn unlocking_merge_schedules(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(59_756_037))
			// Standard Error: 6_206
			.saturating_add((Weight::from_ref_time(199_543)).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn vest_locked(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(59_991_857))
			// Standard Error: 6_228
			.saturating_add((Weight::from_ref_time(204_519)).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn vest_unlocked(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(68_770_488))
			// Standard Error: 3_643
			.saturating_add((Weight::from_ref_time(118_658)).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn vest_other_locked(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(58_757_859))
			// Standard Error: 6_085
			.saturating_add((Weight::from_ref_time(198_617)).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_unlocked(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(66_969_543))
			// Standard Error: 3_743
			.saturating_add((Weight::from_ref_time(118_563)).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	// Storage: Tokens Locks (r:1 w:1)
	fn force_vested_transfer(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(85_464_533))
			// Standard Error: 5_912
			.saturating_add((Weight::from_ref_time(230_641)).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn not_unlocking_merge_schedules(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(59_642_478))
			// Standard Error: 6_355
			.saturating_add((Weight::from_ref_time(205_531)).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Tokens Locks (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	fn unlocking_merge_schedules(_l: u32, s: u32, ) -> Weight {
		(Weight::from_ref_time(59_756_037))
			// Standard Error: 6_206
			.saturating_add((Weight::from_ref_time(199_543)).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}