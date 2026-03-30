#!/usr/bin/env python3
"""
calc.py — Quick outs calculator for Texas Hold'em.

Takes hole cards and a board, then shows per-player hand evaluations,
win percentages, outs, possible nuts, and the river result.

Usage:
    python examples/calc.py -d "As Kh 8d Kc" -b "Ac 8h 7h 9s"
    python examples/calc.py -d "6♠ 6♥ 5♦ 5♣" -b "9♣ 6♦ 5♥ 5♠"

Interesting hands:
    # HSP THE HAND Negreanu/Hansen
    python examples/calc.py -d "6♠ 6♥ 5♦ 5♣" -b "9♣ 6♦ 5♥ 5♠ 8♠"

    # What about calling this hand The Fold?
    python examples/calc.py -d "5♠ 5♦ 9♠ 9♥ K♣ T♦" -b "5♣ 9♦ T♥ T♣ Q♦"

    # Straight Flush at the river
    python examples/calc.py -d "3♥ A♠ 5♥ A♦ 8♦ 7♦ K♥ K♠ 2♥ Q♠" -b "6♦ 6♣ 7♣ 9♦ 5♦"

    # Two Pair vs Straight Draw
    python examples/calc.py -d "3♠ 9♦ J♠ 8♦ 2♠ Q♠ 6♣ 4♠" -b "Q♥ 5♥ 5♣ 7♥ 4♥"

    # Flopping the nuts
    python examples/calc.py -d "K♠ Q♠ 5♦ K♥ 5♥ J♥" -b "J♦ T♣ A♥ K♣ 2♣"

    # HSP S04E08 Harman/Safai
    python examples/calc.py -d "A♣ Q♠ T♦ T♣ 6♦ 4♦ 2♥ 2♦" -b "J♦ J♠ J♥ A♥ 3♦"

    # HSP S04E08 Elezra/Negreanu
    python examples/calc.py -d "T♦ 2♦ 9♠ 6♥" -b "3♠ 8♦ A♦"

    # HSP S04E08 Farha/Harman/Safai
    python examples/calc.py -d "A♣ 4♠ K♥ 6♥ K♦ T♥" -b "7♠ 3♦ A♠ 4♦"

    # HSP S04E08 Harman/Elezra
    python examples/calc.py -d "6♠ 6♦ A♣ Q♠ A♥ 9♥ Q♦ 5♠" -b "9♦ T♦ 6♥ T♥ K♠"

    # HSP S04E08 Harman/Elezra/Farha
    python examples/calc.py -d "T♠ 9♣ J♦ J♣ Q♥ T♣" -b "T♥ 7♣ A♥ J♠ 8♦"

    # HSP S01E01 Negreanu/Buss/Nasseri
    python examples/calc.py -d "A♦ 7♦ T♠ T♥ K♦ K♥" -b "7♠ 6♥ 4♣"

    # HSP S01E01 Negreanu/Greenstein
    python examples/calc.py -d "A♠ J♦ 6♥ 6♣" -b "A♥ 3♠ 6♠ J♠ 5♠"

    # HSP S01E01 Alaei/Negreanu/Harman
    python examples/calc.py -d "7♣ 6♥ K♣ 2♣ J♦ 9♦" -b "Q♣ 7♥ K♥ 6♣ Q♠"

    # HSP S04E09 Hellmuth/Gold
    python examples/calc.py -d "A♠ K♠ A♣ K♥" -b "4♠ 7♠ K♣"

    # HSP S06E10 Grospellier/Benyamine
    python examples/calc.py -d "6♠ 4♠ 8♣ 6♣ A♦ 2♦ K♥ J♣" -b "2♣ 3♦ 3♣ 4♦ 4♣"

    # HSP S06E11 Galfond/Negreanu
    python examples/calc.py -d "A♠ K♥ 9♦ 8♥" -b "6♦ 7♥ T♣ 3♥ 5♥"

    # HSP S08E07 Bellande/Schwimer (first run)
    python examples/calc.py -d "7♠ 6♠ Q♠ Q♦" -b "2♠ 7♥ 9♠ T♦ 4♣"

    # HSP S08E07 Bellande/Schwimer (second run)
    python examples/calc.py -d "7♠ 6♠ Q♠ Q♦" -b "2♠ 7♥ 9♠ A♠ K♠"

    # DNEGS https://youtu.be/yyPU25EGLkA?t=123
    python examples/calc.py -d "T♦ 9♦ 2♠ 2♥" -b "2♦ T♥ 7♦ 8♦ 6♥"

    # HSP S09E03 DNEGS/Bellande
    python examples/calc.py -d "A♦ Q♠ K♣ Q♦" -b "J♥ 9♠ A♣ 4♦ T♣"

    # HSP S09E04 Adelstein/Liu/Antonius
    python examples/calc.py -d "J♥ 8♠ K♠ J♠ 3♠ 3♥" -b "7♥ 8♦ 2♣ 5♣ Q♠"

    # HSP S09E05 Brunson/Tilly/Antonius
    python examples/calc.py -d "A♥ 8♦ K♣ 7♣ T♥ T♦" -b "4♠ K♦ 2♦ J♥ 3♠"

    # HSP S09E05 Adelstein/Brunson 1st
    python examples/calc.py -d "J♥ J♣ A♥ 4♥" -b "3♣ 4♠ 4♣ 7♣ A♣"

    # HSP S09E05 Adelstein/Brunson 2nd
    python examples/calc.py -d "J♥ J♣ A♥ 4♥" -b "3♣ 4♠ 4♣ 7♣ 9♠"

    # HSP S09E05 Tilly/Hultman
    python examples/calc.py -d "8♦ 5♦ K♦ J♥ 2♠ 2♥" -b "9♥ 2♦ K♥ 4♥ J♠"

    # HSP S09E05 Liu/Tilly/Menon
    python examples/calc.py -d "J♥ J♦ A♠ K♦ T♣ 9♣" -b "7♦ K♠ 2♥ 7♣ A♦"

    # Hand with KDog
    python examples/calc.py -d "7s 6c js 4d" -b "8h 5h 9d"
"""

import argparse
import sys
import time

from pkpy import Board, Game, HoleCards


def main():
    parser = argparse.ArgumentParser(
        description="Texas Hold'em hand evaluator",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("-d", "--dealt", required=True,
                        help='Hole cards for all players, e.g. "As Kh 8d Kc"')
    parser.add_argument("-b", "--board", required=True,
                        help='Board cards (3-5), e.g. "Ac 8h 7h 9s"')
    args = parser.parse_args()

    start = time.monotonic()

    try:
        hc = HoleCards.parse(args.dealt)
        board = Board.parse(args.board)
    except ValueError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

    game = Game(hc, board)
    print(game)

    flop_eval = game.flop_eval()
    if flop_eval:
        print()
        print(flop_eval)

    turn_eval = game.turn_eval()
    if turn_eval:
        print(turn_eval)

        nuts = game.turn_nuts_display()
        if nuts.strip():
            top_10 = "\n".join(nuts.splitlines()[:10])
            print("\nThe Nuts @ Turn:")
            print(top_10)

    river = game.river_display()
    if river.strip():
        print(river)

    print(f'python examples/calc.py -d "{args.dealt}" -b "{args.board}"')
    elapsed = (time.monotonic() - start) * 1000
    print(f"Elapsed: {elapsed:.1f}ms")


if __name__ == "__main__":
    main()
