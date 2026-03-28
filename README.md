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

### `Rank`

Card rank enum. Values range from `BLANK` (0) through `DEUCE` (2) up to `ACE` (14).

```python
from pkpy import Rank

Rank.ACE.value()   # -> 14
Rank.KING.value()  # -> 13
Rank.DEUCE.value() # -> 2

str(Rank.ACE)  # -> "A"
Rank.ACE > Rank.KING  # -> True

# All variants:
# ACE, KING, QUEEN, JACK, TEN, NINE, EIGHT, SEVEN,
# SIX, FIVE, FOUR, TREY, DEUCE, BLANK
```

### `Suit`

Card suit enum.

```python
from pkpy import Suit

Suit.SPADES.value()   # -> 4
Suit.CLUBS.value()    # -> 1
Suit.HEARTS.symbol()  # -> "♥"
Suit.DIAMONDS.letter() # -> "D"

str(Suit.SPADES)  # -> "♠"

# All variants: SPADES, HEARTS, DIAMONDS, CLUBS, BLANK
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

## Design Notes

**Why not ctypes or cffi?** Those require a C-compatible ABI layer and manual memory management. PyO3 operates at the Python C API level and handles memory safety through Rust's ownership model. It also provides much richer type integration (Python exceptions, iterators, `__str__`, `__eq__`, etc.) with very little boilerplate.

**Why not pydantic-style dataclasses?** pkcore types carry invariants that are enforced by Rust's type system at construction time (e.g., a `Card` is always a valid CKC `u32`). Reimplementing those in Python would either duplicate the logic or lose the guarantees. Wrapping the Rust types directly means the invariants are never violated.

**String parsing as the primary constructor:** pkcore's Rust API uses `FromStr` extensively, and that maps naturally to static `parse()` class methods in Python. This keeps the Python API idiomatic while reusing the battle-tested Rust parsing logic.

**Player indices are 1-based:** This matches pkcore's convention in `Outs` and `CaseEvals`, where player 1 is the first hand passed to `HoleCards`.

---

## Relationship to pkcore

This project wraps pkcore as a versioned crates.io dependency. The wrapper intentionally exposes a subset of pkcore's API — the analysis-focused surface that's most useful from Python. Lower-level types (binary card maps, SQLite storage, GTO combo explosion, Pluribus log parsing) are not yet exposed.

---

## License

GPL-3.0-or-later, matching pkcore.
