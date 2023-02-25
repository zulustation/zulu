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

//! Fuzz test: EmaMarketVolume is called during second state.
//! -> change state (update), update, get ema, clear
#![allow(
    // Mocks are only used for fuzzing and unit tests
    clippy::integer_arithmetic
)]
#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use substrate_fixed::{types::extra::U33, FixedU128};
use zrml_rikiddo::{
    traits::MarketAverage,
    types::{EmaMarketVolume, TimestampedVolume},
};

mod shared;
use shared::fixed_from_u128;

fuzz_target!(|data: Data| {
    let mut emv = data.ema_market_volume;
    let first_timestamped_volume =
        TimestampedVolume { timestamp: 0, volume: fixed_from_u128(data.first_update_volume) };
    let _ = emv.update_volume(&first_timestamped_volume);
    let _ = emv.update_volume(&data.second_update_volume);
});

#[derive(Debug, Arbitrary)]
struct Data {
    first_update_volume: u128,
    second_update_volume: TimestampedVolume<FixedU128<U33>>,
    ema_market_volume: EmaMarketVolume<FixedU128<U33>>,
}
