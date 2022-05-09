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

//! Autogenerated weights for pallet_crowdloan_rewards
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// /home/ubuntu/mangata-node/scripts/..//target/release/mangata-node
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_crowdloan_rewards
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./benchmarks/pallet_crowdloan_rewards_weights.rs
// --template
// ./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_crowdloan_rewards.
pub trait WeightInfo {
	fn set_crowdloan_allocation() -> Weight;
	fn initialize_reward_vec(x: u32, ) -> Weight;
	fn complete_initialization() -> Weight;
	fn claim() -> Weight;
	fn update_reward_address() -> Weight;
	fn associate_native_identity() -> Weight;
	fn change_association_with_relay_keys(x: u32, ) -> Weight;
}

/// Weights for pallet_crowdloan_rewards using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_crowdloan_rewards::WeightInfo for ModuleWeight<T> {
	// Storage: Crowdloan CrowdloanAllocation (r:0 w:1)
	fn set_crowdloan_allocation() -> Weight {
		(1_867_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Crowdloan Initialized (r:1 w:0)
	// Storage: Crowdloan InitializedRewardAmount (r:1 w:1)
	// Storage: Crowdloan TotalContributors (r:1 w:1)
	// Storage: Crowdloan CrowdloanAllocation (r:1 w:0)
	// Storage: Crowdloan ClaimedRelayChainIds (r:1 w:1)
	// Storage: Crowdloan UnassociatedContributions (r:1 w:0)
	// Storage: Tokens NextCurrencyId (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Crowdloan AccountsPayable (r:1 w:1)
	fn initialize_reward_vec(x: u32, ) -> Weight {
		(251_824_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((52_764_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(x as Weight)))
	}
	// Storage: Crowdloan Initialized (r:1 w:1)
	// Storage: Crowdloan InitRelayBlock (r:1 w:0)
	// Storage: Crowdloan InitializedRewardAmount (r:1 w:0)
	// Storage: Crowdloan CrowdloanAllocation (r:1 w:0)
	// Storage: Crowdloan TotalContributors (r:1 w:0)
	// Storage: Crowdloan EndRelayBlock (r:0 w:1)
	fn complete_initialization() -> Weight {
		(16_764_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Crowdloan Initialized (r:1 w:0)
	// Storage: Crowdloan AccountsPayable (r:1 w:1)
	// Storage: Crowdloan InitRelayBlock (r:1 w:0)
	// Storage: Crowdloan EndRelayBlock (r:1 w:0)
	// Storage: Tokens NextCurrencyId (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn claim() -> Weight {
		(47_962_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Crowdloan AccountsPayable (r:2 w:2)
	fn update_reward_address() -> Weight {
		(26_896_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Crowdloan UnassociatedContributions (r:1 w:1)
	// Storage: Crowdloan ClaimedRelayChainIds (r:1 w:1)
	// Storage: Crowdloan AccountsPayable (r:1 w:1)
	// Storage: Tokens NextCurrencyId (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn associate_native_identity() -> Weight {
		(109_212_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Crowdloan AccountsPayable (r:2 w:2)
	fn change_association_with_relay_keys(x: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 7_000
			.saturating_add((51_117_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn set_crowdloan_allocation() -> Weight {
		(1_867_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn initialize_reward_vec(x: u32, ) -> Weight {
		(251_824_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((52_764_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((5 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((4 as Weight).saturating_mul(x as Weight)))
	}
	fn complete_initialization() -> Weight {
		(16_764_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn claim() -> Weight {
		(47_962_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn update_reward_address() -> Weight {
		(26_896_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn associate_native_identity() -> Weight {
		(109_212_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	fn change_association_with_relay_keys(x: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 7_000
			.saturating_add((51_117_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}