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
card.rank()              # -> Rank
card.suit()              # -> Suit
card.is_dealt()          # -> bool (False for blank/sentinel cards)
card.as_u32()            # -> int (raw Cactus Kev encoding)
card.bit_string()        # -> str (binary representation of the encoding)
card.get_rank_prime()    # -> int (rank prime used in hand evaluation)
card.get_letter_index()  # -> str (letter-index form, e.g. "As")

str(card)                # -> "Q♦"
card == Card.parse("Qd") # -> True
```

### `Deck`

A standard 52-card deck. All methods are static — `Deck` is a namespace for deck-level operations.

```python
from pkpy import Deck

deck = Deck.poker_cards()           # -> Cards, ordered A♠ down to 2♣
shuffled = Deck.poker_cards_shuffled()  # -> Cards, randomly shuffled
Deck.get(0)                         # -> Card (A♠, the first card in deck order)
Deck.len()                          # -> 52
```

### `Cards`

An ordered, unique collection of cards backed by an `IndexSet` (ordered hash set). Duplicate inserts are silently ignored.

```python
from pkpy import Cards

hand = Cards.parse("As Ks Qh")
deck = Cards.deck()           # full 52-card deck in order

len(hand)                         # -> 3
hand.is_empty()                   # -> False
hand.contains(Card.parse("As"))   # -> True
hand.remaining()                  # -> Cards with 49 cards (deck minus hand)
hand.remaining_after(board)       # -> deck minus hand minus board
hand.is_dealt()                   # -> True if no blank cards
hand.are_unique()                 # -> True if no duplicates

for card in hand:                 # iterable
    print(card)

hand.to_list()                    # -> list[Card]
hand.get_index(0)                 # -> Card | None (card at position 0)

# Mutation
hand.insert(Card.parse("Jh"))     # -> bool (True if card was new)
hand.remove(Card.parse("As"))     # -> bool (True if card was present)
hand.append(Cards.parse("Tc 9c")) # merge another Cards in place
hand.shuffle_in_place()           # shuffle in place

# Non-mutating transformations
hand.shuffle()                    # -> Cards (shuffled copy)
hand.sort()                       # -> Cards (sorted highest rank first)
hand.minus(other)                 # -> Cards (this minus other)
hand.filter_by_suit(Suit.SPADES)  # -> Cards (only spades)
hand.combinations(2)              # -> list[Cards] (all 2-card combos)

# Drawing (mutates the source collection)
card = hand.draw_one()            # -> Card (removes and returns the top card)
drawn = hand.draw(3)              # -> Cards (removes and returns 3 cards)
rest = hand.draw_all()            # -> Cards (empties the collection)

# Deck-relative operations
hand.deck_minus()                 # -> Cards (52-card deck minus this collection)
hand.deck_primed()                # -> Cards (this collection first, then rest of deck)
```

### `HoleCards`

A collection of two-card hands for one or more players. Cards are parsed in pairs: the first two belong to player 1, the next two to player 2, and so on.

```python
from pkpy import HoleCards

# Two players
hc = HoleCards.parse("As Kh 8d Kc")
len(hc)        # -> 2
hc.is_empty()  # -> False
hc.get(0)      # -> Two | None (0-indexed)
hc.to_list()   # -> list[Two]

# Build programmatically
hc = HoleCards.parse("As Kh")
hc.push(Two.parse("Qd Jc"))
len(hc)  # -> 2
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

game.has_dealt_turn()          # -> bool (True if board has a turn card)
case_evals = game.turn_case_evals()          # evaluates all possible river cards
game.turn_eval_for_player(0)   # -> Eval for player at index 0 (raises on missing turn)
game.turn_remaining_board()    # -> Cards (deck cards not yet on the board or in hands)
game.flop_and_turn()           # -> Cards (the 4 board cards through the turn)

flop_eval = game.flop_eval()   # -> FlopEval | None
turn_eval = game.turn_eval()   # -> TurnEval | None

print(game.turn_nuts_display())  # best hands possible at the turn
print(game.river_display())      # final result with winner
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

## Binary Card Maps

pkpy exposes pkcore's binary card map types, which provide compact, high-performance hand evaluation storage. These are the building blocks for precomputed lookup tables.

### `Bard`

A 64-bit bitset where each of the 52 cards occupies one bit. Set operations (union, intersection, membership) are single CPU instructions.

