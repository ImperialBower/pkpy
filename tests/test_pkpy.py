"""Tests for the pkpy Python bindings."""

import pytest
from pkpy import (
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


# ============================================================
# Rank
# ============================================================

class TestRank:
    def test_ace_value(self):
        assert Rank.ACE.value() == 14

    def test_king_value(self):
        assert Rank.KING.value() == 13

    def test_deuce_value(self):
        assert Rank.DEUCE.value() == 2

    def test_blank_value(self):
        assert Rank.BLANK.value() == 0

    def test_str(self):
        assert str(Rank.ACE) == "A"
        assert str(Rank.KING) == "K"
        assert str(Rank.DEUCE) == "2"

    def test_equality(self):
        assert Rank.ACE == Rank.ACE
        assert Rank.ACE != Rank.KING

    def test_ordering(self):
        assert Rank.KING < Rank.ACE
        assert Rank.ACE > Rank.DEUCE
        assert Rank.TEN <= Rank.TEN
        assert Rank.TEN >= Rank.NINE


# ============================================================
# Suit
# ============================================================

class TestSuit:
    def test_spades_value(self):
        assert Suit.SPADES.value() == 4

    def test_clubs_value(self):
        assert Suit.CLUBS.value() == 1

    def test_symbol(self):
        assert Suit.SPADES.symbol() == "♠"
        assert Suit.HEARTS.symbol() == "♥"
        assert Suit.DIAMONDS.symbol() == "♦"
        assert Suit.CLUBS.symbol() == "♣"

    def test_letter(self):
        assert Suit.SPADES.letter() == "S"
        assert Suit.HEARTS.letter() == "H"
        assert Suit.DIAMONDS.letter() == "D"
        assert Suit.CLUBS.letter() == "C"

    def test_equality(self):
        assert Suit.SPADES == Suit.SPADES
        assert Suit.SPADES != Suit.HEARTS


# ============================================================
# Card
# ============================================================

class TestCard:
    def test_parse_abbreviation(self):
        card = Card.parse("As")
        assert card.rank() == Rank.ACE
        assert card.suit() == Suit.SPADES

    def test_parse_unicode(self):
        card = Card.parse("K♥")
        assert card.rank() == Rank.KING
        assert card.suit() == Suit.HEARTS

    def test_parse_lowercase_suit(self):
        card = Card.parse("Qd")
        assert card.rank() == Rank.QUEEN
        assert card.suit() == Suit.DIAMONDS

    def test_parse_invalid_raises(self):
        with pytest.raises(ValueError):
            Card.parse("ZZ")

    def test_is_dealt(self):
        assert Card.parse("As").is_dealt()

    def test_from_rank_suit(self):
        card = Card.from_rank_suit(Rank.ACE, Suit.SPADES)
        assert card.rank() == Rank.ACE
        assert card.suit() == Suit.SPADES

    def test_str(self):
        card = Card.parse("As")
        assert str(card) == "A♠"

    def test_equality(self):
        assert Card.parse("As") == Card.parse("As")
        assert Card.parse("As") != Card.parse("Kh")

    def test_ordering(self):
        ace_spades = Card.parse("As")
        king_spades = Card.parse("Ks")
        assert king_spades < ace_spades
        assert ace_spades > king_spades


# ============================================================
# Cards
# ============================================================

class TestCards:
    def test_parse(self):
        cards = Cards.parse("As Ks")
        assert len(cards) == 2

    def test_deck_has_52_cards(self):
        deck = Cards.deck()
        assert len(deck) == 52

    def test_contains(self):
        cards = Cards.parse("As Ks")
        assert cards.contains(Card.parse("As"))
        assert not cards.contains(Card.parse("Qh"))

    def test_remaining_from_deck(self):
        hand = Cards.parse("As Ks")
        remaining = hand.remaining()
        assert len(remaining) == 50

    def test_remaining_after(self):
        hand = Cards.parse("As Ks")
        board = Cards.parse("Qs Js Ts")
        remaining = hand.remaining_after(board)
        assert len(remaining) == 47

    def test_is_dealt(self):
        assert Cards.parse("As Ks").is_dealt()

    def test_are_unique(self):
        assert Cards.parse("As Ks").are_unique()

    def test_to_list(self):
        cards = Cards.parse("As Ks")
        lst = cards.to_list()
        assert len(lst) == 2
        assert all(isinstance(c, Card) for c in lst)

    def test_iteration(self):
        cards = Cards.parse("As Ks Qh")
        collected = list(cards)
        assert len(collected) == 3
        assert all(isinstance(c, Card) for c in collected)

    def test_str(self):
        cards = Cards.parse("As Ks")
        s = str(cards)
        assert "A♠" in s
        assert "K♠" in s

    def test_parse_duplicate_deduplicates(self):
        # Cards uses an IndexSet, so duplicate cards are silently deduplicated
        cards = Cards.parse("As As")
        assert len(cards) == 1

    def test_parse_invalid_raises(self):
        with pytest.raises(ValueError):
            Cards.parse("ZZ QQ")  # invalid card abbreviations


# ============================================================
# HoleCards
# ============================================================

class TestHoleCards:
    def test_parse_two_players(self):
        hc = HoleCards.parse("As Kh 8d Kc")
        assert len(hc) == 2

    def test_parse_one_player(self):
        hc = HoleCards.parse("As Kh")
        assert len(hc) == 1

    def test_str(self):
        hc = HoleCards.parse("As Kh")
        assert isinstance(str(hc), str)

    def test_parse_invalid_raises(self):
        with pytest.raises(ValueError):
            HoleCards.parse("ZZ ZZ")


# ============================================================
# Board
# ============================================================

class TestBoard:
    def test_parse_five_cards(self):
        board = Board.parse("Ac 8h 7h 9s 5s")
        assert isinstance(str(board), str)

    def test_parse_four_cards(self):
        board = Board.parse("Ac 8h 7h 9s")
        assert isinstance(str(board), str)

    def test_parse_three_cards(self):
        board = Board.parse("Ac 8h 7h")
        assert isinstance(str(board), str)

    def test_turn_cards(self):
        board = Board.parse("Ac 8h 7h 9s")
        tc = board.turn_cards()
        assert len(tc) == 4

    def test_str_format(self):
        board = Board.parse("Ac 8h 7h 9s 5s")
        s = str(board)
        assert "FLOP" in s
        assert "TURN" in s
        assert "RIVER" in s


# ============================================================
# Game and analysis
# ============================================================

class TestGame:
    def _make_game(self):
        hc = HoleCards.parse("As Kh 8d Kc")
        board = Board.parse("Ac 8h 7h 9s")
        return Game(hc, board)

    def test_turn_case_evals_length(self):
        game = self._make_game()
        case_evals = game.turn_case_evals()
        # 52 - 4 hole cards - 4 board cards = 44 remaining cards
        assert len(case_evals) == 44

    def test_outs_from_game(self):
        game = self._make_game()
        case_evals = game.turn_case_evals()
        outs = Outs.from_case_evals(case_evals)
        assert isinstance(outs.len_for_player(1), int)
        assert isinstance(outs.len_for_player(2), int)

    def test_outs_longest_player(self):
        game = self._make_game()
        outs = Outs.from_case_evals(game.turn_case_evals())
        player = outs.longest_player()
        assert player in (1, 2)

    def test_outs_get_returns_cards(self):
        game = self._make_game()
        outs = Outs.from_case_evals(game.turn_case_evals())
        longest = outs.longest_player()
        cards = outs.get(longest)
        assert cards is not None
        assert len(cards) > 0

    def test_outs_is_longest(self):
        game = self._make_game()
        outs = Outs.from_case_evals(game.turn_case_evals())
        longest = outs.longest_player()
        assert outs.is_longest(longest)


# ============================================================
# HandRankClass
# ============================================================

class TestHandRankClass:
    def test_royal_flush_str(self):
        assert str(HandRankClass.ROYAL_FLUSH) == "RoyalFlush"

    def test_royal_flush_is_straight_flush(self):
        assert HandRankClass.ROYAL_FLUSH.is_straight_flush()


# ============================================================
# Constants
# ============================================================

class TestConstants:
    def test_unique_5_card_hands(self):
        assert unique_5_card_hands() == 2_598_960

    def test_distinct_5_card_hands(self):
        assert distinct_5_card_hands() == 7_462

    def test_unique_2_card_hands(self):
        assert unique_2_card_hands() == 1_326

    def test_distinct_2_card_hands(self):
        assert distinct_2_card_hands() == 169
