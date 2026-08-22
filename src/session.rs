//! Bindings for pkcore's `casino::session` module.

use crate::table_no_cell::TableNoCell;
use crate::{to_py_err, ForcedBets, Winnings};
use pkcore::casino::action::PlayerAction as PkPlayerAction;
use pkcore::casino::session::PokerSession as PkPokerSession;
use pkcore::casino::session::SessionStep as PkSessionStep;
use pyo3::prelude::*;

/// A player's action in a poker hand.
///
/// Construct via the static methods (`fold()`, `check()`, `call()`,
/// `bet(n)`, `raise_(n)`, `all_in()`). Inspect via `kind()` and `amount()`.
///
/// `raise` is a Python keyword, hence the trailing-underscore naming
/// convention for that constructor.
#[pyclass(from_py_object, name = "PlayerAction")]
#[derive(Clone)]
pub struct PlayerAction(pub(crate) PkPlayerAction);

#[pymethods]
impl PlayerAction {
    #[staticmethod]
    fn fold() -> Self {
        Self(PkPlayerAction::Fold)
    }

    #[staticmethod]
    fn check() -> Self {
        Self(PkPlayerAction::Check)
    }

    #[staticmethod]
    fn call() -> Self {
        Self(PkPlayerAction::Call)
    }

    #[staticmethod]
    fn bet(amount: usize) -> Self {
        Self(PkPlayerAction::Bet(amount))
    }

    #[staticmethod]
    #[pyo3(name = "raise_")]
    fn raise_(amount: usize) -> Self {
        Self(PkPlayerAction::Raise(amount))
    }

    #[staticmethod]
    fn all_in() -> Self {
        Self(PkPlayerAction::AllIn)
    }

    fn kind(&self) -> &'static str {
        match self.0 {
            PkPlayerAction::Fold => "Fold",
            PkPlayerAction::Check => "Check",
            PkPlayerAction::Call => "Call",
            PkPlayerAction::Bet(_) => "Bet",
            PkPlayerAction::Raise(_) => "Raise",
            PkPlayerAction::AllIn => "AllIn",
        }
    }

    fn amount(&self) -> Option<usize> {
        match self.0 {
            PkPlayerAction::Bet(n) | PkPlayerAction::Raise(n) => Some(n),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        match self.0 {
            PkPlayerAction::Bet(n) => format!("PlayerAction.Bet({n})"),
            PkPlayerAction::Raise(n) => format!("PlayerAction.Raise({n})"),
            other => format!("PlayerAction.{other:?}"),
        }
    }

    fn __eq__(&self, other: &PlayerAction) -> bool {
        self.0 == other.0
    }
}

/// A snapshot of where a hand is in its lifecycle.
///
/// Returned by `PokerSession.next_step()`. Read-only; inspect via `kind()`
/// and (for `PlayerToAct`) `seat()`.
#[pyclass(from_py_object, name = "SessionStep")]
#[derive(Clone)]
pub struct SessionStep(pub(crate) PkSessionStep);

#[pymethods]
impl SessionStep {
    fn kind(&self) -> &'static str {
        match self.0 {
            PkSessionStep::PlayerToAct(_) => "PlayerToAct",
            PkSessionStep::StreetAdvanced => "StreetAdvanced",
            PkSessionStep::HandComplete => "HandComplete",
            PkSessionStep::Failed(_) => "Failed",
        }
    }

    /// The error message for a `Failed` step, else `None`.
    ///
    /// A `Failed` step means the hand cannot continue (a dry deck, a failed
    /// chip collection). It is **not** resolvable with `end_hand()` — call
    /// `abort_hand()` to refund committed chips and reset the table.
    fn error(&self) -> Option<String> {
        match self.0 {
            PkSessionStep::Failed(e) => Some(e.to_string()),
            _ => None,
        }
    }

    fn seat(&self) -> Option<u8> {
        match self.0 {
            PkSessionStep::PlayerToAct(s) => Some(s),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        match self.0 {
            PkSessionStep::PlayerToAct(s) => format!("SessionStep.PlayerToAct(seat={s})"),
            PkSessionStep::StreetAdvanced => "SessionStep.StreetAdvanced".to_string(),
            PkSessionStep::HandComplete => "SessionStep.HandComplete".to_string(),
            PkSessionStep::Failed(e) => format!("SessionStep.Failed({e})"),
        }
    }
}

