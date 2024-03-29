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

//! Autogenerated weights for pallet_xyk
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-06, STEPS: `2`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-local"), DB CACHE: 1024

// Executed Command:
// /hdd/work/mangata-ws/mangata-node/scripts/..//target/release/mangata-node
// benchmark
// pallet
// --chain
// kusama-local
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_xyk
// --extrinsic
// *
// --steps
// 2
// --repeat
// 2
// --output
// ./benchmarks/pallet_xyk_weights.rs
// --template
// ./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_xyk.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn sell_asset() -> Weight;
	fn multiswap_sell_asset(x: u32, ) -> Weight;
	fn buy_asset() -> Weight;
	fn multiswap_buy_asset(x: u32, ) -> Weight;
	fn mint_liquidity() -> Weight;
	fn mint_liquidity_using_vesting_native_tokens() -> Weight;
	fn burn_liquidity() -> Weight;
	fn provide_liquidity_with_conversion() -> Weight;
	fn compound_rewards() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AssetRegistry Metadata (r:3 w:1)
	// Storage: Bootstrap ActivePair (r:1 w:0)
	// Storage: Xyk Pools (r:2 w:1)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens NextCurrencyId (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Xyk LiquidityAssets (r:0 w:1)
	// Storage: Xyk LiquidityPools (r:0 w:1)
	fn create_pool() -> Weight {
		(Weight::from_parts(180_230_000, 0))
			.saturating_add(RocksDbWeight::get().reads(14 as u64))
			.saturating_add(RocksDbWeight::get().writes(12 as u64))
	}
	// Storage: Maintenance MaintenanceStatus (r:1 w:0)
	// Storage: AssetRegistry Metadata (r:2 w:0)
	// Storage: Xyk Pools (r:3 w:1)
	// Storage: Tokens Accounts (r:6 w:6)
	// Storage: System Account (r:2 w:2)
	fn sell_asset() -> Weight {
		(Weight::from_parts(197_110_000, 0))
			.saturating_add(RocksDbWeight::get().reads(14 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
	// Storage: Maintenance MaintenanceStatus (r:1 w:0)
	// Storage: AssetRegistry Metadata (r:3 w:0)
	// Storage: Xyk Pools (r:6 w:4)
	// Storage: Tokens Accounts (r:12 w:12)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn multiswap_sell_asset(x: u32, ) -> Weight {
		(Weight::from_parts(517_520_000, 0))
			// Standard Error: 249_655
			.saturating_add((Weight::from_parts(197_021_587, 0)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((8 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
			.saturating_add(RocksDbWeight::get().writes((6 as u64).saturating_mul(x as u64)))
	}
	// Storage: Maintenance MaintenanceStatus (r:1 w:0)
	// Storage: AssetRegistry Metadata (r:2 w:0)
	// Storage: Xyk Pools (r:4 w:1)
	// Storage: Tokens Accounts (r:6 w:6)
	// Storage: System Account (r:2 w:2)
	fn buy_asset() -> Weight {
		(Weight::from_parts(205_329_000, 0))
			.saturating_add(RocksDbWeight::get().reads(15 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
	// Storage: Maintenance MaintenanceStatus (r:1 w:0)
	// Storage: AssetRegistry Metadata (r:3 w:0)
	// Storage: Xyk Pools (r:6 w:4)
	// Storage: Tokens Accounts (r:12 w:12)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn multiswap_buy_asset(x: u32, ) -> Weight {
		(Weight::from_parts(533_530_000, 0))
			// Standard Error: 254_339
			.saturating_add((Weight::from_parts(202_640_460, 0)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((8 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
			.saturating_add(RocksDbWeight::get().writes((6 as u64).saturating_mul(x as u64)))
	}
	// Storage: AssetRegistry Metadata (r:2 w:0)
	// Storage: Xyk LiquidityAssets (r:1 w:0)
	// Storage: Xyk Pools (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: Tokens NextCurrencyId (r:1 w:0)
	// Storage: ProofOfStake PromotedPoolRewards (r:1 w:0)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: ProofOfStake RewardsInfo (r:1 w:1)
	// Storage: ProofOfStake TotalActivatedLiquidity (r:1 w:1)
	fn mint_liquidity() -> Weight {
		(Weight::from_parts(206_570_000, 0))
			.saturating_add(RocksDbWeight::get().reads(15 as u64))
			.saturating_add(RocksDbWeight::get().writes(10 as u64))
	}
	// Storage: Xyk LiquidityAssets (r:1 w:0)
	// Storage: ProofOfStake PromotedPoolRewards (r:1 w:0)
	// Storage: Vesting Vesting (r:2 w:2)
	// Storage: Tokens Locks (r:2 w:2)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: Xyk Pools (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens NextCurrencyId (r:1 w:0)
	fn mint_liquidity_using_vesting_native_tokens() -> Weight {
		(Weight::from_parts(228_030_000, 0))
			.saturating_add(RocksDbWeight::get().reads(14 as u64))
			.saturating_add(RocksDbWeight::get().writes(11 as u64))
	}
	// Storage: AssetRegistry Metadata (r:2 w:0)
	// Storage: Xyk LiquidityAssets (r:1 w:2)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: Xyk Pools (r:1 w:2)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: ProofOfStake PromotedPoolRewards (r:1 w:0)
	// Storage: ProofOfStake RewardsInfo (r:1 w:1)
	// Storage: ProofOfStake TotalActivatedLiquidity (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Xyk LiquidityPools (r:0 w:1)
	fn burn_liquidity() -> Weight {
		(Weight::from_parts(193_430_000, 0))
			.saturating_add(RocksDbWeight::get().reads(14 as u64))
			.saturating_add(RocksDbWeight::get().writes(14 as u64))
	}
	// Storage: Xyk LiquidityPools (r:1 w:0)
	// Storage: AssetRegistry Metadata (r:2 w:0)
	// Storage: Xyk Pools (r:4 w:1)
	// Storage: Tokens Accounts (r:7 w:7)
	// Storage: Maintenance MaintenanceStatus (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Xyk LiquidityAssets (r:2 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens NextCurrencyId (r:1 w:0)
	// Storage: ProofOfStake PromotedPoolRewards (r:1 w:0)
	fn provide_liquidity_with_conversion() -> Weight {
		(Weight::from_parts(314_670_000, 0))
			.saturating_add(RocksDbWeight::get().reads(22 as u64))
			.saturating_add(RocksDbWeight::get().writes(11 as u64))
	}
	// Storage: Xyk LiquidityPools (r:1 w:0)
	// Storage: AssetRegistry Metadata (r:2 w:0)
	// Storage: ProofOfStake PromotedPoolRewards (r:1 w:0)
	// Storage: ProofOfStake RewardsInfo (r:1 w:1)
	// Storage: Tokens Accounts (r:8 w:8)
	// Storage: Xyk Pools (r:2 w:1)
	// Storage: Maintenance MaintenanceStatus (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens TotalIssuance (r:2 w:2)
	// Storage: Xyk LiquidityAssets (r:2 w:0)
	// Storage: Tokens NextCurrencyId (r:1 w:0)
	// Storage: MultiPurposeLiquidity ReserveStatus (r:1 w:1)
	// Storage: ProofOfStake TotalActivatedLiquidity (r:1 w:1)
	fn compound_rewards() -> Weight {
		(Weight::from_parts(436_760_000, 0))
			.saturating_add(RocksDbWeight::get().reads(25 as u64))
			.saturating_add(RocksDbWeight::get().writes(16 as u64))
	}
}
