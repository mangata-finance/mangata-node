// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-08-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/vesting/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_xyk.
pub trait WeightInfo {
	fn init_issuance_config() -> Weight;
	fn finalize_tge() -> Weight;
	fn execute_tge(x: u32) -> Weight;
}

/// Weights for pallet_vesting using the Substrate node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_issuance::WeightInfo for ModuleWeight<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn init_issuance_config() -> Weight {
		(50_642_000 as Weight)
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn finalize_tge() -> Weight {
		(50_830_000 as Weight)
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn execute_tge(l: u32) -> Weight {
		(52_151_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((130_000 as Weight).saturating_mul(l as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn init_issuance_config() -> Weight {
		(50_642_000 as Weight)
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn finalize_tge() -> Weight {
		(50_830_000 as Weight)
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn execute_tge(l: u32) -> Weight {
		(52_151_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((130_000 as Weight).saturating_mul(l as Weight))
	}
}