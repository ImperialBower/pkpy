"""
demo.py — pkcore.py feature showcase

Run with:
    python demo.py
"""

from pkcore import (
    Board,
    Card,
    Cards,
    Game,
    HandRankClass,
    HoleCards,
    Outs,
    Rank,
    Suit,
    distinct_2_card_hands,
    distinct_5_card_hands,
    unique_2_card_hands,
    unique_5_card_hands,
)

SEP = "-" * 60


def section(title):
    print(f"\n{SEP}\n{title}\n{SEP}")


# ============================================================
# Rank
# ============================================================

section("Rank")

print("All rank values:")
for r in [Rank.ACE, Rank.KING, Rank.QUEEN, Rank.JACK, Rank.TEN,
          Rank.NINE, Rank.EIGHT, Rank.SEVEN, Rank.SIX, Rank.FIVE,
          Rank.FOUR, Rank.TREY, Rank.DEUCE, Rank.BLANK]:
    print(f"  {repr(r):20s}  str={str(r)!r:>4}  value={r.value()}")

print(f"\nOrdering: ACE > KING? {Rank.ACE > Rank.KING}")
print(f"Ordering: DEUCE < TREY? {Rank.DEUCE < Rank.TREY}")
print(f"Equality: TEN == TEN? {Rank.TEN == Rank.TEN}")
print(f"Equality: TEN == NINE? {Rank.TEN == Rank.NINE}")


# ============================================================
# Suit
# ============================================================

section("Suit")

for s in [Suit.SPADES, Suit.HEARTS, Suit.DIAMONDS, Suit.CLUBS, Suit.BLANK]:
    print(f"  {repr(s):20s}  str={str(s)!r}  symbol={s.symbol()!r}  letter={s.letter()!r}  value={s.value()}")


# ============================================================
# Card
# ============================================================

section("Card")

# Parsing variants
for notation in ["As", "A♠", "Kh", "K♥", "Qd", "Q♦", "2c"]:
    c = Card.parse(notation)
    print(f"  Card.parse({notation!r:6s}) -> {c}  rank={c.rank()}  suit={c.suit()}  u32={c.as_u32():#010x}  is_dealt={c.is_dealt()}")

# Construction from rank + suit
c = Card.from_rank_suit(Rank.JACK, Suit.CLUBS)
print(f"\n  Card.from_rank_suit(JACK, CLUBS) -> {c}")

# Comparison
a, k = Card.parse("As"), Card.parse("Ks")
print(f"\n  As > Ks? {a > k}")
print(f"  As == As? {a == Card.parse('As')}")

# Invalid parse
try:
    Card.parse("ZZ")
except ValueError as e:
    print(f"\n  Card.parse('ZZ') raises ValueError: {e}")


# ============================================================
# Cards
# ============================================================

section("Cards")

hand = Cards.parse("As Ks Qh")
print(f"  Cards.parse('As Ks Qh') -> {hand}  len={len(hand)}")
print(f"  contains As? {hand.contains(Card.parse('As'))}")
print(f"  contains 2c? {hand.contains(Card.parse('2c'))}")
print(f"  is_dealt={hand.is_dealt()}  are_unique={hand.are_unique()}")

print(f"\n  Iterating:")
for card in hand:
    print(f"    {card}")

print(f"\n  to_list(): {[str(c) for c in hand.to_list()]}")

deck = Cards.deck()
print(f"\n  Cards.deck() has {len(deck)} cards")

remaining = hand.remaining()
print(f"  hand.remaining() has {len(remaining)} cards (deck minus hand)")

board_cards = Cards.parse("Jd Td 9s")
remaining2 = hand.remaining_after(board_cards)
print(f"  hand.remaining_after(board) has {len(remaining2)} cards (deck minus hand minus board)")


# ============================================================
# HoleCards
# ============================================================

section("HoleCards")

hc1 = HoleCards.parse("As Kh")
print(f"  One player:  {hc1}  len={len(hc1)}")

hc2 = HoleCards.parse("As Kh 8d Kc")
print(f"  Two players: {hc2}  len={len(hc2)}")

hc3 = HoleCards.parse("As Kh 8d Kc 5s 5h")
print(f"  Three players: {hc3}  len={len(hc3)}")


# ============================================================
# Board
# ============================================================

section("Board")

flop = Board.parse("Ac 8h 7h")
print(f"  Flop only:       {flop}")

turn = Board.parse("Ac 8h 7h 9s")
print(f"  Flop + turn:     {turn}")
print(f"    turn_cards():  {turn.turn_cards()}  ({len(turn.turn_cards())} cards)")

river = Board.parse("Ac 8h 7h 9s 5s")
print(f"  Full board:      {river}")


# ============================================================
# HandRankClass
# ============================================================

section("HandRankClass")

rf = HandRankClass.ROYAL_FLUSH
print(f"  ROYAL_FLUSH: str={rf}  is_straight_flush={rf.is_straight_flush()}  is_four_of_a_kind={rf.is_four_of_a_kind()}")


# ============================================================
# Game, CaseEvals, Outs
# ============================================================

section("Game / CaseEvals / Outs")

hc    = HoleCards.parse("As Kh 8d Kc")
board = Board.parse("Ac 8h 7h 9s")
game  = Game(hc, board)
print(f"  Game: {game}")

case_evals = game.turn_case_evals()
print(f"  CaseEvals: {case_evals}  (one per possible river card)")

outs = Outs.from_case_evals(case_evals)
print(f"  Outs: {outs}")
print(f"    Player 1 outs: {outs.len_for_player(1)}")
print(f"    Player 2 outs: {outs.len_for_player(2)}")
print(f"    Leading player: {outs.longest_player()}  ({outs.len_longest()} outs)")
print(f"    Player 1 is leading? {outs.is_longest(1)}")
print(f"    Player 2 is leading? {outs.is_longest(2)}")

p2_cards = outs.get(2)
if p2_cards:
    print(f"    Player 2 out cards: {p2_cards}")


# ============================================================
# Full scenario: Negreanu vs Hansen
# ============================================================

section("Full scenario: Negreanu vs Hansen")

print("""
  Daniel Negreanu holds 6♠ 6♥, Gus Hansen holds 5♦ 5♣
  Flop: 9♣ 6♦ 5♥  — Daniel flops top set, Gus flops bottom set
  Turn: 5♠         — Gus makes quad fives
  What river cards give each player a win?
""")

hc    = HoleCards.parse("6s 6h 5d 5c")
board = Board.parse("9c 6d 5h 5s")
outs  = Outs.from_case_evals(Game(hc, board).turn_case_evals())

p1_cards = outs.get(1)
p2_cards = outs.get(2)
print(f"  Player 1 (Daniel, 6♠6♥) outs: {outs.len_for_player(1)}  -> {p1_cards}")
print(f"  Player 2 (Gus,    5♦5♣) outs: {outs.len_for_player(2)}  -> {p2_cards}")
print(f"  Leading player: {outs.longest_player()}")


# ============================================================
# Constants
# ============================================================

section("Constants")

print(f"  unique_5_card_hands():   {unique_5_card_hands():>10,}")
print(f"  distinct_5_card_hands(): {distinct_5_card_hands():>10,}")
print(f"  unique_2_card_hands():   {unique_2_card_hands():>10,}")
print(f"  distinct_2_card_hands(): {distinct_2_card_hands():>10,}")
