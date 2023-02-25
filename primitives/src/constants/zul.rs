// Copyright 2023 Forecasting Technologies LTD.
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

use sp_runtime::Perbill;

// Distribution

/// Total ZUL amount for community incentives.
pub const COMMUNITY_INCENTIVES: u128 = 2_000_000;

/// Total ZUL amount for collators.
pub const COLLATORS: u128 = 0;

/// Total ZUL amount for liquidity mining.
pub const LIQUIDITY_MINING: u128 = 0;

/// Total ZUL amount for parachain lease.
pub const PARACHAIN_LEASE: u128 = 40_000_000;

/// Total ZUL amount for public sale.
pub const PUBLIC_SALE: u128 = 8_000_000;

/// Total ZUL amount for seed sale.
pub const SEED_SALE: u128 = 7_000_000;

/// Total ZUL amount for strategic sale.
pub const STRATEGIC_SALE: u128 = 6_000_000;

/// Total ZUL amount for Team and advisors.
pub const TEAM_AND_ADVISORS: u128 = 15_000_000;

/// Total ZUL amount for Zeitgesit foundation.
pub const ZULU_FOUNDATION: u128 = 22_000_000;

/// Total ZUL amount at genesis.
pub const TOTAL_INITIAL_ZUL: u128 = COMMUNITY_INCENTIVES
    + PARACHAIN_LEASE
    + PUBLIC_SALE
    + SEED_SALE
    + STRATEGIC_SALE
    + TEAM_AND_ADVISORS
    + ZULU_FOUNDATION;

// Inflation

/// Perthousand liquidity mining inflation. 0%
pub const LIQUIDITY_MINING_PTD: Perbill = Perbill::from_perthousand(0);

/// Perthousand collator staking inflation. 5%
pub const STAKING_PTD: Perbill = Perbill::from_perthousand(50);
