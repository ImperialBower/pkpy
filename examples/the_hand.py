"""
The Hand — Daniel Negreanu vs Gus Hansen
High Stakes Poker, Season 2, Episode 11

A recreation of one of the most dramatic hands in televised poker history,
walking street-by-street through the action with outs analysis at each stage.

    https://www.youtube.com/watch?v=vjM60lqRhPg
    https://www.youtube.com/watch?v=fEEW06iX4n8

Run with:
    python examples/the_hand.py
"""

from pkpy import Board, Game, HoleCards, Outs

SEP = "=" * 60
THIN = "-" * 60


def section(title):
    print(f"\n{SEP}")
    print(f"  {title}")
    print(SEP)


def street(label, board_str=None):
    print(f"\n{THIN}")
    print(f"  {label}")
    print(THIN)
    if board_str:
        board = Board.parse(board_str)
        print(f"  Board: {board}")
    print()


def show_outs(hc: HoleCards, board: Board):
    game = Game(hc, board)
    outs = Outs.from_case_evals(game.turn_case_evals())

    p1_outs = outs.get(1)
    p2_outs = outs.get(2)

    print(f"  Outs going to the river:")
    print(f"    Daniel (6♠ 6♥): {outs.len_for_player(1)} out(s)  {p1_outs or 'none'}")
    print(f"    Gus    (5♦ 5♣): {outs.len_for_player(2)} out(s)  {p2_outs or 'none'}")

    leader = outs.longest_player()
    name = "Daniel" if leader == 1 else "Gus"
    print(f"\n  Leading player: {name} with {outs.len_longest()} out(s)")


# ============================================================

section("THE HAND — Negreanu vs Hansen")

print("""
  Daniel Negreanu: 6♠ 6♥
  Gus Hansen:      5♦ 5♣

  Blinds: $400/$800
""")

hc = HoleCards.parse("6s 6h 5d 5c")

# ── Preflop ──────────────────────────────────────────────────

street("PREFLOP")
print("  Gus raises to $2,100.")
print("  Daniel re-raises to $5,000.")
print("  Action folds around. Gus calls.")
print("  Pot: ~$10,800")

# ── Flop ─────────────────────────────────────────────────────

street("FLOP", "9c 6d 5h")
print("  Daniel flopped top set (three 6s).")
print("  Gus flopped bottom set (three 5s).")
print()
print("  Gus checks. Daniel bets $8,000.")
print("  Gus check-raises to $26,000. Daniel calls.")
print("  Pot: ~$62,800")

# ── Turn ─────────────────────────────────────────────────────

street("TURN", "9c 6d 5h 5s")
print("  Gus makes quad fives — one of the best hands possible.")
print("  Daniel still has a full house (sixes full of fives),")
print("  but is drawing nearly dead.")
print()

board_at_turn = Board.parse("9c 6d 5h 5s")
show_outs(hc, board_at_turn)

print()
print("  Gus bets $24,000. Daniel calls.")
print("  Pot: ~$110,800")

# ── River ─────────────────────────────────────────────────────

street("RIVER", "9c 6d 5h 5s 8s")
print("  The 8♠ is a blank — Daniel's only out (6♣) did not arrive.")
print()
print("  Gus checks.")
print("  Daniel bets $65,000.")
print("  Gus moves all-in.")
print("  Daniel calls.")
print()
print("  Gus Hansen wins the pot with quad fives.")
print("  Final pot: ~$575,000")

# ── Summary ───────────────────────────────────────────────────

section("SUMMARY")
print("""
  Street     Daniel (6♠ 6♥)          Gus (5♦ 5♣)
  ─────────  ───────────────────────  ─────────────────────────
  Preflop    Pair of sixes            Pair of fives
  Flop       Top set (trip sixes)     Bottom set (trip fives)
  Turn       Full house, 6s full 5s   Quad fives  ← takes the lead
  River      Full house, 6s full 5s   Quad fives  ← wins

  After the turn Daniel needed the case 6♣ to make quad sixes —
  a single out from 44 possible river cards (~2.3%).
""")
