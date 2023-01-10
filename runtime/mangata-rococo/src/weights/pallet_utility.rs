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

//! Autogenerated weights for pallet_utility
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

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
	fn batch(c: u32, ) -> Weight;
	fn as_derivative() -> Weight;
	fn batch_all(c: u32, ) -> Weight;
	fn dispatch_as() -> Weight;
	fn force_batch(c: u32, ) -> Weight;
}

/// Weights for pallet_utility using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for ModuleWeight<T> {
	fn batch(c: u32, ) -> Weight {
		(Weight::from_ref_time(20_194_974))
			// Standard Error: 2_172
			.saturating_add((Weight::from_ref_time(6_810_959)).saturating_mul(c as u64))
	}
	fn as_derivative() -> Weight {
		(Weight::from_ref_time(10_880_000))
	}
	fn batch_all(c: u32, ) -> Weight {
		(Weight::from_ref_time(28_041_086))
			// Standard Error: 1_589
			.saturating_add((Weight::from_ref_time(7_087_481)).saturating_mul(c as u64))
	}
	fn dispatch_as() -> Weight {
		(Weight::from_ref_time(21_580_000))
	}
	fn force_batch(c: u32, ) -> Weight {
		(Weight::from_ref_time(24_615_257))
			// Standard Error: 1_425
			.saturating_add((Weight::from_ref_time(6_806_940)).saturating_mul(c as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn batch(c: u32, ) -> Weight {
		(Weight::from_ref_time(20_194_974))
			// Standard Error: 2_172
			.saturating_add((Weight::from_ref_time(6_810_959)).saturating_mul(c as u64))
	}
	fn as_derivative() -> Weight {
		(Weight::from_ref_time(10_880_000))
	}
	fn batch_all(c: u32, ) -> Weight {
		(Weight::from_ref_time(28_041_086))
			// Standard Error: 1_589
			.saturating_add((Weight::from_ref_time(7_087_481)).saturating_mul(c as u64))
	}
	fn dispatch_as() -> Weight {
		(Weight::from_ref_time(21_580_000))
	}
	fn force_batch(c: u32, ) -> Weight {
		(Weight::from_ref_time(24_615_257))
			// Standard Error: 1_425
			.saturating_add((Weight::from_ref_time(6_806_940)).saturating_mul(c as u64))
	}
}