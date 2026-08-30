---
type: API Surface
title: GTO, ranges and solving
description: Combo and range types, pot odds and EV, the Solver, and the Kuhn poker CFR toy.
tags: [api, gto, solver, cfr]
timestamp: 2026-08-30T00:00:00Z
---

# Ranges and combos

| Class | Role |
|---|---|
| `Combo`, `Combos` | A starting-hand combination and a set of them. |
| `Qualifier` | The suitedness/pairing qualifier on a combo. |
| `ComboPairs` | Paired combos for matchup work. |
| `Versus` | A hand against a range. |
| `RangeEquity` | Aggregate equity across a range, preflop through turn. |

# Decision maths

| Class | Role |
|---|---|
| `PotOdds` | Pot odds and breakeven equity for one decision point. `is_profitable(equity)` answers the call/fold question. |
| `Ev` | Expected value. |

# The solver

| Class | Role |
|---|---|
| `SolverConfig` | Solver parameters. |
| `BetSize`, `BetSizings` | The action abstraction. |
| `Solver` | Runs the solve. |
| `SolverResult` | The output. |
| `ActionFrequencies` | Per-action mixed-strategy frequencies. |

# Kuhn poker

A complete seven-class CFR sandbox: `KuhnCard`, `KuhnAction`, `KuhnHistory`,
`KuhnInfoSet`, `KuhnState`, `KuhnStrategy`, `KuhnCfr`.

Kuhn poker is a three-card toy game with a known analytic solution, so it is the
standard correctness check for a CFR implementation. It is here to validate and
to teach, not to play.

# Examples

`examples/gto.py` runs a range analysis end to end:

```bash
make gto      # KK vs 66+,AJs+,KQs,AJo+,KQo
```

# Related

- [Evaluation](/api/evaluation.md)
- [Table and session](/api/table-session.md)
