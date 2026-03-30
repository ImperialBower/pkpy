#!/usr/bin/env python3
"""
gto.py — GTO preflop range-vs-hand equity calculator for Texas Hold'em.

Given a hero hand and a villain range, shows combo pairs, per-hand odds,
and consolidated equity. Optionally takes a board for postflop analysis.

Usage:
    python examples/gto.py -p "K♠ K♥" -v "66+,AJs+,KQs,AJo+,KQo"
    python examples/gto.py -p "A♠ K♠" -v "QQ+,AKs,AKo"
    python examples/gto.py -p "K♠ K♥" -v "66+,AJs+,KQs,AJo+,KQo" -b "9♣ 6♦ 5♥"
"""

import argparse
import sys
import time

from pkpy import Board, Combos, Two, Versus


def main():
    parser = argparse.ArgumentParser(
        description="GTO range-vs-hand equity calculator",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("-p", "--player", required=True,
                        help='Hero hole cards, e.g. "K♠ K♥"')
    parser.add_argument("-v", "--villain", required=True,
                        help='Villain range, e.g. "66+,AJs+,KQs,AJo+,KQo"')
    parser.add_argument("-b", "--board", required=False, default=None,
                        help='Board cards (3-5), e.g. "9♣ 6♦ 5♥"')
    args = parser.parse_args()

    start = time.monotonic()

    try:
        hero = Two.parse(args.player)
        villain = Combos.parse(args.villain)
    except ValueError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

    if args.board:
        try:
            board = Board.parse(args.board)
        except ValueError as e:
            print(f"Error: {e}", file=sys.stderr)
            sys.exit(1)
        solver = Versus.with_board(hero, villain, board)
    else:
        solver = Versus(hero, villain)

    print(solver)
    print()

    print("Villain combos before your blockers:")
    print(solver.villain.combo_pairs())
    print()

    print("Villain combos after your blockers:")
    print(solver.combo_pairs())
    print()

    hups = solver.hups_at_deal()

    print("Odds per hand matchup:")
    for hup in hups:
        print(hup)

    results = Versus.combined_odds_at_deal(hups)
    print()
    print("Consolidated odds:")
    print(results)

    if solver.has_board():
        for game in solver.games_at_flop():
            fe = game.flop_eval()
            if fe:
                print(fe)
                print(fe.to_win_lose_draw())
        print(f"FLOP: {solver.combined_odds_at_flop()}")
        print(f"TURN: {solver.combined_odds_at_turn()}")

    print()
    elapsed = (time.monotonic() - start) * 1000
    print(f"Elapsed: {elapsed:.1f}ms")


if __name__ == "__main__":
    main()
