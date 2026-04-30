//! Bindings for pkcore's casino::table_no_cell module.

use pkcore::casino::table_no_cell::PlayerNoCell as PkPlayerNoCell;
use pkcore::casino::table_no_cell::SeatNoCell as PkSeatNoCell;
use pkcore::casino::table_no_cell::SeatsNoCell as PkSeatsNoCell;
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

/// A vector of `SeatNoCell` representing a table's seats.
#[pyclass(from_py_object, name = "SeatsNoCell")]
#[derive(Clone)]
pub struct SeatsNoCell(pub(crate) PkSeatsNoCell);

#[pymethods]
impl SeatsNoCell {
    #[new]
    fn new(seats: Vec<SeatNoCell>) -> Self {
        Self(PkSeatsNoCell::new(seats.into_iter().map(|s| s.0).collect()))
    }

    fn size(&self) -> u8 {
        self.0.size()
    }

    fn get_seat(&self, idx: u8) -> Option<SeatNoCell> {
        self.0.get_seat(idx).cloned().map(SeatNoCell)
    }

    fn is_seat_in_hand(&self, idx: u8) -> bool {
        self.0.is_seat_in_hand(idx)
    }

    fn current_bet(&self) -> usize {
        self.0.current_bet()
    }

    fn to_call(&self, player_idx: u8) -> usize {
        self.0.to_call(player_idx)
    }

    fn total_chip_count(&self) -> usize {
        self.0.total_chip_count()
    }

    fn count_active_in_hand(&self) -> usize {
        self.0.count_active_in_hand()
    }

    fn active_in_hand(&self) -> Vec<u8> {
        self.0.active_in_hand()
    }

    fn are_dealt(&self) -> bool {
        self.0.are_dealt()
    }

    fn are_clear(&self) -> bool {
        self.0.are_clear()
    }

    fn is_betting_complete(&self) -> bool {
        self.0.is_betting_complete()
    }

    fn __repr__(&self) -> String {
        format!("SeatsNoCell(size={})", self.0.size())
    }
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlayerNoCell>()?;
    m.add_class::<SeatNoCell>()?;
    m.add_class::<SeatsNoCell>()?;
    Ok(())
}
