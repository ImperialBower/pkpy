"""
pkpy — Python bindings for the pkcore Rust poker analysis library.

Provides card types, hand evaluation, and Texas Hold'em game simulation.

Basic usage::

    from pkpy import Card, Cards, HoleCards, Board, Game, Outs

    # Parse individual cards
    ace = Card.parse("As")
    king = Card.parse("Kh")

    # Build a game and calculate outs
    hc = HoleCards.parse("As Kh 8d Kc")
    board = Board.parse("Ac 8h 7h 9s")
    game = Game(hc, board)
    outs = Outs.from_case_evals(game.turn_case_evals())
    print(f"Player 1 outs: {outs.len_for_player(1)}")
    print(f"Player 2 outs: {outs.len_for_player(2)}")
"""

from pkpy._pkpy import (
    Board,
    Card,
    Cards,
    CaseEvals,
    Combo,
    ComboPairs,
    Combos,
    Deck,
    Eval,
    FlopEval,
    Game,
    HandRank,
    HandRankClass,
    HoleCards,
    HUPResult,
    Outs,
    Pluribus,
    PluribusEvent,
    Qualifier,
    Rank,
    Suit,
    TurnEval,
    Two,
    Twos,
    Versus,
    WinLoseDraw,
    unique_2_card_hands,
    unique_5_card_hands,
    distinct_2_card_hands,
    distinct_5_card_hands,
)

__all__ = [
    "Board",
    "Card",
    "Cards",
    "CaseEvals",
    "Combo",
    "ComboPairs",
    "Combos",
    "Deck",
    "Eval",
    "FlopEval",
    "Game",
    "HandRank",
    "HandRankClass",
    "HoleCards",
    "HUPResult",
    "Outs",
    "Pluribus",
    "PluribusEvent",
    "Qualifier",
    "Rank",
    "Suit",
    "TurnEval",
    "Two",
    "Twos",
    "Versus",
    "WinLoseDraw",
    "unique_2_card_hands",
    "unique_5_card_hands",
    "distinct_2_card_hands",
    "distinct_5_card_hands",
]
