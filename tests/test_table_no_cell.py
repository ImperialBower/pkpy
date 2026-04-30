"""Tests for pkpy no-cell table primitive bindings."""

import pytest

from pkpy import PlayerNoCell


class TestPlayerNoCell:
    def test_construct_default_chips(self):
        p = PlayerNoCell("Alice")
        assert p.total_chip_count() == 0
        assert p.is_clear()

    def test_construct_with_chips(self):
        p = PlayerNoCell("Alice", chips=1000)
        assert p.total_chip_count() == 1000

    def test_construct_with_positional_chips(self):
        p = PlayerNoCell("Alice", 1000)
        assert p.total_chip_count() == 1000

    def test_state_predicates_default(self):
        p = PlayerNoCell("Alice", chips=1000)
        assert not p.is_all_in()
        assert not p.has_bet()

    def test_repr_contains_handle(self):
        r = repr(PlayerNoCell("Alice", chips=1000))
        assert "Alice" in r


class TestSeatNoCell:
    def test_construct_from_player(self):
        from pkpy import SeatNoCell
        seat = SeatNoCell(PlayerNoCell("Alice", chips=1000))
        assert not seat.is_empty()

    def test_default_state_predicates(self):
        from pkpy import SeatNoCell
        seat = SeatNoCell(PlayerNoCell("Alice", chips=1000))
        # A fresh, funded seat is "yet to act" and not all-in.
        # Note: is_in_hand() is True for any funded, non-folded seat — it
        # answers "is this seat eligible to play?" not "is a hand in progress?"
        assert seat.is_yet_to_act()
        assert not seat.is_all_in()

    def test_repr_contains_handle(self):
        from pkpy import SeatNoCell
        r = repr(SeatNoCell(PlayerNoCell("Alice", chips=1000)))
        assert "Alice" in r


class TestSeatsNoCell:
    def _two_seats(self):
        from pkpy import SeatNoCell
        return [
            SeatNoCell(PlayerNoCell("Alice", chips=1000)),
            SeatNoCell(PlayerNoCell("Bob", chips=2000)),
        ]

    def test_construct_from_list(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        assert seats.size() == 2

    def test_total_chip_count(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        assert seats.total_chip_count() == 3000

    def test_get_seat(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        seat = seats.get_seat(0)
        assert seat is not None
        assert not seat.is_empty()

    def test_get_seat_out_of_range(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        assert seats.get_seat(99) is None

    def test_default_betting_state(self):
        from pkpy import SeatsNoCell
        seats = SeatsNoCell(self._two_seats())
        # Before any hand starts: no bets posted, no cards dealt.
        # count_active_in_hand reflects funded seats (= 2), not whether a
        # hand is in progress — see TestSeatNoCell on the same distinction.
        assert seats.current_bet() == 0
        assert not seats.are_dealt()
        assert seats.count_active_in_hand() == 2

    def test_repr_includes_size(self):
        from pkpy import SeatsNoCell
        r = repr(SeatsNoCell(self._two_seats()))
        assert "size=2" in r