```python
from pkpy import Bard, Card, Cards

# Construct
b = Bard.from_card(Card.parse("As"))        # single card
b = Bard.from_cards(Cards.parse("As Ks"))   # from a Cards collection
b = Bard.from_u64(4_362_862_139_015_168)    # from a raw u64

# Constants
Bard.BLANK    # all bits zero
Bard.ALL      # all 52 card bits set

# Operations
b2 = b.fold_in(Card.parse("Qs"))  # returns new Bard with that card added
b.as_u64()                         # -> int  (raw bit value)
b.to_cards()                       # -> Cards  (reconstruct card set)
b.as_guided_string()               # -> str  (debug visualization)
```

### `SevenFiveBCM`

A binary card map entry for a 5- or 7-card hand. Stores the hand's `Bard`, the best 5-card sub-hand's `Bard`, and the hand rank value. This is the format used by pkcore's precomputed CSV lookup table.

`rank` follows the Cactus Kev convention: **lower is stronger** (1 = royal flush, 7462 = worst high card).

```python
from pkpy import Cards, SevenFiveBCM

# Build from a 5-card hand
bcm = SevenFiveBCM.from_cards(Cards.parse("As Ks Qs Js Ts"))
bcm.rank    # -> 1  (royal flush)
bcm.bc      # -> Bard  (bitset of the full hand)
bcm.best    # -> Bard  (bitset of the best 5-card sub-hand; same as bc for 5 cards)

# Build from a 7-card hand — bc is the full 7-card bard, best is the best 5
bcm7 = SevenFiveBCM.from_cards(Cards.parse("As Ks Qs Js Ts 9s 8s"))
bcm7.rank                     # -> 1
bcm7.bc.to_cards()            # -> Cards (7 cards)
bcm7.best.to_cards()          # -> Cards (best 5 cards)

# CSV generation (produces the ~5 GB bcm.csv lookup file — slow)
SevenFiveBCM.default_csv_path           # -> "generated/bcm.csv"
SevenFiveBCM.generate_csv("bcm.csv")   # enumerate all 5- and 7-card combos
```

### `IndexCardMap`

Like `SevenFiveBCM` but stores card hands as human-readable display strings instead of `Bard` bitsets. Useful for inspectable CSV output.

```python
from pkpy import Cards, IndexCardMap

icm = IndexCardMap.from_cards(Cards.parse("As Ks Qs Js Ts"))
icm.rank     # -> 1
icm.cards    # -> "A♠ K♠ Q♠ J♠ T♠"
icm.best     # -> "A♠ K♠ Q♠ J♠ T♠"  (same for 5-card hand)

icm7 = IndexCardMap.from_cards(Cards.parse("As Ks Qs Js Ts 9s 8s"))
icm7.cards   # -> "A♠ K♠ Q♠ J♠ T♠ 9♠ 8♠"  (all 7 cards)
icm7.best    # -> "A♠ K♠ Q♠ J♠ T♠"          (best 5)

IndexCardMap.generate_csv("icm.csv")
```

### BCM example

```python
from pkpy import Cards, SevenFiveBCM, IndexCardMap

hands = [
    Cards.parse("As Ks Qs Js Ts"),      # royal flush
    Cards.parse("As Ks Qs Js 9s"),      # king-high straight flush
    Cards.parse("As Ad Ah Ac Ks"),      # four aces
]

for hand in hands:
    bcm = SevenFiveBCM.from_cards(hand)
    icm = IndexCardMap.from_cards(hand)
    print(f"{icm.cards}  rank={bcm.rank}  best={icm.best}")
```

---

## Pluribus Log Parsing

