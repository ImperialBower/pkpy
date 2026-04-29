//! Bindings for pkcore's casino::session module.

use pkcore::casino::action::PlayerAction as PkPlayerAction;
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

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerAction>()?;
    Ok(())
}
