//! Bindings for pkcore's casino::table_no_cell module.

use pkcore::casino::table_no_cell::PlayerNoCell as PkPlayerNoCell;
use pkcore::casino::table_no_cell::SeatNoCell as PkSeatNoCell;
use pyo3::prelude::*;

/// A no-cell player record (handle + chip stack + state flags).
///
/// Constructed standalone or via `PlayerNoCell(handle, chips=N)`. Wrapped
/// in `SeatNoCell` for table assembly.
#[pyclass(from_py_object, name = "PlayerNoCell")]
#[derive(Clone)]
pub struct PlayerNoCell(pub(crate) PkPlayerNoCell);

#[pymethods]
impl PlayerNoCell {
    #[new]
    #[pyo3(signature = (handle, chips=0))]
    fn new(handle: String, chips: usize) -> Self {
        if chips == 0 {
            Self(PkPlayerNoCell::new(handle))
        } else {
            Self(PkPlayerNoCell::new_with_chips(handle, chips))
        }
    }

    fn total_chip_count(&self) -> usize {
        self.0.total_chip_count()
    }

    fn is_active(&self) -> bool {
        self.0.is_active()
    }

    fn is_all_in(&self) -> bool {
        self.0.is_all_in()
    }

    fn is_in_hand(&self) -> bool {
        self.0.is_in_hand()
    }

    fn is_out(&self) -> bool {
        self.0.is_out()
    }

    fn is_tapped_out(&self) -> bool {
        self.0.is_tapped_out()
    }

    fn is_clear(&self) -> bool {
        self.0.is_clear()
    }

    fn has_bet(&self) -> bool {
        self.0.has_bet()
    }

    fn __repr__(&self) -> String {
        format!("PlayerNoCell({})", self.0)
    }
}

/// A seat at a no-cell table, wrapping a `PlayerNoCell`.
#[pyclass(from_py_object, name = "SeatNoCell")]
#[derive(Clone)]
pub struct SeatNoCell(pub(crate) PkSeatNoCell);

#[pymethods]
impl SeatNoCell {
    #[new]
    fn new(player: &PlayerNoCell) -> Self {
        Self(PkSeatNoCell::new(player.0.clone()))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn is_active(&self) -> bool {
        self.0.is_active()
    }

    fn is_all_in(&self) -> bool {
        self.0.is_all_in()
    }

    fn is_in_hand(&self) -> bool {
        self.0.is_in_hand()
    }

    fn is_yet_to_act(&self) -> bool {
        self.0.is_yet_to_act()
    }

    fn is_yet_to_act_or_blind(&self) -> bool {
        self.0.is_yet_to_act_or_blind()
    }

    fn is_clear(&self) -> bool {
        self.0.is_clear()
    }

    fn __repr__(&self) -> String {
        format!("SeatNoCell({})", self.0)
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerNoCell>()?;
    m.add_class::<SeatNoCell>()?;
    Ok(())
}
