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


class TestPokerSession:
    def _heads_up(self, sb=50, bb=100, stacks=(1000, 1000)):
        from pkpy import ForcedBets, PokerSession
        return PokerSession.heads_up(ForcedBets(sb, bb), stacks=stacks)

    def test_construct_from_table(self):
        from pkpy import ForcedBets, PokerSession, TableNoCell
        table = TableNoCell.heads_up(ForcedBets(50, 100))
        session = PokerSession(table)
        assert session.hand_number == 0
        assert session.shuffled_deck_str is None

    def test_heads_up_factory(self):
        session = self._heads_up()
        assert session.hand_number == 0
        assert not session.is_hand_in_progress()

    def test_start_hand_increments_hand_number(self):
        session = self._heads_up()
        session.start_hand()
        assert session.hand_number == 1
        assert session.is_hand_in_progress()

    def test_next_step_after_start_is_player_to_act(self):
        session = self._heads_up()
        session.start_hand()
        step = session.next_step()
        assert step.kind() == "PlayerToAct"
        assert step.seat() is not None

    def test_healthy_step_has_no_error(self):
        # pkcore 0.7.0 added SessionStep::Failed; error() is None otherwise.
        session = self._heads_up()
        session.start_hand()
        assert session.next_step().error() is None

    def test_abort_hand_refunds_and_ends_hand(self):
        # The escape hatch for a Failed step: refund committed chips and
        # reset the table. Blinds are posted at start_hand, so the refund
        # covers at least the small blind.
        session = self._heads_up()
        session.start_hand()
        refunded = session.abort_hand()
        assert isinstance(refunded, int)
        assert refunded > 0
        assert not session.is_hand_in_progress()

    def test_count_funded(self):
        session = self._heads_up()
        assert session.count_funded() == 2

    def test_apply_action_fold_ends_hand(self):
        from pkpy import PlayerAction
        session = self._heads_up()
        session.start_hand()
        actor = session.next_actor()
        assert actor is not None
        session.apply_action(actor, PlayerAction.fold())
        winnings = session.end_hand()
        assert not winnings.is_empty()
        assert len(winnings) >= 1

    # ── 0.0.53 regression ports ──────────────────────────────────────────
    # Direct translations of pkcore unit tests at casino/session.rs:970-1010.

    def test_set_blinds_between_hands_applies_immediately(self):
        from pkpy import ForcedBets
        session = self._heads_up()
        session.set_blinds(ForcedBets(100, 200))
        # Before any hand starts, the snapshot reflects the *new* blinds
        # because PokerSession::new captures the table's current forced
        # bets, and set_blinds (with no hand in progress) overwrites them.
        # We check the snapshot via forced_at_hand_start AFTER start_hand,
        # which is the documented stable surface.
        session.start_hand()
        assert session.forced_at_hand_start().small_blind == 100
        assert session.forced_at_hand_start().big_blind == 200

    def test_set_blinds_during_hand_defers_to_next_hand(self):
        from pkpy import ForcedBets, PlayerAction
        session = self._heads_up()
        session.start_hand()
        # Mid-hand: bump blinds.
        session.set_blinds(ForcedBets(100, 200))
        # forced_at_hand_start still reflects what was posted this hand.
        assert session.forced_at_hand_start().small_blind == 50
        assert session.forced_at_hand_start().big_blind == 100

    def test_deferred_blinds_take_effect_on_next_start_hand(self):
        from pkpy import ForcedBets, PlayerAction
        session = self._heads_up()
        session.start_hand()
        session.set_blinds(ForcedBets(100, 200))
        # Finish the hand by folding the next actor.
        actor = session.next_actor()
        session.apply_action(actor, PlayerAction.fold())
        session.end_hand()
        # Next hand picks up the deferred blinds.
        session.start_hand()
        assert session.forced_at_hand_start().small_blind == 100
        assert session.forced_at_hand_start().big_blind == 200

    def test_forced_at_hand_start_stable_during_hand(self):
        from pkpy import ForcedBets
        session = self._heads_up()
        session.start_hand()
        snap1 = session.forced_at_hand_start()
        session.set_blinds(ForcedBets(400, 800))
        snap2 = session.forced_at_hand_start()
        assert snap1.small_blind == snap2.small_blind == 50
        assert snap1.big_blind == snap2.big_blind == 100
