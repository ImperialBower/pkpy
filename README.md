[![CI](https://github.com/ImperialBower/pkpy/actions/workflows/ci.yml/badge.svg)](https://github.com/ImperialBower/pkpy/actions/workflows/ci.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

# pkpy

Python bindings for [pkcore](https://github.com/folkengine/pkcore), a high-performance poker analysis library written in Rust.

## What This Project Does

pkpy lets Python developers use pkcore's poker engine — card parsing, hand evaluation, Texas Hold'em game simulation, outs calculation, and more — without writing any Rust. The Rust library runs natively and is called directly from Python with no subprocess overhead or serialization round-trips.

---

## Dependencies

- [pkcore](https://crates.io/crates/pkcore) — the underlying Rust poker analysis library
- [PyO3](https://pyo3.rs) — Rust/Python bindings framework
- [Maturin](https://maturin.rs) — build tool for PyO3 extension modules

See [docs/STACK.md](docs/STACK.md) for more details on the technology stack.

---

## Cactus Kev Binary Card Representation

pkcore represents each card as a single `u32` using a variation of [Cactus Kev's binary encoding](https://suffe.cool/poker/evaluator.html), designed for O(1) hand evaluation via lookup tables.

```
+--------+--------+--------+--------+
|mmmbbbbb|bbbbbbbb|SHDCrrrr|xxpppppp|
+--------+--------+--------+--------+
```

| Bits | Meaning |
|------|---------|
| `p` (6 bits) | Prime number for the rank (Deuce=2, Trey=3, ..., Ace=41) |
| `r` (4 bits) | Rank index (Deuce=0, Trey=1, ..., Ace=12) |
| `SHDC` (4 bits) | Suit flags — one bit per suit |
| `b` (13 bits) | One bit set per rank — used for flush/straight detection |
| `m` (3 bits) | Frequency flags (paired, tripped, quaded) — stripped during eval |

This encoding makes many operations branch-free bit manipulations. For example, detecting a flush is a single bitwise AND across five cards' suit bits.

## Hand Evaluation

pkcore uses a two-level lookup table strategy (the same approach as the original Cactus Kev evaluator):

1. **Flushes and straights** are detected via the rank-bit field (`b` bits). A 13-bit mask uniquely identifies every possible straight and flush pattern.
2. **All other hands** are identified by multiplying the five rank primes together. Since every rank maps to a distinct prime, the product uniquely identifies the rank multiset — pairs, trips, quads, and full houses all have unique products. The product indexes into a lookup table that returns the `HandRankValue`.

A lower `HandRankValue` is a stronger hand (1 = royal flush, 7462 = worst high card).

---

## Project Structure

```
pkpy/
├── Cargo.toml              # Rust crate manifest
├── pyproject.toml          # Python build config (maturin)
├── src/
│   └── lib.rs              # All PyO3 bindings
├── python/
│   └── pkpy/
│       └── __init__.py     # Python package — re-exports everything from the extension
└── tests/
    └── test_pkpy.py        # pytest test suite
```

The `python/pkpy/` directory is the Python package. The compiled Rust extension (`_pkpy.so`) is dropped into it by maturin. `__init__.py` re-exports everything so users write `from pkpy import Card` rather than `from pkpy._pkpy import Card`.

---

## API Reference

### `Card`

A single playing card. Internally a `u32` in Cactus Kev format.

```python
from pkpy import Card, Rank, Suit

# Parse from string — accepts "As", "A♠", "a♠", "AH", etc.
ace_spades = Card.parse("As")
king_hearts = Card.parse("K♥")

# Construct from rank and suit
card = Card.from_rank_suit(Rank.QUEEN, Suit.DIAMONDS)

# Inspect
card.rank()       # -> Rank
card.suit()       # -> Suit
card.is_dealt()   # -> bool (False for blank/sentinel cards)
card.as_u32()     # -> int (raw Cactus Kev encoding)

str(card)         # -> "Q♦"
card == Card.parse("Qd")  # -> True
```

### `Cards`

An ordered, unique collection of cards backed by an `IndexSet` (ordered hash set). Duplicate inserts are silently ignored.

```python
from pkpy import Cards

hand = Cards.parse("As Ks Qh")
deck = Cards.deck()           # full 52-card deck

len(hand)                     # -> 3
hand.contains(Card.parse("As")) # -> True
hand.remaining()              # -> Cards with 49 cards (deck minus hand)
hand.remaining_after(board)   # -> deck minus hand minus board

for card in hand:             # iterable
    print(card)

hand.to_list()                # -> list[Card]
hand.is_dealt()               # -> True if no blank cards
hand.are_unique()             # -> True if no duplicates
```

### `HoleCards`

A collection of two-card hands for one or more players. Cards are parsed in pairs: the first two belong to player 1, the next two to player 2, and so on.

```python
from pkpy import HoleCards

# Two players
hc = HoleCards.parse("As Kh 8d Kc")
len(hc)  # -> 2

# One player
hc = HoleCards.parse("As Kh")
len(hc)  # -> 1
```

### `Board`

The community cards (flop, turn, river).

```python
from pkpy import Board

board = Board.parse("Ac 8h 7h 9s")      # flop + turn
board = Board.parse("Ac 8h 7h 9s 5s")  # full board

board.turn_cards()  # -> Cards (flop + turn, 4 cards)
str(board)          # -> "FLOP: A♣ 8♥ 7♥, TURN: 9♠, RIVER: _"
```

### `Game`

Combines hole cards and a board. The main entry point for analysis.

```python
from pkpy import Game, HoleCards, Board, Outs

hc    = HoleCards.parse("As Kh 8d Kc")
board = Board.parse("Ac 8h 7h 9s")
game  = Game(hc, board)

case_evals = game.turn_case_evals()  # evaluates all possible river cards
```

### `CaseEvals`

The result of `game.turn_case_evals()`. Contains one evaluation per possible river card (typically 44–46 entries depending on how many cards are already accounted for).

```python
len(case_evals)  # -> number of possible river cards evaluated
```

### `Outs`

Cards that, if dealt on the river, cause a specific player to win. Built from `CaseEvals`.

```python
from pkpy import Outs

outs = Outs.from_case_evals(case_evals)

outs.len_for_player(1)   # -> int: number of winning river cards for player 1
outs.len_for_player(2)   # -> int: number of winning river cards for player 2
outs.get(1)              # -> Cards | None: the actual out cards for player 1
outs.longest_player()    # -> int: player id with the most outs
outs.is_longest(2)       # -> bool
outs.len_longest()       # -> int: how many outs the leading player has
```

Players are 1-indexed.

### `HandRank` and `HandRankClass`

`HandRank` holds the numeric strength of a five-card hand. Lower `value` = stronger hand.

`HandRankClass` is the detailed category (e.g., `RoyalFlush`, `FourAces`, `AcesOverKings`).

```python
from pkpy import HandRankClass

HandRankClass.ROYAL_FLUSH.is_straight_flush()  # -> True
str(HandRankClass.ROYAL_FLUSH)                 # -> "RoyalFlush"
```

`HandRank` is obtained from `Eval` objects, which come out of `CaseEvals`. Direct construction is not exposed since you'd normally get them via game evaluation.

### Constants

```python
from pkpy import (
    unique_5_card_hands,    # 2,598,960
    distinct_5_card_hands,  # 7,462
    unique_2_card_hands,    # 1,326
    distinct_2_card_hands,  # 169
)
```

---

## GTO Range Analysis

### `Combo`

An abstract hand combination defined by rank(s) and a suit qualifier.

```python
from pkpy import Combo

c = Combo.parse("AKs")
c.is_suited()           # -> True
c.is_pair()             # -> False
c.is_ace_x()            # -> True
c.total_pairs()         # -> 4  (four suited AK combos)
c.first                 # -> Rank.ACE
c.second                # -> Rank.KING
c.plus                  # -> False

Combo.parse("JJ+").plus         # -> True
Combo.parse("QQ").total_pairs() # -> 6  (six ways to make QQ)
Combo.parse("AKo").total_pairs()# -> 12 (twelve offsuit AK combos)
```

### `Combos`

A range of abstract hand combinations, parsed from standard poker range notation.

```python
from pkpy import Combos

r = Combos.parse("QQ+, AK")
len(r)          # -> 5  (QQ, KK, AA, AKs, AKo as abstract combos)

twos = r.explode()
len(twos)       # -> 34 (all concrete two-card hands)

# Predefined ranges (returned as strings, pass to Combos.parse)
Combos.PERCENT_2_5   # "QQ+, AK"       — top ~2.5% of hands
Combos.PERCENT_5     # "TT+, AQ+"      — top ~5%
Combos.PERCENT_10    # "44+, AJ+, ..."  — top ~10%
Combos.PERCENT_20    # top ~20%
Combos.PERCENT_33    # top ~33%

# Parse a predefined range
tight = Combos.parse(Combos.PERCENT_2_5)
```

### `Two`

A concrete two-card hand — the unit produced by combo explosion.

```python
from pkpy import Two

t = Two.parse("As Kh")
t.first()               # -> Card (A♠)
t.second()              # -> Card (K♥)
t.is_suited()           # -> False
t.is_pair()             # -> False
t.contains_rank(Rank.ACE)   # -> True
t.contains_suit(Suit.SPADES) # -> True
```

### `Twos`

The collection returned by `Combos.explode()`. Supports filtering.

```python
from pkpy import Combos

twos = Combos.parse("QQ+, AK").explode()

twos.filter_is_paired()       # -> Twos  (only pocket pairs)
twos.filter_is_not_paired()   # -> Twos  (only non-paired hands)
twos.filter_is_suited()       # -> Twos  (only suited hands)
twos.filter_is_not_suited()   # -> Twos  (only offsuit hands)
twos.filter_on_rank(Rank.ACE) # -> Twos  (hands containing an Ace)
twos.filter_on_card(Card.parse("As"))  # -> Twos (hands containing A♠)

twos.to_list()    # -> list[Two]
twos.contains(Two.parse("As Kh"))  # -> bool
```

### `Qualifier`

The suit qualifier for a combo: `SUITED`, `OFFSUIT`, or `ALL`.

```python
from pkpy import Combo, Qualifier

Combo.parse("AKs").qualifier == Qualifier.SUITED   # -> True
Combo.parse("AKo").qualifier == Qualifier.OFFSUIT  # -> True
Combo.parse("AK").qualifier  == Qualifier.ALL      # -> True
```

### GTO Example

```python
from pkpy import Combos, Rank

# Villain's opening range
villain_range = Combos.parse("66+,AJs+,KQs,AJo+,KQo")

# Expand to all concrete two-card hands
twos = villain_range.explode()
print(f"Total hands in range: {len(twos)}")

# How many are pocket pairs vs. unpaired?
pairs  = twos.filter_is_paired()
unpaired = twos.filter_is_not_paired()
print(f"Pairs: {len(pairs)}, Unpaired: {len(unpaired)}")

# Hands containing an Ace
ace_hands = twos.filter_on_rank(Rank.ACE)
print(f"Ace-x hands: {len(ace_hands)}")

# Suited vs. offsuit breakdowns
print(f"Suited: {len(twos.filter_is_suited())}")
print(f"Offsuit: {len(twos.filter_is_not_suited())}")
```

---

## Complete Example

```python
from pkpy import HoleCards, Board, Game, Outs

# Recreate the famous Negreanu vs Hansen hand:
# Daniel holds 6♠ 6♥, Gus holds 5♦ 5♣
# Flop: 9♣ 6♦ 5♥ — Daniel flops top set, Gus flops bottom set
# Turn: 5♠ — Gus rivers quads. What are the outs for each player?

hc    = HoleCards.parse("6s 6h 5d 5c")
board = Board.parse("9c 6d 5h 5s")
game  = Game(hc, board)

outs = Outs.from_case_evals(game.turn_case_evals())

print(f"Player 1 (Daniel, 6♠6♥) outs: {outs.len_for_player(1)}")
print(f"Player 2 (Gus,    5♦5♣) outs: {outs.len_for_player(2)}")
print(f"Leading player: {outs.longest_player()}")
```

---

## Development Setup

**Prerequisites:** Rust toolchain (`rustup`), Python 3.8+

```bash
# Clone and enter the project
git clone <repo-url> pkpy
cd pkpy

# Create a virtual environment
python3 -m venv .venv
source .venv/bin/activate       # Windows: .venv\Scripts\activate

# Install build and test tools
pip install maturin pytest

# Compile the Rust extension and install it into the venv
python3 -m maturin develop

# Run tests
pytest
```

After changing `src/lib.rs`, re-run `python3 -m maturin develop` to recompile. Only the Rust source is recompiled on subsequent runs — Cargo's incremental compilation keeps this fast.

### Building a Release Wheel

```bash
python3 -m maturin build --release
# Wheel lands in target/wheels/pkpy-*.whl
pip install target/wheels/pkpy-*.whl
```

For distribution, maturin can also publish directly to PyPI:

```bash
python3 -m maturin publish
```

---

## Relationship to pkcore

This project wraps pkcore as a versioned crates.io dependency. The wrapper intentionally exposes a
subset of pkcore's API — the analysis-focused surface that's most useful from Python. Lower-level
types (binary card maps, SQLite storage, Pluribus log parsing) are not yet exposed.

---

## License

GPL-3.0-or-later, matching pkcore.
