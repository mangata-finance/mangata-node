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
//! DATE: 2023-11-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("mangata-kusama"), DB CACHE: 1024

// Executed Command:
// target/release/mangata-node
// benchmark
// pallet
// -l=info,runtime::collective=warn,xyk=warn
// --chain
// mangata-kusama
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
	// Storage: `Crowdloan::Initialized` (r:1 w:0)
	// Proof: `Crowdloan::Initialized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::InitializedRewardAmount` (r:1 w:0)
	// Proof: `Crowdloan::InitializedRewardAmount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanAllocation` (r:0 w:1)
	// Proof: `Crowdloan::CrowdloanAllocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_crowdloan_allocation() -> Weight {
		(Weight::from_parts(13_910_000, 0))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: `Crowdloan::Initialized` (r:1 w:0)
	// Proof: `Crowdloan::Initialized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::InitializedRewardAmount` (r:1 w:1)
	// Proof: `Crowdloan::InitializedRewardAmount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::TotalContributors` (r:1 w:1)
	// Proof: `Crowdloan::TotalContributors` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanAllocation` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanAllocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::ClaimedRelayChainIds` (r:100 w:100)
	// Proof: `Crowdloan::ClaimedRelayChainIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::UnassociatedContributions` (r:100 w:0)
	// Proof: `Crowdloan::UnassociatedContributions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:100 w:100)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn initialize_reward_vec(x: u32, ) -> Weight {
		(Weight::from_parts(123_974_567, 0))
			// Standard Error: 61_847
			.saturating_add((Weight::from_parts(25_859_849, 0)).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(x as u64)))
	}
	// Storage: `Crowdloan::Initialized` (r:1 w:1)
	// Proof: `Crowdloan::Initialized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::InitializedRewardAmount` (r:1 w:0)
	// Proof: `Crowdloan::InitializedRewardAmount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanAllocation` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanAllocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::TotalContributors` (r:1 w:0)
	// Proof: `Crowdloan::TotalContributors` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanPeriod` (r:0 w:1)
	// Proof: `Crowdloan::CrowdloanPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn complete_initialization() -> Weight {
		(Weight::from_parts(28_570_000, 0))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::Initialized` (r:1 w:0)
	// Proof: `Crowdloan::Initialized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:1 w:1)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::NextCurrencyId` (r:1 w:0)
	// Proof: `Tokens::NextCurrencyId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	// Storage: `Crowdloan::CrowdloanPeriod` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Vesting::Vesting` (r:1 w:1)
	// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1869), added: 4344, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		(Weight::from_parts(129_160_000, 0))
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:2 w:2)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_reward_address() -> Weight {
		(Weight::from_parts(35_100_000, 0))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::UnassociatedContributions` (r:1 w:1)
	// Proof: `Crowdloan::UnassociatedContributions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::ClaimedRelayChainIds` (r:1 w:1)
	// Proof: `Crowdloan::ClaimedRelayChainIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:1 w:1)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn associate_native_identity() -> Weight {
		(Weight::from_parts(118_910_000, 0))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:2 w:2)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn change_association_with_relay_keys(x: u32, ) -> Weight {
		(Weight::from_parts(50_224_766, 0))
			// Standard Error: 121_712
			.saturating_add((Weight::from_parts(65_897_329, 0)).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: `Crowdloan::Initialized` (r:1 w:0)
	// Proof: `Crowdloan::Initialized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::InitializedRewardAmount` (r:1 w:0)
	// Proof: `Crowdloan::InitializedRewardAmount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanAllocation` (r:0 w:1)
	// Proof: `Crowdloan::CrowdloanAllocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_crowdloan_allocation() -> Weight {
		(Weight::from_parts(13_910_000, 0))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: `Crowdloan::Initialized` (r:1 w:0)
	// Proof: `Crowdloan::Initialized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::InitializedRewardAmount` (r:1 w:1)
	// Proof: `Crowdloan::InitializedRewardAmount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::TotalContributors` (r:1 w:1)
	// Proof: `Crowdloan::TotalContributors` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanAllocation` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanAllocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::ClaimedRelayChainIds` (r:100 w:100)
	// Proof: `Crowdloan::ClaimedRelayChainIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::UnassociatedContributions` (r:100 w:0)
	// Proof: `Crowdloan::UnassociatedContributions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:100 w:100)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn initialize_reward_vec(x: u32, ) -> Weight {
		(Weight::from_parts(123_974_567, 0))
			// Standard Error: 61_847
			.saturating_add((Weight::from_parts(25_859_849, 0)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(x as u64)))
	}
	// Storage: `Crowdloan::Initialized` (r:1 w:1)
	// Proof: `Crowdloan::Initialized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::InitializedRewardAmount` (r:1 w:0)
	// Proof: `Crowdloan::InitializedRewardAmount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanAllocation` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanAllocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::TotalContributors` (r:1 w:0)
	// Proof: `Crowdloan::TotalContributors` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::CrowdloanPeriod` (r:0 w:1)
	// Proof: `Crowdloan::CrowdloanPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn complete_initialization() -> Weight {
		(Weight::from_parts(28_570_000, 0))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::Initialized` (r:1 w:0)
	// Proof: `Crowdloan::Initialized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:1 w:1)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::NextCurrencyId` (r:1 w:0)
	// Proof: `Tokens::NextCurrencyId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	// Storage: `Crowdloan::CrowdloanPeriod` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Vesting::Vesting` (r:1 w:1)
	// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1869), added: 4344, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1261), added: 3736, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		(Weight::from_parts(129_160_000, 0))
			.saturating_add(RocksDbWeight::get().reads(10 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:2 w:2)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_reward_address() -> Weight {
		(Weight::from_parts(35_100_000, 0))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::UnassociatedContributions` (r:1 w:1)
	// Proof: `Crowdloan::UnassociatedContributions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::ClaimedRelayChainIds` (r:1 w:1)
	// Proof: `Crowdloan::ClaimedRelayChainIds` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:1 w:1)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn associate_native_identity() -> Weight {
		(Weight::from_parts(118_910_000, 0))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: `Crowdloan::CrowdloanId` (r:1 w:0)
	// Proof: `Crowdloan::CrowdloanId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Crowdloan::AccountsPayable` (r:2 w:2)
	// Proof: `Crowdloan::AccountsPayable` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn change_association_with_relay_keys(x: u32, ) -> Weight {
		(Weight::from_parts(50_224_766, 0))
			// Standard Error: 121_712
			.saturating_add((Weight::from_parts(65_897_329, 0)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}
