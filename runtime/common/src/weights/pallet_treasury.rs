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

//! Autogenerated weights for pallet_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zulu
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_treasury
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/common/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for pallet_treasury (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::weights::WeightInfo for WeightInfo<T> {
    fn spend() -> Weight {
        Weight::from_ref_time(280_000)
    }
    // Storage: Treasury ProposalCount (r:1 w:1)
    // Storage: Treasury Proposals (r:0 w:1)
    fn propose_spend() -> Weight {
        Weight::from_ref_time(60_380_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: Treasury Proposals (r:1 w:1)
    fn reject_proposal() -> Weight {
        Weight::from_ref_time(72_761_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: Treasury Proposals (r:1 w:0)
    // Storage: Treasury Approvals (r:1 w:1)
    fn approve_proposal(p: u32) -> Weight {
        Weight::from_ref_time(27_423_000)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(174_000).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: Treasury Approvals (r:1 w:1)
    fn remove_approval() -> Weight {
        Weight::from_ref_time(19_090_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: System Account (r:1 w:1)
    // Storage: Treasury Approvals (r:1 w:1)
    // Storage: Bounties BountyApprovals (r:1 w:1)
    // Storage: Treasury Proposals (r:2 w:2)
    fn on_initialize_proposals(p: u32) -> Weight {
        Weight::from_ref_time(43_929_000)
            // Standard Error: 358_000
            .saturating_add(Weight::from_ref_time(45_522_000).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(p.into())))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(p.into())))
    }
}
