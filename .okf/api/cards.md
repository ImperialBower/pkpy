---
type: API Surface
title: Cards and card collections
description: The parsing and container types — Card, Cards, Deck, HoleCards, Board and the stored card maps.
tags: [api, cards, parsing]
timestamp: 2026-08-30T00:00:00Z
---

# Classes

| Class | Role |
|---|---|
| `Rank`, `Suit` | The two halves of a card. |
| `Card` | One playing card. Backed by a CKC `u32`, so it is always valid once built. |
| `Cards` | An ordered collection. Iterable from Python via a private `CardsIterator`. |
| `Ranks` | A rank-only collection. |
| `Deck` | A 52-card deck. |
| `Two`, `Twos` | Two-card array and a collection of them. `Two` normalizes high-to-low. |
| `HoleCards` | The dealt hands, one entry per player. |
| `Board` | The community cards. |
| `Bard` | Board-and-hand pairing helper. |
| `SevenFiveBCM`, `IndexCardMap` | Precomputed lookup maps. **These need the `store` feature** — see [upstream dependency](/decisions/pkcore-upgrades.md). |

# Module functions

Four constants are exposed as functions, not attributes:

| Function | Value |
|---|---:|
| `unique_5_card_hands()` | 2,598,960 |
| `distinct_5_card_hands()` | 7,462 |
| `unique_2_card_hands()` | 1,326 |
| `distinct_2_card_hands()` | 169 |

# Examples

```python
from pkcore import Card, HoleCards, Board

ace = Card.parse("As")
hc = HoleCards.parse("As Kh 8d Kc")     # two players, two cards each
board = Board.parse("Ac 8h 7h 9s")
```

`parse()` is the primary constructor throughout — see
[the binding layer](/architecture/binding-layer.md).

Unicode suits work too; the examples in `examples/` use them
(`"6♠ 6♥ 5♦ 5♣"`).

# Related

- [Evaluation](/api/evaluation.md) — what you do with a `Game` built from these.
