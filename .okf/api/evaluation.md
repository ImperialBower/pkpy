---
type: API Surface
title: Hand evaluation and outs
description: Game, the per-street eval types, HandRank, CaseEvals and Outs.
tags: [api, evaluation, outs]
timestamp: 2026-08-30T00:00:00Z
---

# Classes

| Class | Role |
|---|---|
| `Game` | A `HoleCards` + `Board` pairing. The usual entry point. |
| `Eval` | The result of ranking one hand. |
| `HandRank` | The numeric strength value. |
| `HandRankClass` | The named category (pair, flush, …). |
| `DealEval`, `FlopEval`, `TurnEval`, `RiverEval` | Per-street evaluation snapshots. |
| `CaseEvals` | Every remaining runout, evaluated. |
| `Outs` | Which cards win for which player, derived from `CaseEvals`. |
| `WinLoseDraw` | A win/lose/draw tally. |
| `HUPResult` | Heads-up precomputed result (needs the `store` feature). |
| `Percentage` | Percentage value wrapper. |

# Examples

```python
from pkcore import HoleCards, Board, Game, Outs

game = Game(HoleCards.parse("As Kh 8d Kc"), Board.parse("Ac 8h 7h 9s"))
outs = Outs.from_case_evals(game.turn_case_evals())
print(outs.len_for_player(1))    # 1-based!
```

> **Player indices are 1-based.** Player 1 is the first hand passed to
> `HoleCards`. See [the binding layer](/architecture/binding-layer.md).

`demo.py` at the repository root walks the full flow on a real hand.

# Related

- [Cards](/api/cards.md) — the inputs.
- [GTO and solving](/api/gto.md) — range-level analysis instead of hand-level.
