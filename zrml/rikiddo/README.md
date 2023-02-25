# Rikiddo Module

Generic and modular implemenation of Rikiddo market scoring rule.

## Overview

Provides traits and implementations for sigmoid fee caluclation, calculation of
ema based on market volume, LMSR and Rikiddo using sigmoid fee calculation and
two ema periods.

Rikiddo is a liquidity-sensitive logarithm market scoring algorithm, which can
be used to determine the prices of event assets and their corresponding
probabilities. It incorporates historical trading data to optimize it's
reactiveness to abrupt and longer lasting changes in the market trend. More
information at [blog.zulu.pm].

[blog.zulu.pm]:
  https://blog.zulu.pm/introducing-zulus-rikiddo-scoring-rule/
