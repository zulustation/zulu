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

//! Autogenerated weights for pallet_collective
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
// --pallet=pallet_collective
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

/// Weight functions for pallet_collective (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::weights::WeightInfo for WeightInfo<T> {
    // Storage: AdvisoryCommittee Members (r:1 w:1)
    // Storage: AdvisoryCommittee Proposals (r:1 w:0)
    // Storage: AdvisoryCommittee Voting (r:255 w:255)
    // Storage: AdvisoryCommittee Prime (r:0 w:1)
    fn set_members(m: u32, n: u32, p: u32) -> Weight {
        Weight::from_ref_time(0)
            // Standard Error: 195_000
            .saturating_add(Weight::from_ref_time(41_182_000).saturating_mul(m.into()))
            // Standard Error: 195_000
            .saturating_add(Weight::from_ref_time(210_000).saturating_mul(n.into()))
            // Standard Error: 76_000
            .saturating_add(Weight::from_ref_time(21_533_000).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
            .saturating_add(T::DbWeight::get().writes(2))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
    }
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    fn execute(b: u32, m: u32) -> Weight {
        Weight::from_ref_time(38_237_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000).saturating_mul(b.into()))
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(40_000).saturating_mul(m.into()))
            .saturating_add(T::DbWeight::get().reads(1))
    }
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee ProposalOf (r:1 w:0)
    fn propose_execute(b: u32, m: u32) -> Weight {
        Weight::from_ref_time(44_582_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000).saturating_mul(b.into()))
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(14_000).saturating_mul(m.into()))
            .saturating_add(T::DbWeight::get().reads(2))
    }
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee ProposalOf (r:1 w:1)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    // Storage: AdvisoryCommittee ProposalCount (r:1 w:1)
    // Storage: AdvisoryCommittee Voting (r:0 w:1)
    fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
        Weight::from_ref_time(37_436_000)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(20_000).saturating_mul(b.into()))
            // Standard Error: 11_000
            .saturating_add(Weight::from_ref_time(39_000).saturating_mul(m.into()))
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(279_000).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(4))
    }
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    fn vote(m: u32) -> Weight {
        Weight::from_ref_time(75_377_000)
            // Standard Error: 18_000
            .saturating_add(Weight::from_ref_time(454_000).saturating_mul(m.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    // Storage: AdvisoryCommittee ProposalOf (r:0 w:1)
    fn close_early_disapproved(_m: u32, p: u32) -> Weight {
        Weight::from_ref_time(125_780_000)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(208_000).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee ProposalOf (r:1 w:1)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
        Weight::from_ref_time(50_328_000)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(14_000).saturating_mul(b.into()))
            // Standard Error: 14_000
            .saturating_add(Weight::from_ref_time(134_000).saturating_mul(m.into()))
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(320_000).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee Prime (r:1 w:0)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    // Storage: AdvisoryCommittee ProposalOf (r:0 w:1)
    fn close_disapproved(_m: u32, p: u32) -> Weight {
        Weight::from_ref_time(77_551_000)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(224_000).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    // Storage: AdvisoryCommittee Voting (r:1 w:1)
    // Storage: AdvisoryCommittee Members (r:1 w:0)
    // Storage: AdvisoryCommittee Prime (r:1 w:0)
    // Storage: AdvisoryCommittee ProposalOf (r:1 w:1)
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    fn close_approved(b: u32, m: u32, p: u32) -> Weight {
        Weight::from_ref_time(23_404_000)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(30_000).saturating_mul(b.into()))
            // Standard Error: 14_000
            .saturating_add(Weight::from_ref_time(288_000).saturating_mul(m.into()))
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(360_000).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    // Storage: AdvisoryCommittee Proposals (r:1 w:1)
    // Storage: AdvisoryCommittee Voting (r:0 w:1)
    // Storage: AdvisoryCommittee ProposalOf (r:0 w:1)
    fn disapprove_proposal(p: u32) -> Weight {
        Weight::from_ref_time(48_539_000)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(202_000).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(3))
    }
}
