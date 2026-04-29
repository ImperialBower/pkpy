"""Tests for pkpy poker session bindings."""

import pytest

from pkpy import PlayerAction


class TestPlayerAction:
    def test_fold(self):
        a = PlayerAction.fold()
        assert a.kind() == "Fold"
        assert a.amount() is None

    def test_check(self):
        a = PlayerAction.check()
        assert a.kind() == "Check"
        assert a.amount() is None

    def test_call(self):
        a = PlayerAction.call()
        assert a.kind() == "Call"
        assert a.amount() is None

    def test_bet(self):
        a = PlayerAction.bet(200)
        assert a.kind() == "Bet"
        assert a.amount() == 200

    def test_raise_(self):
        a = PlayerAction.raise_(400)
        assert a.kind() == "Raise"
        assert a.amount() == 400

    def test_all_in(self):
        a = PlayerAction.all_in()
        assert a.kind() == "AllIn"
        assert a.amount() is None

    def test_equality(self):
        assert PlayerAction.bet(200) == PlayerAction.bet(200)
        assert PlayerAction.bet(200) != PlayerAction.bet(300)
        assert PlayerAction.fold() == PlayerAction.fold()
        assert PlayerAction.fold() != PlayerAction.check()

    def test_repr_contains_kind(self):
        assert "Bet" in repr(PlayerAction.bet(200))
        assert "200" in repr(PlayerAction.bet(200))
        assert "Fold" in repr(PlayerAction.fold())
