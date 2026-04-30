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