pkpy can parse hand histories from the [Pluribus](https://en.wikipedia.org/wiki/Pluribus_(poker_bot)) AI poker logs. Each line in a log file is a `STATE` record encoding one hand.

### Log format

```
STATE:{index}:{rounds}:{cards}:{winnings}:{players}
```

- **rounds** — slash-separated betting round strings, e.g. `r200ffcfc/cr850cf`. Each character is `f` (fold), `c` (call), or `r{n}` (raise to n).
- **cards** — pipe-separated two-card hands, optionally followed by `/board`, e.g. `Qc4h|Tc9c|5h5d/3h7s5c/Qs/6c`.
- **winnings** — pipe-separated signed integers, one per player.
- **players** — pipe-separated player names.

### `PluribusEvent`

A single action: fold, call, or raise.

```python
from pkpy import Pluribus

hand = Pluribus.parse("STATE:0:ffr225fff:3c9s|6d5s|9dTs|2sQs|AdKd|7cTc:-50|-100|0|0|150|0:MrWhite|Gogo|Budd|Eddie|Bill|Pluribus")

for event in hand.actions():
    print(event)              # "Fold", "Call", "Raise(225)", etc.
    event.is_fold()           # -> bool
    event.is_call()           # -> bool
    event.is_raise()          # -> bool
    event.raise_amount()      # -> int | None
```

### `Pluribus`

A parsed hand record.

```python
from pkpy import Pluribus

# Parse a single log line
hand = Pluribus.parse("STATE:27:r200ffcfc/cr850cf/cr1825r3775c/r10000c:Qc4h|Tc9c|8sAs|Qh7c|JcQd|5h5d/3h7s5c/Qs/6c:-50|-200|-10000|0|0|10250:Eddie|Bill|Pluribus|MrWhite|Gogo|Budd")

hand.index           # -> 27
hand.players         # -> ['Eddie', 'Bill', 'Pluribus', 'MrWhite', 'Gogo', 'Budd']
hand.winnings        # -> [-50, -200, -10000, 0, 0, 10250]
hand.hole_cards      # -> HoleCards (6 players' hands)
hand.board           # -> Board (3h 7s 5c Qs 6c)
hand.raw             # -> the original log line string

hand.rounds()                 # -> list[str]  raw round strings
hand.actions()                # -> list[PluribusEvent]  all actions flat
hand.actions_for_round(0)     # -> list[PluribusEvent]  actions in round 0
hand.display_results()        # -> str  formatted winnings summary

# Parse an entire log file — invalid lines are silently skipped
hands = Pluribus.read_log("/path/to/pluribus.log")
print(f"Loaded {len(hands)} hands")
```

### Pluribus example

```python
from pkpy import Pluribus

LOG_LINE = "STATE:27:r200ffcfc/cr850cf/cr1825r3775c/r10000c:Qc4h|Tc9c|8sAs|Qh7c|JcQd|5h5d/3h7s5c/Qs/6c:-50|-200|-10000|0|0|10250:Eddie|Bill|Pluribus|MrWhite|Gogo|Budd"

hand = Pluribus.parse(LOG_LINE)

print(f"Hand #{hand.index}")
print(f"Players: {', '.join(hand.players)}")
print(f"Board: {hand.board}")
print(f"Hole cards dealt: {len(hand.hole_cards)} players")

raises = [e for e in hand.actions() if e.is_raise()]
print(f"Raises this hand: {len(raises)}")
for r in raises:
    print(f"  {r.raise_amount()}")

print(hand.display_results())
```

---

## Casino Table Simulation

pkpy exposes pkcore's casino table simulation layer, which models a heads-up or multi-player poker table with blinds, betting, and chip accounting. The key types are `Dealer` (the engine), `Player`, `ForcedBets`, and the log/result types.

### `ForcedBets`

Configures the blinds and optional ante for a hand.

```python
from pkpy import ForcedBets

bets = ForcedBets(small_blind=50, big_blind=100)
bets = ForcedBets(small_blind=50, big_blind=100, ante=25)
```

### `Stack`

A chip count wrapper.

```python
from pkpy import Stack

s = Stack(1000)
s.count()     # -> 1000
s.is_empty()  # -> False
```

### `Player`

A player seated at the table with a name and chip stack.

```python
from pkpy import Player

p = Player("Alice", 1000)
p.handle          # -> "Alice"
p.chips()         # -> 1000  (current stack, excluding chips committed to pot)
p.total_chips()   # -> 1000  (chips + any committed amount)
p.state()         # -> PlayerState
p.is_active()     # -> bool
p.is_folded()     # -> bool
p.is_all_in()     # -> bool
p.is_sitting_out()# -> bool
```

### `PlayerState`

Describes what a player is currently doing at the table.

```python
state = player.state()
state.kind()      # -> str  ("Active", "Folded", "AllIn", "SittingOut")
state.amount()    # -> int  (chips committed in current state, e.g. blind amount)
state.is_active()     # -> bool
state.is_folded()     # -> bool
state.is_all_in()     # -> bool
state.is_sitting_out()# -> bool
```

### `Seatbit`

A compact bitset of occupied seat numbers (seats 0–15).

```python
from pkpy import Seatbit

sb = dealer.ready()
sb.contains(0)   # -> bool  (is seat 0 occupied?)
sb.count()       # -> int   (number of occupied seats)
sb.as_u16()      # -> int   (raw bitset value)
```

### `SeatEquity`

Chip allocation tied to a set of seats — used inside `Win` to record who wins what.

```python
se = win.equity
se.chips         # -> int   (chip amount)
se.seats         # -> Seatbit
se.count()       # -> int   (number of winning seats)
se.is_nada()     # -> bool  (True if chips == 0)
```

### `Win`

One entry in a `Winnings` result. Pairs an equity award with the `Eval` that justified it.

```python
win = winnings.first()
win.equity   # -> SeatEquity
win.eval     # -> Eval
```

### `Winnings`

The payout result returned by `Dealer.end_hand()`.

```python
winnings = dealer.end_hand()
len(winnings)           # -> int  (number of pots/side-pots awarded)
winnings.first()        # -> Win  (main pot winner)
winnings.to_list()      # -> list[Win]
```

### `TableAction`

A single event recorded in the table log.

```python
action = log.last()
action.kind()    # -> str  ("Bet", "Raise", "Call", "Check", "Fold", "PostBlind", etc.)
action.seat()    # -> int  (seat number that took the action)
action.amount()  # -> int  (chip amount, 0 for non-chip actions like fold/check)
```

### `TableLog`

A running record of all actions taken during the hand.

```python
log = dealer.event_log()
log.entries()              # -> list[TableAction]  (all recorded events)
log.last()                 # -> TableAction | None
log.last_player_action()   # -> TableAction | None  (last non-system action)
log.have_posted_blinds()   # -> bool
```

### `Dealer`

The table engine. Manages seating, hand flow, betting, and chip accounting.

```python
from pkpy import Dealer, ForcedBets, Player

dealer = Dealer(ForcedBets(50, 100))

# Seat players — consumes the Player object (ownership transfer)
seat0 = dealer.seat_player(alice)   # -> int  (assigned seat number)
seat1 = dealer.seat_player(bob)

# Hand lifecycle
dealer.start_hand()           # post blinds, deal hole cards
dealer.advance_street()       # deal flop / turn / river
winnings = dealer.end_hand()  # showdown, chip transfer

# Betting actions (seat is the acting seat number)
dealer.bet(seat, amount)
dealer.call(seat)
dealer.check(seat)
dealer.raise_to(seat, amount)
dealer.all_in(seat)
dealer.fold(seat)

# State queries
dealer.ready()           # -> Seatbit  (seats with players ready to play)
dealer.next_to_act()     # -> int | None  (seat that must act next)
dealer.pot()             # -> int  (current pot total)
dealer.chips_at(seat)    # -> int  (chip count at a seat, 0 if empty)
dealer.event_log()       # -> TableLog
```

### Casino example

```python
from pkpy import Dealer, ForcedBets, Player, Winnings

# Set up a heads-up table: 50/100 blinds
dealer = Dealer(ForcedBets(50, 100))

alice = Player("Alice", 1000)
bob   = Player("Bob",   1000)

s_alice = dealer.seat_player(alice)
s_bob   = dealer.seat_player(bob)

# Start the hand — posts blinds, deals hole cards
dealer.start_hand()

print(f"Pot after blinds: {dealer.pot()}")      # -> 0 (blinds not in pot yet)
print(f"Next to act: {dealer.next_to_act()}")   # -> seat of first actor

# Simple action: big blind checks, small blind raises, BB calls
acting = dealer.next_to_act()
dealer.call(acting)                             # SB calls
acting = dealer.next_to_act()
dealer.check(acting)                            # BB checks

# Deal the flop, turn, river
dealer.advance_street()   # flop
dealer.advance_street()   # turn
dealer.advance_street()   # river

# Showdown
winnings = dealer.end_hand()
winner = winnings.first()
print(f"Pot won: {winner.equity.chips}")
print(f"Winning seat(s): {winner.equity.seats.as_u16()}")

# Inspect the action log
for action in dealer.event_log().entries():
    print(f"  seat {action.seat()}: {action.kind()} {action.amount() or ''}")
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

This project wraps pkcore as a versioned crates.io dependency. The wrapper exposes the
analysis-focused surface most useful from Python: card/deck primitives, hand evaluation, outs
calculation, GTO range analysis, heads-up equity, binary card maps, Pluribus log parsing, and
casino table simulation. Lower-level types (SQLite storage) are not exposed.

---

## License

GPL-3.0-or-later, matching pkcore.
