// Copyright 2022-2023 Forecasting Technologies LTD.
// Copyright 2021-2022 Zulu PM LLC.
//
// This file is part of Zulu.
//
// Zulu is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// Zulu is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Zulu. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for zrml_swaps
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-20, STEPS: `10`, REPEAT: 1000, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zulu
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=1000
// --pallet=zrml_swaps
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/weight_template.hbs
// --output=./zrml/swaps/src/weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{traits::Get, weights::Weight};

///  Trait containing the required functions for weight retrival within
/// zrml_swaps (automatically generated)
pub trait WeightInfoZulu {
    fn admin_clean_up_pool_cpmm_categorical(a: u32) -> Weight;
    fn admin_clean_up_pool_cpmm_scalar() -> Weight;
    fn apply_to_cached_pools_execute_arbitrage(a: u32) -> Weight;
    fn apply_to_cached_pools_noop(a: u32) -> Weight;
    fn destroy_pool_in_subsidy_phase(a: u32) -> Weight;
    fn distribute_pool_share_rewards(a: u32, b: u32) -> Weight;
    fn end_subsidy_phase(a: u32, b: u32) -> Weight;
    fn execute_arbitrage_buy_burn(a: u32) -> Weight;
    fn execute_arbitrage_mint_sell(a: u32) -> Weight;
    fn execute_arbitrage_skipped(a: u32) -> Weight;
    fn pool_exit(a: u32) -> Weight;
    fn pool_exit_subsidy() -> Weight;
    fn pool_exit_with_exact_asset_amount() -> Weight;
    fn pool_exit_with_exact_pool_amount() -> Weight;
    fn pool_join(a: u32) -> Weight;
    fn pool_join_subsidy() -> Weight;
    fn pool_join_with_exact_asset_amount() -> Weight;
    fn pool_join_with_exact_pool_amount() -> Weight;
    fn clean_up_pool_categorical_without_reward_distribution(a: u32) -> Weight;
    fn swap_exact_amount_in_cpmm() -> Weight;
    fn swap_exact_amount_in_rikiddo(a: u32) -> Weight;
    fn swap_exact_amount_out_cpmm() -> Weight;
    fn swap_exact_amount_out_rikiddo(a: u32) -> Weight;
    fn open_pool(a: u32) -> Weight;
    fn close_pool(a: u32) -> Weight;
    fn destroy_pool(a: u32) -> Weight;
}

