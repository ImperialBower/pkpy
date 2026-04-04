"""Tests for the pkpy Python bindings."""

import pytest
from pkpy import (
    Bard,
    Board,
    Card,
    Cards,
    Deck,
    Game,
    HandRankClass,
    HoleCards,
    IndexCardMap,
    Outs,
    Pluribus,
    PluribusEvent,
    Rank,
    SevenFiveBCM,
    Suit,
    Two,
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


# ============================================================
# Rank (new methods)
# ============================================================

class TestRankNewMethods:
    def test_prime_ace(self):
        assert Rank.ACE.prime() == 41

    def test_prime_deuce(self):
        assert Rank.DEUCE.prime() == 2

    def test_bits_nonzero(self):
        assert Rank.ACE.bits() > 0
        assert Rank.DEUCE.bits() > 0

    def test_number_ace(self):
        assert Rank.ACE.number() == 12

    def test_number_deuce(self):
        assert Rank.DEUCE.number() == 0

    def test_number_ordering(self):
        assert Rank.DEUCE.number() < Rank.TREY.number() < Rank.ACE.number()


# ============================================================
# Card (new methods)
# ============================================================

class TestCardNewMethods:
    def test_bit_string_is_string(self):
        s = Card.parse("As").bit_string()
        assert isinstance(s, str)
        assert len(s) > 0

    def test_get_rank_prime(self):
        assert Card.parse("As").get_rank_prime() == 41
        assert Card.parse("2c").get_rank_prime() == 2

    def test_get_letter_index(self):
        assert Card.parse("As").get_letter_index() == "AS"
        assert Card.parse("K♥").get_letter_index() == "KH"


# ============================================================
# Deck
# ============================================================

class TestDeck:
    def test_len(self):
        assert Deck.len() == 52

    def test_poker_cards_length(self):
        assert len(Deck.poker_cards()) == 52

    def test_poker_cards_shuffled_length(self):
        assert len(Deck.poker_cards_shuffled()) == 52

    def test_poker_cards_is_dealt(self):
        assert Deck.poker_cards().is_dealt()

    def test_get_first_card(self):
        card = Deck.get(0)
        assert isinstance(card, Card)
        assert card.is_dealt()

    def test_get_all_indices(self):
        cards = [Deck.get(i) for i in range(52)]
        assert len(set(str(c) for c in cards)) == 52

    def test_shuffled_contains_same_cards(self):
        ordered = Deck.poker_cards()
        shuffled = Deck.poker_cards_shuffled()
        assert len(shuffled) == len(ordered)
        for card in ordered.to_list():
            assert shuffled.contains(card)


# ============================================================
# Cards (new methods)
# ============================================================

class TestCardsNewMethods:
    def test_is_empty_false(self):
        assert not Cards.parse("As Ks").is_empty()

    def test_is_empty_true(self):
        cards = Cards.parse("As")
        cards.draw_all()
        assert cards.is_empty()

    def test_insert(self):
        cards = Cards.parse("As")
        inserted = cards.insert(Card.parse("Ks"))
        assert inserted
        assert len(cards) == 2

    def test_insert_duplicate_returns_false(self):
        cards = Cards.parse("As")
        assert not cards.insert(Card.parse("As"))
        assert len(cards) == 1

    def test_remove(self):
        cards = Cards.parse("As Ks")
        removed = cards.remove(Card.parse("As"))
        assert removed
        assert len(cards) == 1
        assert not cards.contains(Card.parse("As"))

    def test_remove_absent_returns_false(self):
        cards = Cards.parse("As")
        assert not cards.remove(Card.parse("Ks"))

    def test_get_index(self):
        cards = Cards.parse("As Ks")
        assert cards.get_index(0) == Card.parse("As")
        assert cards.get_index(1) == Card.parse("Ks")
        assert cards.get_index(99) is None

    def test_append(self):
        a = Cards.parse("As Ks")
        b = Cards.parse("Qh Jh")
        a.append(b)
        assert len(a) == 4

    def test_draw_one(self):
        cards = Cards.parse("As Ks Qh")
        card = cards.draw_one()
        assert isinstance(card, Card)
        assert len(cards) == 2

    def test_draw_one_empty_raises(self):
        cards = Cards.parse("As")
        cards.draw_one()
        with pytest.raises(ValueError):
            cards.draw_one()

    def test_draw(self):
        cards = Cards.parse("As Ks Qh Jd")
        drawn = cards.draw(2)
        assert len(drawn) == 2
        assert len(cards) == 2

    def test_draw_too_many_raises(self):
        cards = Cards.parse("As Ks")
        with pytest.raises(ValueError):
            cards.draw(5)

    def test_draw_all(self):
        cards = Cards.parse("As Ks Qh")
        rest = cards.draw_all()
        assert len(rest) == 3
        assert len(cards) == 0

    def test_shuffle_returns_same_cards(self):
        cards = Cards.parse("As Ks Qh Jd Tc")
        shuffled = cards.shuffle()
        assert len(shuffled) == len(cards)
        for card in cards.to_list():
            assert shuffled.contains(card)

    def test_shuffle_in_place(self):
        cards = Cards.parse("As Ks Qh Jd Tc")
        original = set(str(c) for c in cards.to_list())
        cards.shuffle_in_place()
        shuffled = set(str(c) for c in cards.to_list())
        assert original == shuffled

    def test_sort(self):
        cards = Cards.parse("2c As Kh")
        sorted_cards = cards.sort()
        lst = sorted_cards.to_list()
        assert lst[0].rank() == Rank.ACE

    def test_filter_by_suit(self):
        cards = Cards.parse("As Kh Qh Jd")
        hearts = cards.filter_by_suit(Suit.HEARTS)
        assert len(hearts) == 2
        for card in hearts.to_list():
            assert card.suit() == Suit.HEARTS

    def test_minus(self):
        a = Cards.parse("As Ks Qh Jd")
        b = Cards.parse("As Qh")
        result = a.minus(b)
        assert len(result) == 2
        assert not result.contains(Card.parse("As"))
        assert not result.contains(Card.parse("Qh"))

    def test_combinations(self):
        cards = Cards.parse("As Ks Qh")
        combos = cards.combinations(2)
        assert len(combos) == 3
        assert all(len(c) == 2 for c in combos)

    def test_deck_minus(self):
        hand = Cards.parse("As Ks")
        rest = hand.deck_minus()
        assert len(rest) == 50
        assert not rest.contains(Card.parse("As"))
        assert not rest.contains(Card.parse("Ks"))

    def test_deck_primed(self):
        hand = Cards.parse("As Ks")
        primed = hand.deck_primed()
        assert len(primed) == 52
        assert primed.get_index(0) == Card.parse("As")
        assert primed.get_index(1) == Card.parse("Ks")


# ============================================================
# HoleCards (new methods)
# ============================================================

class TestHoleCardsNewMethods:
    def test_is_empty_false(self):
        hc = HoleCards.parse("As Kh")
        assert not hc.is_empty()

    def test_get(self):
        hc = HoleCards.parse("As Kh 8d Kc")
        first = hc.get(0)
        assert first is not None
        assert isinstance(first, Two)

    def test_get_out_of_bounds(self):
        hc = HoleCards.parse("As Kh")
        assert hc.get(5) is None

    def test_to_list(self):
        hc = HoleCards.parse("As Kh 8d Kc")
        lst = hc.to_list()
        assert len(lst) == 2
        assert all(isinstance(t, Two) for t in lst)

    def test_push(self):
        hc = HoleCards.parse("As Kh")
        hc.push(Two.parse("Qd Jc"))
        assert len(hc) == 2


# ============================================================
# Game (new methods)
# ============================================================

class TestGameNewMethods:
    def _make_turn_game(self):
        hc = HoleCards.parse("As Kh 8d Kc")
        board = Board.parse("Ac 8h 7h 9s")
        return Game(hc, board)

    def _make_flop_game(self):
        hc = HoleCards.parse("As Kh 8d Kc")
        board = Board.parse("Ac 8h 7h")
        return Game(hc, board)

    def test_has_dealt_turn_true(self):
        assert self._make_turn_game().has_dealt_turn()

    def test_has_dealt_turn_false(self):
        assert not self._make_flop_game().has_dealt_turn()

    def test_turn_eval_for_player(self):
        game = self._make_turn_game()
        eval_ = game.turn_eval_for_player(0)
        assert eval_ is not None

    def test_turn_remaining_board(self):
        game = self._make_turn_game()
        remaining = game.turn_remaining_board()
        assert isinstance(remaining, Cards)
        # excludes the 4 board cards only (not hole cards)
        assert len(remaining) == 48

    def test_flop_and_turn(self):
        game = self._make_turn_game()
        four = game.flop_and_turn()
        assert isinstance(four, Cards)
        assert len(four) == 4


# ============================================================
# Pluribus
# ============================================================

LOG = "STATE:27:r200ffcfc/cr850cf/cr1825r3775c/r10000c:Qc4h|Tc9c|8sAs|Qh7c|JcQd|5h5d/3h7s5c/Qs/6c:-50|-200|-10000|0|0|10250:Eddie|Bill|Pluribus|MrWhite|Gogo|Budd"
PREFLOP_LOG = "STATE:0:ffr225fff:3c9s|6d5s|9dTs|2sQs|AdKd|7cTc:-50|-100|0|0|150|0:MrWhite|Gogo|Budd|Eddie|Bill|Pluribus"


class TestPluribusEvent:
    def test_fold(self):
        hand = Pluribus.parse(PREFLOP_LOG)
        folds = [e for e in hand.actions() if e.is_fold()]
        assert len(folds) > 0
        assert folds[0].is_fold()
        assert not folds[0].is_call()
        assert not folds[0].is_raise()
        assert folds[0].raise_amount() is None

    def test_raise(self):
        hand = Pluribus.parse(PREFLOP_LOG)
        raises = [e for e in hand.actions() if e.is_raise()]
        assert len(raises) == 1
        assert raises[0].raise_amount() == 225

    def test_str_fold(self):
        hand = Pluribus.parse(PREFLOP_LOG)
        fold = next(e for e in hand.actions() if e.is_fold())
        assert str(fold) == "Fold"

    def test_str_raise(self):
        hand = Pluribus.parse(PREFLOP_LOG)
        raise_ = next(e for e in hand.actions() if e.is_raise())
        assert "225" in str(raise_)


class TestPluribus:
    def test_parse_index(self):
        hand = Pluribus.parse(LOG)
        assert hand.index == 27

    def test_parse_players(self):
        hand = Pluribus.parse(LOG)
        assert hand.players == ["Eddie", "Bill", "Pluribus", "MrWhite", "Gogo", "Budd"]

    def test_parse_winnings(self):
        hand = Pluribus.parse(LOG)
        assert hand.winnings == [-50, -200, -10000, 0, 0, 10250]
        assert sum(hand.winnings) == 0

    def test_parse_hole_cards(self):
        hand = Pluribus.parse(LOG)
        assert len(hand.hole_cards) == 6

    def test_parse_board(self):
        hand = Pluribus.parse(LOG)
        assert isinstance(hand.board, Board)

    def test_parse_raw(self):
        hand = Pluribus.parse(LOG)
        assert hand.raw == LOG

    def test_rounds(self):
        hand = Pluribus.parse(LOG)
        rounds = hand.rounds()
        assert len(rounds) == 4
        assert rounds[0] == "r200ffcfc"

    def test_actions_flat(self):
        hand = Pluribus.parse(LOG)
        actions = hand.actions()
        assert len(actions) > 0
        assert all(isinstance(e, PluribusEvent) for e in actions)

    def test_actions_for_round(self):
        hand = Pluribus.parse(LOG)
        round0 = hand.actions_for_round(0)
        assert len(round0) > 0
        assert round0[0].is_raise()
        assert round0[0].raise_amount() == 200

    def test_actions_for_invalid_round(self):
        hand = Pluribus.parse(LOG)
        assert hand.actions_for_round(99) == []

    def test_display_results(self):
        hand = Pluribus.parse(LOG)
        result = hand.display_results()
        assert isinstance(result, str)
        assert len(result) > 0

    def test_parse_preflop_only(self):
        hand = Pluribus.parse(PREFLOP_LOG)
        assert hand.index == 0
        assert len(hand.players) == 6

    def test_parse_invalid_raises(self):
        with pytest.raises(ValueError):
            Pluribus.parse("not a valid log line")

    def test_str(self):
        hand = Pluribus.parse(LOG)
        assert isinstance(str(hand), str)


# ============================================================
# Bard
# ============================================================

ROYAL_FLUSH_BARD_U64 = 4_362_862_139_015_168  # A♠ K♠ Q♠ J♠ T♠


class TestBard:
    def test_from_u64_roundtrip(self):
        b = Bard.from_u64(ROYAL_FLUSH_BARD_U64)
        assert b.as_u64() == ROYAL_FLUSH_BARD_U64

    def test_from_card(self):
        b = Bard.from_card(Card.parse("As"))
        assert b.as_u64() > 0

    def test_from_cards(self):
        b = Bard.from_cards(Cards.parse("As Ks"))
        assert b.as_u64() > 0

    def test_blank_is_zero(self):
        assert Bard.BLANK.as_u64() == 0

    def test_all_has_52_bits(self):
        b = Bard.ALL
        assert bin(b.as_u64()).count("1") == 52

    def test_fold_in(self):
        b = Bard.BLANK
        b2 = b.fold_in(Card.parse("As"))
        assert b2.as_u64() > 0
        assert b.as_u64() == 0  # original unchanged

    def test_to_cards_roundtrip(self):
        original = Cards.parse("As Ks Qh")
        b = Bard.from_cards(original)
        recovered = b.to_cards()
        assert len(recovered) == 3
        for card in original.to_list():
            assert recovered.contains(card)

    def test_single_card_roundtrip(self):
        card = Card.parse("As")
        b = Bard.from_card(card)
        recovered = b.to_cards()
        assert len(recovered) == 1
        assert recovered.contains(card)

    def test_equality(self):
        b1 = Bard.from_cards(Cards.parse("As Ks"))
        b2 = Bard.from_cards(Cards.parse("As Ks"))
        assert b1 == b2

    def test_inequality(self):
        b1 = Bard.from_cards(Cards.parse("As Ks"))
        b2 = Bard.from_cards(Cards.parse("As Qh"))
        assert b1 != b2

    def test_as_guided_string(self):
        b = Bard.from_card(Card.parse("As"))
        s = b.as_guided_string()
        assert isinstance(s, str)
        assert len(s) > 0

    def test_repr(self):
        b = Bard.from_u64(42)
        assert "42" in repr(b)

    def test_hash(self):
        b1 = Bard.from_u64(100)
        b2 = Bard.from_u64(100)
        assert hash(b1) == hash(b2)


# ============================================================
# SevenFiveBCM
# ============================================================

ROYAL_FLUSH_5 = Cards.parse("As Ks Qs Js Ts")
ROYAL_FLUSH_7 = Cards.parse("As Ks Qs Js Ts 9s 8s")


class TestSevenFiveBCM:
    def test_five_card_rank(self):
        bcm = SevenFiveBCM.from_cards(ROYAL_FLUSH_5)
        assert bcm.rank == 1

    def test_seven_card_rank(self):
        bcm = SevenFiveBCM.from_cards(ROYAL_FLUSH_7)
        assert bcm.rank == 1

    def test_five_card_bc_equals_best(self):
        bcm = SevenFiveBCM.from_cards(ROYAL_FLUSH_5)
        assert bcm.bc == bcm.best

    def test_seven_card_bc_differs_from_best(self):
        bcm = SevenFiveBCM.from_cards(ROYAL_FLUSH_7)
        assert bcm.bc != bcm.best

    def test_seven_best_is_five_cards(self):
        bcm = SevenFiveBCM.from_cards(ROYAL_FLUSH_7)
        best_cards = bcm.best.to_cards()
        assert len(best_cards) == 5

    def test_bc_is_bard(self):
        bcm = SevenFiveBCM.from_cards(ROYAL_FLUSH_5)
        assert isinstance(bcm.bc, Bard)
        assert isinstance(bcm.best, Bard)

    def test_invalid_card_count_returns_default(self):
        # pkcore returns Ok(default) for unsupported counts rather than Err
        bcm = SevenFiveBCM.from_cards(Cards.parse("As Ks Qs"))
        assert bcm.rank == 0

    def test_equality(self):
        bcm1 = SevenFiveBCM.from_cards(ROYAL_FLUSH_5)
        bcm2 = SevenFiveBCM.from_cards(ROYAL_FLUSH_5)
        assert bcm1 == bcm2

    def test_default_csv_path(self):
        assert isinstance(SevenFiveBCM.default_csv_path, str)
        assert SevenFiveBCM.default_csv_path.endswith(".csv")

    def test_repr(self):
        bcm = SevenFiveBCM.from_cards(ROYAL_FLUSH_5)
        r = repr(bcm)
        assert "rank=1" in r


# ============================================================
# IndexCardMap
# ============================================================

class TestIndexCardMap:
    def test_five_card_rank(self):
        icm = IndexCardMap.from_cards(ROYAL_FLUSH_5)
        assert icm.rank == 1

    def test_seven_card_rank(self):
        icm = IndexCardMap.from_cards(ROYAL_FLUSH_7)
        assert icm.rank == 1

    def test_five_card_cards_equals_best(self):
        icm = IndexCardMap.from_cards(ROYAL_FLUSH_5)
        assert icm.cards == icm.best

    def test_seven_card_cards_differs_from_best(self):
        icm = IndexCardMap.from_cards(ROYAL_FLUSH_7)
        assert icm.cards != icm.best

    def test_best_contains_five_cards(self):
        icm = IndexCardMap.from_cards(ROYAL_FLUSH_7)
        best_cards = Cards.parse(icm.best)
        assert len(best_cards) == 5

    def test_cards_are_strings(self):
        icm = IndexCardMap.from_cards(ROYAL_FLUSH_5)
        assert isinstance(icm.cards, str)
        assert isinstance(icm.best, str)

    def test_invalid_card_count_returns_default(self):
        # pkcore returns Ok(default) for unsupported counts rather than Err
        icm = IndexCardMap.from_cards(Cards.parse("As Ks Qs"))
        assert icm.rank == 0

    def test_equality(self):
        icm1 = IndexCardMap.from_cards(ROYAL_FLUSH_5)
        icm2 = IndexCardMap.from_cards(ROYAL_FLUSH_5)
        assert icm1 == icm2

    def test_repr(self):
        icm = IndexCardMap.from_cards(ROYAL_FLUSH_5)
        r = repr(icm)
        assert "rank=1" in r