/// A multi-hand poker session wrapping a `TableNoCell`.
///
/// Drive a hand: `start_hand()` → loop on `next_step()`, calling
/// `apply_action(seat, PlayerAction.X)` for each `PlayerToAct` —
/// `end_hand()` to settle. Use `set_blinds` (deferred mid-hand) to
/// adjust forced bets between hands.
#[pyclass(name = "PokerSession")]
pub struct PokerSession(pub(crate) PkPokerSession);

#[pymethods]
impl PokerSession {
    #[new]
    fn new(table: &TableNoCell) -> Self {
        Self(PkPokerSession::new(table.0.clone()))
    }

    /// Convenience: heads-up session in one call. Mirrors
    /// `TableNoCell.heads_up`'s defaults.
    #[staticmethod]
    #[pyo3(signature = (forced, stacks=(1000, 1000), names=("A".to_string(), "B".to_string())))]
    fn heads_up(forced: &ForcedBets, stacks: (usize, usize), names: (String, String)) -> Self {
        let table = TableNoCell::heads_up(forced, stacks, names);
        Self(PkPokerSession::new(table.0))
    }

    // ── 0.0.53 NEW ───────────────────────────────────────────────────────

    fn set_blinds(&mut self, forced: &ForcedBets) {
        self.0.set_blinds(forced.0);
    }

    fn forced_at_hand_start(&self) -> ForcedBets {
        ForcedBets(self.0.forced_at_hand_start())
    }

    // ── Hand lifecycle ──────────────────────────────────────────────────

    fn start_hand(&mut self) -> PyResult<()> {
        self.0.start_hand().map_err(to_py_err)
    }

    fn end_hand(&mut self) -> PyResult<Winnings> {
        self.0.end_hand().map(Winnings).map_err(to_py_err)
    }

    fn is_hand_in_progress(&self) -> bool {
        self.0.is_hand_in_progress()
    }

    fn is_hand_complete(&self) -> bool {
        self.0.is_hand_complete()
    }

    fn next_actor(&mut self) -> PyResult<Option<u8>> {
        self.0.next_actor().map_err(to_py_err)
    }

    fn next_step(&mut self) -> SessionStep {
        SessionStep(self.0.next_step())
    }

    fn apply_action(&mut self, seat: u8, action: &PlayerAction) -> PyResult<()> {
        self.0.apply_action(seat, action.0).map_err(to_py_err)
    }

    /// Abandons a hand that cannot continue, returning the chips refunded.
    ///
    /// Use this when `next_step()` yields a `Failed` step or `next_actor()`
    /// raises: each player's committed chips go back to their stack and the
    /// table resets. `end_hand()` cannot settle such a hand — there was no
    /// showdown to resolve.
    fn abort_hand(&mut self) -> PyResult<usize> {
        self.0.abort_hand().map_err(to_py_err)
    }

    // ── Session-wide ────────────────────────────────────────────────────

    fn count_funded(&self) -> usize {
        self.0.count_funded()
    }

    fn eliminate_busted(&mut self) -> Vec<u8> {
        self.0.eliminate_busted()
    }

    // ── Field accessors ─────────────────────────────────────────────────

    #[getter]
    fn hand_number(&self) -> u32 {
        self.0.hand_number
    }

    #[getter]
    fn shuffled_deck_str(&self) -> Option<String> {
        self.0.shuffled_deck_str.clone()
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerAction>()?;
    m.add_class::<SessionStep>()?;
    m.add_class::<PokerSession>()?;
    Ok(())
}
