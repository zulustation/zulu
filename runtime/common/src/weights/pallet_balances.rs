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

//! Autogenerated weights for pallet_balances
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
// --pallet=pallet_balances
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

/// Weight functions for pallet_balances (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::weights::WeightInfo for WeightInfo<T> {
    // Storage: System Account (r:1 w:1)
    fn transfer() -> Weight {
        Weight::from_ref_time(75_550_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: System Account (r:1 w:1)
    fn transfer_keep_alive() -> Weight {
        Weight::from_ref_time(61_350_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: System Account (r:1 w:1)
    fn set_balance_creating() -> Weight {
        Weight::from_ref_time(44_461_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: System Account (r:1 w:1)
    fn set_balance_killing() -> Weight {
        Weight::from_ref_time(47_560_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: System Account (r:2 w:2)
    fn force_transfer() -> Weight {
        Weight::from_ref_time(74_281_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: System Account (r:1 w:1)
    fn transfer_all() -> Weight {
        Weight::from_ref_time(65_200_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: System Account (r:1 w:1)
    fn force_unreserve() -> Weight {
        Weight::from_ref_time(37_050_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}
