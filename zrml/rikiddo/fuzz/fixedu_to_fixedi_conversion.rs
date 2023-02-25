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

//! Fuzz test: Conversion FixedU -> FixedI
#![allow(
    // Mocks are only used for fuzzing and unit tests
    clippy::integer_arithmetic
)]
#![no_main]

use libfuzzer_sys::fuzz_target;
use substrate_fixed::{types::extra::U33, FixedI128};
use zrml_rikiddo::utils::convert_to_signed;

mod shared;
use shared::fixed_from_u128;

fuzz_target!(|fixed_num: u128| {
    let _: Result<FixedI128<U33>, &'static str> = convert_to_signed(fixed_from_u128(fixed_num));
});
