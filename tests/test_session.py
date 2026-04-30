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


class TestSessionStep:
    """SessionStep is read-only — produced by PokerSession.next_step().

    These tests construct one indirectly via a session in Phase 7. For now,
    we just confirm the type exists in the module so import doesn't fail.
    """

    def test_import(self):
        from pkpy import SessionStep
        # Class must exist; instances are created by PokerSession.next_step.
        assert SessionStep is not None