/// Weight functions for zrml_swaps (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfoZulu for WeightInfo<T> {
    // Storage: MarketCommons Markets (r:1 w:0)
    // Storage: MarketCommons MarketPool (r:1 w:0)
    // Storage: Swaps Pools (r:1 w:1)
    fn admin_clean_up_pool_cpmm_categorical(a: u32) -> Weight {
        Weight::from_ref_time(58_086_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(583_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: MarketCommons Markets (r:1 w:0)
    // Storage: MarketCommons MarketPool (r:1 w:0)
    // Storage: Swaps Pools (r:1 w:1)
    fn admin_clean_up_pool_cpmm_scalar() -> Weight {
        Weight::from_ref_time(58_390_000)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: Swaps PoolsCachedForArbitrage (r:8 w:7)
    // Storage: Swaps Pools (r:7 w:0)
    // Storage: Tokens Accounts (r:462 w:462)
    // Storage: System Account (r:7 w:0)
    // Storage: Tokens TotalIssuance (r:64 w:64)
    fn apply_to_cached_pools_execute_arbitrage(a: u32) -> Weight {
        Weight::from_ref_time(0)
            // Standard Error: 803_000
            .saturating_add(Weight::from_ref_time(2_356_167_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(43))
            .saturating_add(T::DbWeight::get().reads((70_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes(42))
            .saturating_add(T::DbWeight::get().writes((67_u64).saturating_mul(a.into())))
    }
    // Storage: Swaps PoolsCachedForArbitrage (r:8 w:7)
    fn apply_to_cached_pools_noop(a: u32) -> Weight {
        Weight::from_ref_time(6_148_000)
            // Standard Error: 13_000
            .saturating_add(Weight::from_ref_time(8_898_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:1 w:0)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:1)
    // Storage: Tokens Accounts (r:1 w:1)
    fn destroy_pool_in_subsidy_phase(a: u32) -> Weight {
        Weight::from_ref_time(47_422_000)
            // Standard Error: 40_000
            .saturating_add(Weight::from_ref_time(21_468_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
    }
    // Storage: Tokens TotalIssuance (r:2 w:1)
    // Storage: Tokens Accounts (r:46 w:21)
    // Storage: System Account (r:11 w:10)
    fn distribute_pool_share_rewards(a: u32, b: u32) -> Weight {
        Weight::from_ref_time(55_071_000)
            // Standard Error: 180_000
            .saturating_add(Weight::from_ref_time(28_979_000).saturating_mul(a.into()))
            // Standard Error: 180_000
            .saturating_add(Weight::from_ref_time(40_233_000).saturating_mul(b.into()))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(b.into())))
            .saturating_add(T::DbWeight::get().writes(1))
            .saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(b.into())))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:11 w:10)
    // Storage: Tokens Accounts (r:22 w:22)
    // Storage: System Account (r:11 w:11)
    // Storage: Tokens TotalIssuance (r:2 w:2)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:0)
    fn end_subsidy_phase(a: u32, b: u32) -> Weight {
        Weight::from_ref_time(0)
            // Standard Error: 168_000
            .saturating_add(Weight::from_ref_time(26_503_000).saturating_mul(a.into()))
            // Standard Error: 1_034_000
            .saturating_add(Weight::from_ref_time(126_477_000).saturating_mul(b.into()))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().reads((9_u64).saturating_mul(b.into())))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes((9_u64).saturating_mul(b.into())))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: System Account (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    fn execute_arbitrage_buy_burn(a: u32) -> Weight {
        Weight::from_ref_time(61_994_000)
            // Standard Error: 34_000
            .saturating_add(Weight::from_ref_time(36_370_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: System Account (r:2 w:1)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    fn execute_arbitrage_mint_sell(a: u32) -> Weight {
        Weight::from_ref_time(80_675_000)
            // Standard Error: 31_000
            .saturating_add(Weight::from_ref_time(33_433_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes(1))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:2 w:0)
    fn execute_arbitrage_skipped(a: u32) -> Weight {
        Weight::from_ref_time(29_826_000)
            // Standard Error: 13_000
            .saturating_add(Weight::from_ref_time(5_199_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:5 w:5)
    // Storage: System Account (r:1 w:0)
    fn pool_exit(a: u32) -> Weight {
        Weight::from_ref_time(64_421_000)
            // Standard Error: 34_000
            .saturating_add(Weight::from_ref_time(27_430_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:1 w:1)
    // Storage: Tokens Accounts (r:1 w:1)
    fn pool_exit_subsidy() -> Weight {
        Weight::from_ref_time(77_761_000)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_exit_with_exact_asset_amount() -> Weight {
        Weight::from_ref_time(142_560_000)
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_exit_with_exact_pool_amount() -> Weight {
        Weight::from_ref_time(143_190_000)
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:5 w:5)
    fn pool_join(a: u32) -> Weight {
        Weight::from_ref_time(74_055_000)
            // Standard Error: 32_000
            .saturating_add(Weight::from_ref_time(26_488_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Tokens Accounts (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:1 w:1)
    fn pool_join_subsidy() -> Weight {
        Weight::from_ref_time(78_671_000)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_join_with_exact_asset_amount() -> Weight {
        Weight::from_ref_time(128_600_000)
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_join_with_exact_pool_amount() -> Weight {
        Weight::from_ref_time(132_720_000)
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: Swaps Pools (r:1 w:1)
    fn clean_up_pool_categorical_without_reward_distribution(a: u32) -> Weight {
        Weight::from_ref_time(16_673_000)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(325_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:4 w:4)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn swap_exact_amount_in_cpmm() -> Weight {
        Weight::from_ref_time(170_520_000)
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Tokens TotalIssuance (r:2 w:1)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    fn swap_exact_amount_in_rikiddo(a: u32) -> Weight {
        Weight::from_ref_time(159_946_000)
            // Standard Error: 28_000
            .saturating_add(Weight::from_ref_time(19_270_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:4 w:4)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn swap_exact_amount_out_cpmm() -> Weight {
        Weight::from_ref_time(169_581_000)
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:4 w:3)
    // Storage: Tokens TotalIssuance (r:2 w:1)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn swap_exact_amount_out_rikiddo(a: u32) -> Weight {
        Weight::from_ref_time(101_410_000)
            // Standard Error: 42_000
            .saturating_add(Weight::from_ref_time(35_542_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    // Storage: Swaps Pools (r:1 w:1)
    fn open_pool(a: u32) -> Weight {
        Weight::from_ref_time(28_613_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(559_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: Swaps Pools (r:1 w:1)
    fn close_pool(a: u32) -> Weight {
        Weight::from_ref_time(31_634_000)
            // Standard Error: 2_000
            .saturating_add(Weight::from_ref_time(366_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Tokens Accounts (r:2 w:2)
    // Storage: System Account (r:1 w:1)
    // Storage: Tokens TotalIssuance (r:2 w:2)
    fn destroy_pool(a: u32) -> Weight {
        Weight::from_ref_time(38_770_000)
            // Standard Error: 30_000
            .saturating_add(Weight::from_ref_time(26_154_000).saturating_mul(a.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(a.into())))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
    }
}
