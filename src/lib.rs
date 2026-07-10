use pkcore::analysis::case_evals::CaseEvals as PkCaseEvals;
use pkcore::analysis::class::HandRankClass as PkHandRankClass;
use pkcore::analysis::ev::Ev as PkEv;
use pkcore::analysis::eval::Eval as PkEval;
use pkcore::analysis::gto::combo::{Combo as PkCombo, Qualifier as PkQualifier};
use pkcore::analysis::gto::combo_pairs::ComboPairs as PkComboPairs;
use pkcore::analysis::gto::combos::Combos as PkCombos;
use pkcore::analysis::gto::odds::WinLoseDraw as PkWinLoseDraw;
use pkcore::analysis::gto::solver::Solver as PkSolver;
use pkcore::analysis::gto::solver_config::{
    BetSize as PkBetSize, BetSizings as PkBetSizings, SolverConfig as PkSolverConfig,
};
use pkcore::analysis::gto::strategy_profile::ActionFrequencies as PkActionFrequencies;
use pkcore::analysis::gto::twos::Twos as PkTwos;
use pkcore::analysis::gto::vs::Versus as PkVersus;
use pkcore::analysis::hand_rank::HandRank as PkHandRank;
use pkcore::analysis::nubibus::{Pluribus as PkPluribus, PluribusEvent as PkPluribusEvent};
use pkcore::analysis::outs::Outs as PkOuts;
use pkcore::analysis::pot_odds::PotOdds as PkPotOdds;
use pkcore::analysis::range_equity::RangeEquity as PkRangeEquity;
use pkcore::analysis::store::bcm::binary_card_map::SevenFiveBCM as PkSevenFiveBCM;
use pkcore::analysis::store::bcm::index_card_map::IndexCardMap as PkIndexCardMap;
use pkcore::analysis::store::db::hup::HUPResult as PkHUPResult;
use pkcore::arrays::two::Two as PkTwo;
use pkcore::bard::Bard as PkBard;
use pkcore::card::Card as PkCard;
use pkcore::cards::Cards as PkCards;
use pkcore::casino::action::TableAction as PkTableAction;
use pkcore::casino::cashier::chips::Stack as PkStack;
use pkcore::casino::dealer::{Dealer as PkDealer, DealerError as PkDealerError};
use pkcore::casino::equity::seat_equity::SeatEquity as PkSeatEquity;
use pkcore::casino::equity::seatbit::Seatbit as PkSeatbit;
use pkcore::casino::game::ForcedBets as PkForcedBets;
use pkcore::casino::player::Player as PkPlayer;
use pkcore::casino::state::PlayerState as PkPlayerState;
use pkcore::casino::table_celled::event::TableLog as PkTableLog;
use pkcore::casino::winnings::{PotWin as PkPotWin, Winnings as PkWinnings};
use pkcore::deck::Deck as PkDeck;
use pkcore::games::kuhn::{
    KuhnAction as PkKuhnAction, KuhnCard as PkKuhnCard, KuhnCfr as PkKuhnCfr,
    KuhnHistory as PkKuhnHistory, KuhnInfoSet as PkKuhnInfoSet, KuhnState as PkKuhnState,
    KuhnStrategy as PkKuhnStrategy,
};
use pkcore::play::board::Board as PkBoard;
use pkcore::play::game::Game as PkGame;
use pkcore::play::hole_cards::HoleCards as PkHoleCards;
use pkcore::play::stages::deal_eval::DealEval as PkDealEval;
use pkcore::play::stages::flop_eval::FlopEval as PkFlopEval;
use pkcore::play::stages::river_eval::RiverEval as PkRiverEval;
use pkcore::play::stages::turn_eval::TurnEval as PkTurnEval;
use pkcore::rank::Rank as PkRank;
use pkcore::ranks::Ranks as PkRanks;
use pkcore::suit::Suit as PkSuit;
use pkcore::util::Percentage as PkPercentage;
use pkcore::{Pile, GTO};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

mod bot;
mod hand_history;
mod session;
mod stats;
mod table_no_cell;

pub(crate) fn to_py_err(e: impl std::fmt::Display) -> PyErr {
    PyValueError::new_err(e.to_string())
}

// ============================================================
// Rank
// ============================================================

/// Represents the rank of a playing card (Ace through Deuce).
///
/// Examples:
///     >>> from pkcore import Rank
///     >>> r = Rank.ACE
///     >>> str(r)
///     'A'
#[pyclass(from_py_object, name = "Rank")]
#[derive(Clone)]
pub struct Rank(PkRank);

#[pymethods]
impl Rank {
    #[classattr]
    #[allow(non_snake_case)]
    fn ACE() -> Self {
        Rank(PkRank::ACE)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn KING() -> Self {
        Rank(PkRank::KING)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn QUEEN() -> Self {
        Rank(PkRank::QUEEN)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn JACK() -> Self {
        Rank(PkRank::JACK)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn TEN() -> Self {
        Rank(PkRank::TEN)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn NINE() -> Self {
        Rank(PkRank::NINE)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn EIGHT() -> Self {
        Rank(PkRank::EIGHT)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn SEVEN() -> Self {
        Rank(PkRank::SEVEN)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn SIX() -> Self {
        Rank(PkRank::SIX)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn FIVE() -> Self {
        Rank(PkRank::FIVE)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn FOUR() -> Self {
        Rank(PkRank::FOUR)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn TREY() -> Self {
        Rank(PkRank::TREY)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn DEUCE() -> Self {
        Rank(PkRank::DEUCE)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn BLANK() -> Self {
        Rank(PkRank::BLANK)
    }

    /// The integer value of this rank (Ace=14, King=13, ..., Deuce=2, Blank=0).
    fn value(&self) -> u8 {
        self.0 as u8
    }

    /// The prime number associated with this rank (used in Cactus Kev hand evaluation).
    fn prime(&self) -> u32 {
        self.0.prime()
    }

    /// The bit flag for this rank as used in the Cactus Kev binary card representation.
    fn bits(&self) -> u32 {
        self.0.bits()
    }

    /// The numeric index of this rank (0-based, Deuce=0, Ace=12).
    fn number(&self) -> u32 {
        self.0.number()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0.to_char())
    }

    fn __repr__(&self) -> String {
        format!("Rank.{:?}", self.0)
    }

    fn __eq__(&self, other: &Rank) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u8 {
        self.0 as u8
    }

    fn __lt__(&self, other: &Rank) -> bool {
        self.0 < other.0
    }

    fn __le__(&self, other: &Rank) -> bool {
        self.0 <= other.0
    }

    fn __gt__(&self, other: &Rank) -> bool {
        self.0 > other.0
    }

    fn __ge__(&self, other: &Rank) -> bool {
        self.0 >= other.0
    }
}

// ============================================================
// Suit
// ============================================================

/// Represents the suit of a playing card (Spades, Hearts, Diamonds, Clubs).
///
/// Examples:
///     >>> from pkcore import Suit
///     >>> s = Suit.SPADES
///     >>> str(s)
///     '♠'
#[pyclass(from_py_object, name = "Suit")]
#[derive(Clone)]
pub struct Suit(PkSuit);

#[pymethods]
impl Suit {
    #[classattr]
    #[allow(non_snake_case)]
    fn SPADES() -> Self {
        Suit(PkSuit::SPADES)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn HEARTS() -> Self {
        Suit(PkSuit::HEARTS)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn DIAMONDS() -> Self {
        Suit(PkSuit::DIAMONDS)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn CLUBS() -> Self {
        Suit(PkSuit::CLUBS)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn BLANK() -> Self {
        Suit(PkSuit::BLANK)
    }

    /// The integer value of this suit (Spades=4, Hearts=3, Diamonds=2, Clubs=1, Blank=0).
    fn value(&self) -> u8 {
        self.0 as u8
    }

    /// The Unicode symbol for this suit (♠, ♥, ♦, ♣).
    fn symbol(&self) -> String {
        format!("{}", self.0.to_char_symbol())
    }

    /// The letter abbreviation for this suit (S, H, D, C).
    fn letter(&self) -> String {
        format!("{}", self.0.to_char_letter())
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Suit.{:?}", self.0)
    }

    fn __eq__(&self, other: &Suit) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u8 {
        self.0 as u8
    }
}

// ============================================================
// Card
// ============================================================

/// A single playing card, represented internally using the Cactus Kev binary format.
///
/// Parse cards from strings like "As", "Kh", "Q♦", "2c".
///
/// Examples:
///     >>> from pkcore import Card
///     >>> c = Card.parse("As")
///     >>> str(c)
///     'A♠'
///     >>> c.rank().value()
///     14
#[pyclass(from_py_object, name = "Card")]
#[derive(Clone)]
pub struct Card(PkCard);

#[pymethods]
impl Card {
    /// Parse a card from a string such as "As", "Kh", "Q♦", "2c", "A♠".
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        PkCard::from_str(s).map(Card).map_err(to_py_err)
    }

    /// Create a card from a Rank and Suit.
    #[staticmethod]
    fn from_rank_suit(rank: &Rank, suit: &Suit) -> Self {
        Card(PkCard::new(rank.0, suit.0))
    }

    /// Returns the rank of this card.
    fn rank(&self) -> Rank {
        Rank(self.0.get_rank())
    }

    /// Returns the suit of this card.
    fn suit(&self) -> Suit {
        Suit(self.0.get_suit())
    }

    /// Returns True if this card has been dealt (is not a blank card).
    fn is_dealt(&self) -> bool {
        Pile::is_dealt(&self.0)
    }

    /// Returns the raw u32 Cactus Kev encoding of this card.
    fn as_u32(&self) -> u32 {
        self.0.as_u32()
    }

    /// Returns the binary string representation of this card's encoding.
    fn bit_string(&self) -> String {
        self.0.bit_string()
    }

    /// Returns the rank prime number of this card.
    fn get_rank_prime(&self) -> u32 {
        self.0.get_rank_prime()
    }

    /// Returns the letter-index representation of this card (e.g. "As").
    fn get_letter_index(&self) -> String {
        self.0.get_letter_index()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Card.parse('{}')", self.0)
    }

    fn __eq__(&self, other: &Card) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u32 {
        self.0.as_u32()
    }

    fn __lt__(&self, other: &Card) -> bool {
        self.0 < other.0
    }

    fn __le__(&self, other: &Card) -> bool {
        self.0 <= other.0
    }

    fn __gt__(&self, other: &Card) -> bool {
        self.0 > other.0
    }

    fn __ge__(&self, other: &Card) -> bool {
        self.0 >= other.0
    }
}

// ============================================================
// Cards
// ============================================================

/// An ordered, unique collection of playing cards.
///
/// Parse from a space-separated string of card abbreviations, e.g. "As Ks Qh".
///
/// Examples:
///     >>> from pkcore import Cards
///     >>> hand = Cards.parse("As Ks")
///     >>> len(hand)
///     2
///     >>> deck = Cards.deck()
///     >>> len(deck)
///     52
#[pyclass(from_py_object, name = "Cards")]
#[derive(Clone)]
pub struct Cards(PkCards);

#[pymethods]
impl Cards {
    /// Parse a Cards collection from a space-separated string of card abbreviations.
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        PkCards::from_str(s).map(Cards).map_err(to_py_err)
    }

    /// Returns a full 52-card deck.
    #[staticmethod]
    fn deck() -> Self {
        Cards(PkCards::deck())
    }

    /// Returns the number of cards in this collection.
    fn __len__(&self) -> usize {
        self.0.len()
    }

    /// Returns True if this collection contains the given card.
    fn contains(&self, card: &Card) -> bool {
        Pile::contains(&self.0, &card.0)
    }

    /// Returns all 52 deck cards not present in this collection.
    fn remaining(&self) -> Self {
        Cards(self.0.remaining())
    }

    /// Returns all deck cards not present in either this collection or the given other collection.
    fn remaining_after(&self, other: &Cards) -> Self {
        Cards(self.0.remaining_after(&other.0))
    }

    /// Returns True if all cards in this collection are valid (no blanks, all unique).
    fn is_dealt(&self) -> bool {
        Pile::is_dealt(&self.0)
    }

    /// Returns True if all cards in this collection are unique.
    fn are_unique(&self) -> bool {
        self.0.are_unique()
    }

    /// Returns all cards as a Python list.
    fn to_list(&self) -> Vec<Card> {
        self.0.to_vec().into_iter().map(Card).collect()
    }

    /// Returns True if this collection has no cards.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Append another Cards collection to this one (mutates in place).
    fn append(&mut self, other: &Cards) {
        self.0.append(&other.0)
    }

    /// Insert a card. Returns True if the card was not already present.
    fn insert(&mut self, card: &Card) -> bool {
        self.0.insert(card.0)
    }

    /// Remove a card. Returns True if the card was present.
    fn remove(&mut self, card: &Card) -> bool {
        self.0.remove(&card.0)
    }

    /// Return the card at the given index, or None.
    fn get_index(&self, index: usize) -> Option<Card> {
        self.0.get_index(index).map(|c| Card(*c))
    }

    /// Draw `n` cards from the top, returning them as a new Cards. Raises ValueError if not enough cards.
    fn draw(&mut self, n: usize) -> PyResult<Self> {
        self.0.draw(n).map(Cards).map_err(to_py_err)
    }

    /// Draw one card from the top. Raises ValueError if empty.
    fn draw_one(&mut self) -> PyResult<Card> {
        self.0.draw_one().map(Card).map_err(to_py_err)
    }

    /// Draw all remaining cards, leaving this collection empty.
    fn draw_all(&mut self) -> Self {
        Cards(self.0.draw_all())
    }

    /// Return a shuffled copy.
    fn shuffle(&self) -> Self {
        Cards(self.0.shuffle())
    }

    /// Shuffle this collection in place.
    fn shuffle_in_place(&mut self) {
        self.0.shuffle_in_place()
    }

    /// Return a sorted copy (highest rank first).
    fn sort(&self) -> Self {
        Cards(self.0.sort())
    }

    /// Return only cards of the given suit.
    fn filter_by_suit(&self, suit: &Suit) -> Self {
        Cards(self.0.filter_by_suit(suit.0))
    }

    /// Return this collection with the given cards removed.
    fn minus(&self, other: &Cards) -> Self {
        Cards(self.0.minus(&other.0))
    }

    /// Return all k-card combinations as a list of Cards objects.
    fn combinations(&self, k: usize) -> Vec<Cards> {
        self.0
            .combinations(k)
            .map(|combo| Cards(PkCards::from(combo)))
            .collect()
    }

    /// Return all 52 deck cards not in this collection.
    fn deck_minus(&self) -> Self {
        Cards(PkCards::deck_minus(&self.0))
    }

    /// Return a "primed" deck: this collection's cards first, then the rest of the deck.
    fn deck_primed(&self) -> Self {
        Cards(PkCards::deck_primed(&self.0))
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<CardsIterator>> {
        let cards: Vec<Card> = slf.0.to_vec().into_iter().map(Card).collect();
        let iter = CardsIterator {
            inner: cards.into_iter(),
        };
        Py::new(slf.py(), iter)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Cards.parse('{}')", self.0)
    }

    fn __eq__(&self, other: &Cards) -> bool {
        self.0 == other.0
    }
}

#[pyclass]
struct CardsIterator {
    inner: std::vec::IntoIter<Card>,
}

#[pymethods]
impl CardsIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Card> {
        slf.inner.next()
    }
}

// ============================================================
// HoleCards
// ============================================================

/// A collection of player hole cards (two-card hands) for one or more players.
///
/// Parse from a space-separated string where each pair of cards is one player's hand.
/// For example, "As Ks Ah Kh" gives two players: player 1 has A♠ K♠, player 2 has A♥ K♥.
///
/// Examples:
///     >>> from pkcore import HoleCards
///     >>> hc = HoleCards.parse("As Ks Ah Kh")
///     >>> len(hc)
///     2
#[pyclass(from_py_object, name = "HoleCards")]
#[derive(Clone)]
pub struct HoleCards(PkHoleCards);

#[pymethods]
impl HoleCards {
    /// Parse hole cards from a space-separated string.
    /// Cards are grouped in pairs: first two cards are player 1, next two are player 2, etc.
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        PkHoleCards::from_str(s).map(HoleCards).map_err(to_py_err)
    }

    /// Returns the number of players (two-card hands) in this collection.
    fn __len__(&self) -> usize {
        self.0.len()
    }

    /// Returns True if this collection has no hole cards.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the Two at the given index (0-based), or None.
    fn get(&self, index: usize) -> Option<Two> {
        self.0.get(index).map(|t| Two(t.clone()))
    }

    /// Append a two-card hand to this collection.
    fn push(&mut self, two: &Two) {
        self.0.push(two.0.clone())
    }

    /// Returns all hole card hands as a Python list of Two objects.
    fn to_list(&self) -> Vec<Two> {
        self.0.iter().map(|t| Two(t.clone())).collect()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("HoleCards.parse('{}')", self.0)
    }
}

// ============================================================
// Board
// ============================================================

/// The community cards on the board (flop, turn, river).
///
/// Parse from a space-separated string of up to 5 cards.
///
/// Examples:
///     >>> from pkcore import Board
///     >>> board = Board.parse("As Ks Qh Jd Tc")
///     >>> str(board)
///     'FLOP: A♠ K♠ Q♥, TURN: J♦, RIVER: T♣'
#[pyclass(from_py_object, name = "Board")]
#[derive(Clone)]
pub struct Board(PkBoard);

#[pymethods]
impl Board {
    /// Parse a board from a space-separated string of card abbreviations (3-5 cards).
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        PkBoard::from_str(s).map(Board).map_err(to_py_err)
    }

    /// Returns the cards visible at and before the turn (flop + turn).
    fn turn_cards(&self) -> Cards {
        Cards(self.0.turn_cards())
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Board.parse('{}')", self.0)
    }
}

// ============================================================
// HandRankClass
// ============================================================

/// The general category of a five-card poker hand.
///
/// Examples:
///     >>> from pkcore import HandRankClass
///     >>> str(HandRankClass.ROYAL_FLUSH)
///     'RoyalFlush'
#[pyclass(from_py_object, name = "HandRankClass")]
#[derive(Clone)]
pub struct HandRankClass(PkHandRankClass);

#[pymethods]
impl HandRankClass {
    #[classattr]
    #[allow(non_snake_case)]
    fn ROYAL_FLUSH() -> Self {
        HandRankClass(PkHandRankClass::RoyalFlush)
    }

    /// Returns True if this hand is any kind of straight flush (including royal flush).
    fn is_straight_flush(&self) -> bool {
        matches!(
            self.0,
            PkHandRankClass::RoyalFlush
                | PkHandRankClass::KingHighStraightFlush
                | PkHandRankClass::QueenHighStraightFlush
                | PkHandRankClass::JackHighStraightFlush
                | PkHandRankClass::TenHighStraightFlush
                | PkHandRankClass::NineHighStraightFlush
                | PkHandRankClass::EightHighStraightFlush
                | PkHandRankClass::SevenHighStraightFlush
                | PkHandRankClass::SixHighStraightFlush
                | PkHandRankClass::FiveHighStraightFlush
        )
    }

    /// Returns True if this hand is four of a kind.
    fn is_four_of_a_kind(&self) -> bool {
        matches!(
            self.0,
            PkHandRankClass::FourAces
                | PkHandRankClass::FourKings
                | PkHandRankClass::FourQueens
                | PkHandRankClass::FourJacks
                | PkHandRankClass::FourTens
                | PkHandRankClass::FourNines
                | PkHandRankClass::FourEights
                | PkHandRankClass::FourSevens
                | PkHandRankClass::FourSixes
                | PkHandRankClass::FourFives
                | PkHandRankClass::FourFours
                | PkHandRankClass::FourTreys
                | PkHandRankClass::FourDeuces
        )
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("HandRankClass('{:?}')", self.0)
    }

    fn __eq__(&self, other: &HandRankClass) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> usize {
        self.0 as usize
    }

    fn __lt__(&self, other: &HandRankClass) -> bool {
        self.0 < other.0
    }

    fn __le__(&self, other: &HandRankClass) -> bool {
        self.0 <= other.0
    }

    fn __gt__(&self, other: &HandRankClass) -> bool {
        self.0 > other.0
    }

    fn __ge__(&self, other: &HandRankClass) -> bool {
        self.0 >= other.0
    }
}

// ============================================================
// HandRank
// ============================================================

/// The rank of a specific five-card hand, including its numeric value and class.
///
/// Lower `value` means a stronger hand (1 = best possible hand, royal flush).
///
/// Examples:
///     >>> # HandRank is obtained from an Eval, not constructed directly.
///     >>> from pkcore import Eval
#[pyclass(from_py_object, name = "HandRank")]
#[derive(Clone)]
pub struct HandRank(PkHandRank);

#[pymethods]
impl HandRank {
    /// The numeric rank value. Lower is better (1 = royal flush).
    #[getter]
    fn value(&self) -> u16 {
        self.0.value
    }

    /// The detailed class of this hand (e.g., RoyalFlush, FourAces, etc.).
    #[getter]
    fn class(&self) -> HandRankClass {
        HandRankClass(self.0.class)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("HandRank(value={}, class={:?})", self.0.value, self.0.class)
    }

    fn __eq__(&self, other: &HandRank) -> bool {
        self.0 == other.0
    }
}

// ============================================================
// Eval
// ============================================================

/// An evaluation of a single five-card poker hand.
///
/// Contains the best five-card hand found and its rank.
///
/// Examples:
///     >>> from pkcore import Cards, Eval
///     >>> # Typically obtained via Game.turn_case_evals() rather than constructed directly.
#[pyclass(from_py_object, name = "Eval")]
#[derive(Clone)]
pub struct Eval(PkEval);

#[pymethods]
impl Eval {
    /// The hand rank of this evaluation.
    #[getter]
    fn hand_rank(&self) -> HandRank {
        HandRank(self.0.hand_rank)
    }

    /// The class of this hand (e.g., 'RoyalFlush', 'FullHouse', etc.).
    fn hand_class(&self) -> HandRankClass {
        HandRankClass(self.0.hand_rank.class)
    }

    /// The numeric rank value (lower = stronger hand).
    fn rank_value(&self) -> u16 {
        self.0.hand_rank.value
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!(
            "Eval(rank_value={}, class={:?})",
            self.0.hand_rank.value, self.0.hand_rank.class
        )
    }
}

// ============================================================
// CaseEvals
// ============================================================

/// A collection of case evaluations, one per possible river card.
///
/// Obtained from `Game.turn_case_evals()`. Used to calculate outs.
///
/// Examples:
///     >>> from pkcore import HoleCards, Board, Game
///     >>> hc = HoleCards.parse("As Kh 8d Kc")
///     >>> board = Board.parse("Ac 8h 7h 9s")
///     >>> game = Game(hc, board)
///     >>> case_evals = game.turn_case_evals()
///     >>> len(case_evals)
///     46
#[pyclass(from_py_object, name = "CaseEvals")]
#[derive(Clone)]
pub struct CaseEvals(PkCaseEvals);

#[pymethods]
impl CaseEvals {
    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __repr__(&self) -> String {
        format!("CaseEvals(len={})", self.0.len())
    }
}

// ============================================================
// Outs
// ============================================================

/// The outs for each player — cards that, if dealt on the river, cause that player to win.
///
/// Created from a `CaseEvals` instance.
///
/// Examples:
///     >>> from pkcore import HoleCards, Board, Game, Outs
///     >>> hc = HoleCards.parse("As Kh 8d Kc")
///     >>> board = Board.parse("Ac 8h 7h 9s")
///     >>> game = Game(hc, board)
///     >>> outs = Outs.from_case_evals(game.turn_case_evals())
///     >>> outs.len_for_player(1)
///     1
#[pyclass(from_py_object, name = "Outs")]
#[derive(Clone)]
pub struct Outs(PkOuts);

#[pymethods]
impl Outs {
    /// Build an Outs from a CaseEvals collection.
    #[staticmethod]
    fn from_case_evals(case_evals: &CaseEvals) -> Self {
        Outs(PkOuts::from(&case_evals.0))
    }

    /// Returns the cards that are outs for the given player (1-indexed).
    /// Returns None if the player has no outs.
    fn get(&self, player: usize) -> Option<Cards> {
        self.0.get(player).map(|c| Cards(c.clone()))
    }

    /// Returns the number of outs for the given player (1-indexed).
    fn len_for_player(&self, player: usize) -> usize {
        self.0.len_for_player(player)
    }

    /// Returns the number of outs for the player with the most outs.
    fn len_longest(&self) -> usize {
        self.0.len_longest()
    }

    /// Returns the player id (1-indexed) with the most outs.
    fn longest_player(&self) -> usize {
        self.0.longest_player()
    }

    /// Returns True if the given player has the most outs.
    fn is_longest(&self, player: usize) -> bool {
        self.0.is_longest(player)
    }

    fn __repr__(&self) -> String {
        format!(
            "Outs(longest_player={}, len_longest={})",
            self.0.longest_player(),
            self.0.len_longest()
        )
    }
}

// ============================================================
// Game
// ============================================================

/// A Texas Hold'em game combining hole cards and a board.
///
/// Use `turn_case_evals()` to calculate all possible river outcomes.
///
/// Examples:
///     >>> from pkcore import HoleCards, Board, Game, Outs
///     >>> hc = HoleCards.parse("As Kh 8d Kc")
///     >>> board = Board.parse("Ac 8h 7h 9s")
///     >>> game = Game(hc, board)
///     >>> outs = Outs.from_case_evals(game.turn_case_evals())
///     >>> outs.longest_player()
///     2
#[pyclass(skip_from_py_object, name = "Game")]
pub struct Game(PkGame);

#[pymethods]
impl Game {
    /// Create a new Game from HoleCards and a Board.
    #[new]
    fn new(hole_cards: &HoleCards, board: &Board) -> Self {
        Game(PkGame::new(hole_cards.0.clone(), board.0))
    }

    /// Calculate case evaluations for all possible river cards at the turn.
    ///
    /// Returns a CaseEvals with one entry per possible river card.
    fn turn_case_evals(&self) -> CaseEvals {
        CaseEvals(self.0.turn_case_evals())
    }

    /// Evaluate all players' best hands and win percentages at the flop.
    ///
    /// Returns None if the board does not have a flop.
    fn flop_eval(&self) -> Option<FlopEval> {
        PkFlopEval::try_from(self.0.clone()).ok().map(FlopEval)
    }

    /// Evaluate all players' best hands, win percentages, and outs at the turn.
    ///
    /// Returns None if the board does not have a turn card.
    fn turn_eval(&self) -> Option<TurnEval> {
        PkTurnEval::try_from(&self.0).ok().map(TurnEval)
    }

    /// Returns the formatted "Nuts @ Turn" string — all distinct best hands
    /// possible at the turn, sorted by strength.
    ///
    /// Returns an empty string if the board does not have a turn card.
    fn turn_nuts_display(&self) -> String {
        format!("{}", self.0.turn_the_nuts().to_evals())
    }

    /// True if the board has a turn card dealt.
    fn has_dealt_turn(&self) -> bool {
        self.0.has_dealt_turn()
    }

    /// Returns the best-hand Eval for a specific player at the turn (0-indexed).
    /// Raises ValueError if the board has no turn card or the index is out of range.
    fn turn_eval_for_player(&self, index: usize) -> PyResult<Eval> {
        self.0
            .turn_eval_for_player(index)
            .map(Eval)
            .map_err(to_py_err)
    }

    /// Returns the remaining deck cards (cards not on the board or in any hand) at the turn.
    fn turn_remaining_board(&self) -> Cards {
        Cards(self.0.turn_remaining_board())
    }

    /// Returns the flop and turn cards (4 cards) as a Cards collection.
    /// Returns an empty Cards if the board does not have a turn card.
    fn flop_and_turn(&self) -> Cards {
        let four = self.0.flop_and_turn();
        Cards(PkCards::from(four.to_arr()))
    }

    /// Returns the formatted river result string showing each player's final
    /// hand and the winner.
    ///
    /// Returns an empty string if the board does not have a river card.
    fn river_display(&self) -> String {
        match self.0.river_case_eval() {
            Err(_) => String::new(),
            Ok(case_eval) => {
                let winning_hand_rank = case_eval.winning_hand_rank();
                let mut out = format!(
                    "\nThe River: {} {} {}\n",
                    self.0.board.flop, self.0.board.turn, self.0.board.river
                );
                out.push_str(&format!(" Winning Hand: {winning_hand_rank}\n"));
                for (i, eval) in case_eval.iter().enumerate() {
                    if eval.hand_rank == winning_hand_rank {
                        out.push_str(&format!("   Player #{i}: {eval} has the best hand!\n"));
                    } else if eval.hand_rank.value > 0 {
                        out.push_str(&format!("   Player #{i}: {eval}\n"));
                    }
                }
                out
            }
        }
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        "Game(hole_cards=..., board=...)".to_string()
    }
}

// ============================================================
// FlopEval
// ============================================================

/// Per-player hand evaluation and win percentages at the flop.
///
/// Obtain via `game.flop_eval()`. Printing a FlopEval shows each player's
/// best hand, win percentage, and the board state.
///
/// Examples:
///     >>> from pkpy import HoleCards, Board, Game
///     >>> hc = HoleCards.parse("6s 6h 5d 5c")
///     >>> board = Board.parse("9c 6d 5h")
///     >>> game = Game(hc, board)
///     >>> flop = game.flop_eval()
///     >>> print(flop)
#[pyclass(from_py_object, name = "FlopEval")]
#[derive(Clone)]
pub struct FlopEval(PkFlopEval);

#[pymethods]
impl FlopEval {
    /// Convert to a WinLoseDraw summary of wins, losses, and draws for player 0.
    fn to_win_lose_draw(&self) -> WinLoseDraw {
        WinLoseDraw(PkWinLoseDraw::from(self.0.clone()))
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        "FlopEval(...)".to_string()
    }
}

// ============================================================
// TurnEval
// ============================================================

/// Per-player hand evaluation, win percentages, and outs at the turn.
///
/// Obtain via `game.turn_eval()`. Printing a TurnEval shows each player's
/// best hand, win percentage, and outs going into the river.
///
/// Examples:
///     >>> from pkpy import HoleCards, Board, Game
///     >>> hc = HoleCards.parse("6s 6h 5d 5c")
///     >>> board = Board.parse("9c 6d 5h 5s")
///     >>> game = Game(hc, board)
///     >>> turn = game.turn_eval()
///     >>> print(turn)
#[pyclass(from_py_object, name = "TurnEval")]
#[derive(Clone)]
pub struct TurnEval(PkTurnEval);

#[pymethods]
impl TurnEval {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        "TurnEval(...)".to_string()
    }
}

// ============================================================
// Qualifier
// ============================================================

/// The suit qualifier for a hand combo: suited, offsuit, or both.
#[pyclass(from_py_object, name = "Qualifier")]
#[derive(Clone)]
pub struct Qualifier(PkQualifier);

#[pymethods]
impl Qualifier {
    #[classattr]
    #[allow(non_snake_case)]
    fn SUITED() -> Self {
        Qualifier(PkQualifier::SUITED)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn OFFSUIT() -> Self {
        Qualifier(PkQualifier::OFFSUIT)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn ALL() -> Self {
        Qualifier(PkQualifier::ALL)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Qualifier.{:?}", self.0)
    }

    fn __eq__(&self, other: &Qualifier) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> usize {
        match self.0 {
            PkQualifier::OFFSUIT => 0,
            PkQualifier::SUITED => 1,
            PkQualifier::ALL => 2,
        }
    }
}

// ============================================================
// Combo
// ============================================================

/// An abstract poker hand combination such as "AKs", "JJ+", or "QQ".
///
/// A Combo represents a category of two-card hands defined by rank(s) and a suit
/// qualifier. Use `Combos.explode()` to expand a range into all concrete `Two` hands.
///
/// Examples:
///     >>> from pkpy import Combo
///     >>> c = Combo.parse("AKs")
///     >>> c.is_suited()
///     True
///     >>> c.total_pairs()
///     4
#[pyclass(from_py_object, name = "Combo")]
#[derive(Clone)]
pub struct Combo(PkCombo);

#[pymethods]
impl Combo {
    /// Parse a combo from a string such as "AKs", "JJ+", "QQ", "AJo".
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        PkCombo::from_str(s).map(Combo).map_err(to_py_err)
    }

    /// The higher rank of this combo.
    #[getter]
    fn first(&self) -> Rank {
        Rank(self.0.first)
    }

    /// The lower rank of this combo.
    #[getter]
    fn second(&self) -> Rank {
        Rank(self.0.second)
    }

    /// True if this combo has a "+" suffix (e.g., "JJ+").
    #[getter]
    fn plus(&self) -> bool {
        self.0.plus
    }

    /// The suit qualifier: SUITED, OFFSUIT, or ALL.
    #[getter]
    fn qualifier(&self) -> Qualifier {
        Qualifier(self.0.qualifier)
    }

    /// True if this combo is a pocket pair (both ranks equal).
    fn is_pair(&self) -> bool {
        self.0.is_pair()
    }

    /// True if this combo requires both cards to be suited.
    fn is_suited(&self) -> bool {
        self.0.is_suited()
    }

    /// True if this combo requires both cards to be offsuit.
    fn is_offsuit(&self) -> bool {
        self.0.is_offsuit()
    }

    /// True if the higher rank is an Ace (e.g., AK, AQ, A2).
    fn is_ace_x(&self) -> bool {
        self.0.is_ace_x()
    }

    /// True if this is a suited Ace-X combo.
    fn is_ace_x_suited(&self) -> bool {
        self.0.is_ace_x_suited()
    }

    /// True if this is an offsuit Ace-X combo.
    fn is_ace_x_offsuit(&self) -> bool {
        self.0.is_ace_x_offsuit()
    }

    /// True if the two ranks are consecutive (e.g., KQ, JT, 54).
    fn is_connector(&self) -> bool {
        self.0.is_connector()
    }

    /// True if this is a suited connector.
    fn is_suited_connector(&self) -> bool {
        self.0.is_suited_connector()
    }

    /// True if this is an offsuit connector.
    fn is_offsuit_connector(&self) -> bool {
        self.0.is_offsuit_connector()
    }

    /// Number of concrete two-card hands this combo represents.
    /// Pairs → 6, suited → 4, offsuit → 12, all → 16.
    fn total_pairs(&self) -> usize {
        self.0.total_pairs()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Combo.parse('{}')", self.0)
    }

    fn __eq__(&self, other: &Combo) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }
}

// ============================================================
// Two
// ============================================================

/// A specific two-card poker hand (e.g., "As Kh").
///
/// `Two` is the concrete unit produced by combo explosion. Each `Two` holds
/// exactly two distinct cards and can be inspected for pairing, suitedness, and
/// card membership.
///
/// Examples:
///     >>> from pkpy import Two
///     >>> t = Two.parse("As Kh")
///     >>> t.is_suited()
///     False
///     >>> str(t.first())
///     'A♠'
#[pyclass(from_py_object, name = "Two")]
#[derive(Clone)]
pub struct Two(PkTwo);

#[pymethods]
impl Two {
    /// Parse a two-card hand from a space-separated string such as "As Kh".
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        PkTwo::from_str(s).map(Two).map_err(to_py_err)
    }

    /// The first (higher) card.
    fn first(&self) -> Card {
        Card(self.0.first())
    }

    /// The second (lower) card.
    fn second(&self) -> Card {
        Card(self.0.second())
    }

    /// True if both cards share the same rank (pocket pair).
    fn is_pair(&self) -> bool {
        self.0.is_pair()
    }

    /// True if both cards share the same suit.
    fn is_suited(&self) -> bool {
        self.0.is_suited()
    }

    /// True if either card has the given rank.
    fn contains_rank(&self, rank: &Rank) -> bool {
        self.0.contains_rank(rank.0)
    }

    /// True if either card has the given suit.
    fn contains_suit(&self, suit: &Suit) -> bool {
        self.0.contains_suit(suit.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Two.parse('{}')", self.0)
    }

    fn __eq__(&self, other: &Two) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }
}

// ============================================================
// Twos
// ============================================================

/// A collection of concrete two-card hands produced by combo explosion.
///
/// Obtained by calling `Combos.explode()`. Can be filtered by card, rank,
/// suit, pairing, or suitedness.
///
/// Examples:
///     >>> from pkpy import Combos
///     >>> twos = Combos.parse("QQ+, AK").explode()
///     >>> len(twos)
///     30
#[pyclass(from_py_object, name = "Twos")]
#[derive(Clone)]
pub struct Twos(PkTwos);

#[pymethods]
impl Twos {
    /// Number of hands in this collection.
    fn __len__(&self) -> usize {
        self.0.len()
    }

    /// True if this collection contains no hands.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// All hands as a Python list, sorted high to low.
    fn to_list(&self) -> Vec<Two> {
        self.0.to_vec().into_iter().map(Two).collect()
    }

    /// True if the given Two is in this collection.
    fn contains(&self, two: &Two) -> bool {
        self.0.contains(&two.0)
    }

    /// Return only hands that contain the given card.
    fn filter_on_card(&self, card: &Card) -> Self {
        Twos(self.0.filter_on_card(card.0))
    }

    /// Return only pocket pairs.
    fn filter_is_paired(&self) -> Self {
        Twos(self.0.filter_is_paired())
    }

    /// Return only non-paired hands.
    fn filter_is_not_paired(&self) -> Self {
        Twos(self.0.filter_is_not_paired())
    }

    /// Return only suited hands.
    fn filter_is_suited(&self) -> Self {
        Twos(self.0.filter_is_suited())
    }

    /// Return only offsuit hands.
    fn filter_is_not_suited(&self) -> Self {
        Twos(self.0.filter_is_not_suited())
    }

    /// Return only hands that contain the given rank.
    fn filter_on_rank(&self, rank: &Rank) -> Self {
        Twos(self.0.filter_on_rank(rank.0))
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Twos(len={})", self.0.len())
    }
}

// ============================================================
// Combos
// ============================================================

/// A range of abstract hand combinations (e.g., "QQ+, AK", "66+,AJs+,KQs").
///
/// Parse a range from a standard poker range string, then call `explode()` to
/// get all concrete two-card hands the range represents.
///
/// Predefined range strings are available as class attributes:
///     - `Combos.PERCENT_2_5`  — top ~2.5% of hands ("QQ+, AK")
///     - `Combos.PERCENT_5`    — top ~5%
///     - `Combos.PERCENT_10`   — top ~10%
///     - `Combos.PERCENT_20`   — top ~20%
///     - `Combos.PERCENT_33`   — top ~33%
///
/// Examples:
///     >>> from pkpy import Combos
///     >>> r = Combos.parse("QQ+, AK")
///     >>> len(r)
///     5
///     >>> twos = r.explode()
///     >>> len(twos)
///     30
#[pyclass(from_py_object, name = "Combos")]
#[derive(Clone)]
pub struct Combos(PkCombos);

#[pymethods]
impl Combos {
    /// Parse a range from a standard poker range string.
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        PkCombos::from_str(s).map(Combos).map_err(to_py_err)
    }

    /// Predefined range string for the top ~2.5% of hands.
    #[classattr]
    #[allow(non_snake_case)]
    fn PERCENT_2_5() -> &'static str {
        PkCombos::PERCENT_2_5
    }

    /// Predefined range string for the top ~5% of hands.
    #[classattr]
    #[allow(non_snake_case)]
    fn PERCENT_5() -> &'static str {
        PkCombos::PERCENT_5
    }

    /// Predefined range string for the top ~10% of hands.
    #[classattr]
    #[allow(non_snake_case)]
    fn PERCENT_10() -> &'static str {
        PkCombos::PERCENT_10
    }

    /// Predefined range string for the top ~20% of hands.
    #[classattr]
    #[allow(non_snake_case)]
    fn PERCENT_20() -> &'static str {
        PkCombos::PERCENT_20
    }

    /// Predefined range string for the top ~33% of hands.
    #[classattr]
    #[allow(non_snake_case)]
    fn PERCENT_33() -> &'static str {
        PkCombos::PERCENT_33
    }

    /// Expand this range into all concrete two-card hands it represents.
    fn explode(&self) -> Twos {
        Twos(self.0.explode())
    }

    /// All abstract combos in this range as a Python list, sorted high to low.
    fn to_list(&self) -> Vec<Combo> {
        self.0.to_vec().into_iter().map(Combo).collect()
    }

    /// Number of abstract combos in this range.
    fn __len__(&self) -> usize {
        self.0.len()
    }

    /// True if this range contains no combos.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Group the concrete hands in this range by their abstract combo pattern.
    fn combo_pairs(&self) -> ComboPairs {
        ComboPairs(self.0.combo_pairs())
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Combos.parse('{}')", self.0)
    }
}

// ============================================================
// WinLoseDraw
// ============================================================

/// Aggregate win, loss, and draw counts for a heads-up matchup.
///
/// Examples:
///     >>> from pkpy import Versus, Two, Combos
///     >>> solver = Versus(Two.parse("K♠ K♥"), Combos.parse("66+,AJs+,KQs,AJo+,KQo"))
///     >>> hups = solver.hups_at_deal()
///     >>> results = Versus.combined_odds_at_deal(hups)
///     >>> results.win_percentage()
#[pyclass(from_py_object, name = "WinLoseDraw")]
#[derive(Clone)]
pub struct WinLoseDraw(PkWinLoseDraw);

#[pymethods]
impl WinLoseDraw {
    /// Total number of outcomes (wins + losses + draws).
    fn total(&self) -> u64 {
        self.0.total()
    }

    /// Fraction of outcomes that are wins, as a percentage (0–100).
    fn win_percentage(&self) -> f32 {
        self.0.win_percentage()
    }

    /// Fraction of outcomes that are losses, as a percentage (0–100).
    fn loss_percentage(&self) -> f32 {
        self.0.loss_percentage()
    }

    /// Fraction of outcomes that are draws, as a percentage (0–100).
    fn draw_percentage(&self) -> f32 {
        self.0.draw_percentage()
    }

    #[getter]
    fn wins(&self) -> u64 {
        self.0.wins
    }

    #[getter]
    fn losses(&self) -> u64 {
        self.0.losses
    }

    #[getter]
    fn draws(&self) -> u64 {
        self.0.draws
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!(
            "WinLoseDraw(wins={}, losses={}, draws={})",
            self.0.wins, self.0.losses, self.0.draws
        )
    }
}

// ============================================================
// HUPResult
// ============================================================

/// Preflop heads-up odds for a specific hero vs. villain hand matchup.
///
/// Returned as elements of the list from `Versus.hups_at_deal()`. Each
/// HUPResult shows both hands and their win/loss/draw percentages.
#[pyclass(from_py_object, name = "HUPResult")]
#[derive(Clone)]
pub struct HUPResult(PkHUPResult);

#[pymethods]
impl HUPResult {
    /// The win/loss/draw breakdown for this matchup.
    #[getter]
    fn odds(&self) -> WinLoseDraw {
        WinLoseDraw(self.0.odds)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        "HUPResult(...)".to_string()
    }
}

// ============================================================
// ComboPairs
// ============================================================

/// A range grouped by abstract combo pattern, showing how many of each
/// combo remain after accounting for blockers.
///
/// Obtained from `Combos.combo_pairs()` or `Versus.combo_pairs()`.
#[pyclass(from_py_object, name = "ComboPairs")]
#[derive(Clone)]
pub struct ComboPairs(PkComboPairs);

#[pymethods]
impl ComboPairs {
    /// The combos in this collection, sorted high to low.
    fn key_vec(&self) -> Vec<Combo> {
        self.0.key_vec().into_iter().map(Combo).collect()
    }

    /// The concrete two-card hands for a specific combo, or None if absent.
    fn twos_for_combo(&self, combo: &Combo) -> Option<Twos> {
        self.0.twos_for_combo(&combo.0).map(|t| Twos(t.clone()))
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        "ComboPairs(...)".to_string()
    }
}

// ============================================================
// Versus
// ============================================================

/// A heads-up matchup between a specific hero hand and a villain range.
///
/// Core type for GTO range analysis. Computes preflop equity, removes
/// blockers from the villain range, and (if a board is provided) evaluates
/// postflop equity across all possible villain hands.
///
/// Examples:
///     >>> from pkpy import Versus, Two, Combos
///     >>> hero = Two.parse("K♠ K♥")
///     >>> villain = Combos.parse("66+,AJs+,KQs,AJo+,KQo")
///     >>> solver = Versus(hero, villain)
///     >>> hups = solver.hups_at_deal()
///     >>> results = Versus.combined_odds_at_deal(hups)
///     >>> print(results)
#[pyclass(skip_from_py_object, name = "Versus")]
pub struct Versus(PkVersus);

#[pymethods]
impl Versus {
    /// Create a preflop matchup between a hero hand and a villain range.
    #[new]
    fn new(hero: &Two, villain: &Combos) -> Self {
        Versus(PkVersus::new(hero.0, villain.0.clone()))
    }

    /// Create a postflop matchup with a board.
    #[staticmethod]
    fn with_board(hero: &Two, villain: &Combos, board: &Board) -> Self {
        Versus(PkVersus::new_with_board(hero.0, villain.0.clone(), board.0))
    }

    /// The hero's specific two-card hand.
    #[getter]
    fn hero(&self) -> Two {
        Two(self.0.hero)
    }

    /// The villain's full abstract range (before blockers).
    #[getter]
    fn villain(&self) -> Combos {
        Combos(self.0.villain.clone())
    }

    /// True if this solver has a board (postflop scenario).
    fn has_board(&self) -> bool {
        self.0.has_board()
    }

    /// Villain's remaining concrete hands after removing hero's blockers.
    fn remaining(&self) -> Twos {
        Twos(self.0.remaining())
    }

    /// Villain's remaining hands grouped by abstract combo (after blockers).
    fn combo_pairs(&self) -> ComboPairs {
        ComboPairs(self.0.combo_pairs())
    }

    /// Preflop heads-up odds for every villain hand vs. the hero.
    ///
    /// Returns a list of HUPResult objects, one per villain hand.
    fn hups_at_deal(&self) -> Vec<HUPResult> {
        self.0
            .hups_at_deal()
            .unwrap()
            .into_values()
            .map(HUPResult)
            .collect()
    }

    /// Aggregate preflop equity across all villain hands.
    ///
    /// Pass the list returned by `hups_at_deal()`.
    #[staticmethod]
    fn combined_odds_at_deal(hups: Vec<HUPResult>) -> WinLoseDraw {
        let pk: Vec<PkHUPResult> = hups.iter().map(|h| h.0).collect();
        let refs: Vec<&PkHUPResult> = pk.iter().collect();
        WinLoseDraw(PkVersus::combined_odds_at_deal(&refs))
    }

    /// Aggregate equity at the flop across all villain hands (requires board).
    fn combined_odds_at_flop(&self) -> WinLoseDraw {
        WinLoseDraw(self.0.combined_odds_at_flop())
    }

    /// Aggregate equity at the turn across all villain hands (requires board).
    fn combined_odds_at_turn(&self) -> WinLoseDraw {
        WinLoseDraw(self.0.combined_odds_at_turn())
    }

    /// All Game objects for each villain hand at the flop (requires board).
    fn games_at_flop(&self) -> Vec<Game> {
        self.0.games_at_flop().into_iter().map(Game).collect()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        "Versus(...)".to_string()
    }
}

// ============================================================
// PluribusEvent
// ============================================================

/// A single action in a Pluribus hand log: Fold, Call, or Raise(amount).
///
/// Examples:
///     >>> from pkcore import Pluribus
///     >>> hand = Pluribus.parse("STATE:0:ffr225fff:3c9s|6d5s|9dTs|2sQs|AdKd|7cTc:-50|-100|0|0|150|0:MrWhite|Gogo|Budd|Eddie|Bill|Pluribus")
///     >>> for event in hand.actions():
///     ...     print(event)
#[pyclass(from_py_object, name = "PluribusEvent")]
#[derive(Clone)]
pub struct PluribusEvent(PkPluribusEvent);

#[pymethods]
impl PluribusEvent {
    /// True if this event is a fold.
    fn is_fold(&self) -> bool {
        self.0.is_fold()
    }

    /// True if this event is a call.
    fn is_call(&self) -> bool {
        self.0.is_call()
    }

    /// True if this event is a raise.
    fn is_raise(&self) -> bool {
        self.0.is_raise()
    }

    /// The raise amount, or None if this is not a raise.
    fn raise_amount(&self) -> Option<usize> {
        self.0.raise_amount()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("PluribusEvent('{}')", self.0)
    }

    fn __eq__(&self, other: &PluribusEvent) -> bool {
        self.0 == other.0
    }
}

// ============================================================
// Pluribus
// ============================================================

/// A single hand record from a Pluribus AI poker log.
///
/// Pluribus logs use the format:
///   `STATE:{index}:{rounds}:{cards}:{winnings}:{players}`
///
/// Cards are pipe-separated two-card hands, optionally followed by `/board`.
/// Rounds are slash-separated action strings where `f`=fold, `c`=call, `r{n}`=raise.
///
/// Examples:
///     >>> from pkcore import Pluribus
///     >>> log = "STATE:27:r200ffcfc/cr850cf:Qc4h|Tc9c|8sAs/3h7s5c:-50|-200|250:Alice|Bob|Pluribus"
///     >>> hand = Pluribus.parse(log)
///     >>> hand.index
///     27
///     >>> hand.players
///     ['Alice', 'Bob', 'Pluribus']
///     >>> hand.board
///     Board.parse('3♥ 7♠ 5♣')
#[pyclass(from_py_object, name = "Pluribus")]
#[derive(Clone)]
pub struct Pluribus(PkPluribus);

#[pymethods]
impl Pluribus {
    /// Parse a single Pluribus log line.
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        s.parse::<PkPluribus>().map(Pluribus).map_err(to_py_err)
    }

    /// Parse all valid Pluribus hands from a log file. Invalid lines are skipped.
    #[staticmethod]
    fn read_log(filename: &str) -> PyResult<Vec<Pluribus>> {
        PkPluribus::read_in_log(filename)
            .map(|v| v.into_iter().map(Pluribus).collect())
            .map_err(to_py_err)
    }

    /// The hand index number from the log.
    #[getter]
    fn index(&self) -> usize {
        self.0.index
    }

    /// The raw log line this hand was parsed from.
    #[getter]
    fn raw(&self) -> String {
        self.0.raw.clone()
    }

    /// The hole cards dealt to each player.
    #[getter]
    fn hole_cards(&self) -> HoleCards {
        HoleCards(self.0.hole_cards.clone())
    }

    /// The community board cards (may be empty for preflop-only hands).
    #[getter]
    fn board(&self) -> Board {
        Board(self.0.board)
    }

    /// The winnings/losses for each player (positive = won, negative = lost).
    #[getter]
    fn winnings(&self) -> Vec<isize> {
        self.0.winnings.clone()
    }

    /// The player names in seat order.
    #[getter]
    fn players(&self) -> Vec<String> {
        self.0.players.clone()
    }

    /// The raw round strings (e.g. `["r200ffcfc", "cr850cf"]`).
    fn rounds(&self) -> Vec<String> {
        self.0.rounds.clone()
    }

    /// All actions across all rounds as a flat list of PluribusEvent objects.
    fn actions(&self) -> Vec<PluribusEvent> {
        self.0.actions.iter().map(|e| PluribusEvent(*e)).collect()
    }

    /// The actions for a specific round (0-indexed). Returns an empty list for invalid indices.
    fn actions_for_round(&self, round_index: usize) -> Vec<PluribusEvent> {
        self.0
            .parse_round_at(round_index)
            .into_iter()
            .map(PluribusEvent)
            .collect()
    }

    /// A formatted string showing each player's winnings/losses.
    fn display_results(&self) -> String {
        self.0.display_results()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Pluribus.parse('{}')", self.0.raw)
    }
}

// ============================================================
// Bard
// ============================================================

/// A 64-bit bitset where each bit represents one of the 52 playing cards.
///
/// `Bard` is the binary card representation used internally for fast set operations
/// and as the key type for binary card map lookups. Each card occupies exactly one bit,
/// so operations like union, intersection, and membership test are single CPU instructions.
///
/// Examples:
///     >>> from pkcore import Bard, Card, Cards
///     >>> b = Bard.from_card(Card.parse("As"))
///     >>> b.as_u64() > 0
///     True
///     >>> b.to_cards()
///     Cards.parse('A♠')
///     >>> b2 = Bard.from_cards(Cards.parse("As Ks"))
///     >>> b2.to_cards()
///     Cards.parse('A♠ K♠')
#[pyclass(from_py_object, name = "Bard")]
#[derive(Clone)]
pub struct Bard(PkBard);

#[pymethods]
impl Bard {
    /// Construct a Bard from a raw u64 value.
    #[staticmethod]
    fn from_u64(n: u64) -> Self {
        Bard(PkBard::from(n))
    }

    /// Construct a Bard representing a single card.
    #[staticmethod]
    fn from_card(card: &Card) -> Self {
        Bard(PkBard::from(card.0))
    }

    /// Construct a Bard from a Cards collection (OR of all card bits).
    #[staticmethod]
    fn from_cards(cards: &Cards) -> Self {
        Bard(PkBard::from(cards.0.clone()))
    }

    /// A Bard with no cards set (value = 0).
    #[classattr]
    #[allow(non_snake_case)]
    fn BLANK() -> Self {
        Bard(PkBard::BLANK)
    }

    /// A Bard with all 52 card bits set.
    #[classattr]
    #[allow(non_snake_case)]
    fn ALL() -> Self {
        Bard(PkBard::ALL)
    }

    /// The raw u64 value of this Bard.
    fn as_u64(&self) -> u64 {
        self.0.as_u64()
    }

    /// Return a new Bard with the given card's bit added.
    fn fold_in(&self, card: &Card) -> Self {
        Bard(self.0.fold_in(card.0))
    }

    /// Convert this Bard back to a Cards collection.
    fn to_cards(&self) -> Cards {
        Cards(PkCards::from(self.0))
    }

    /// A human-readable binary string showing which card bits are set, with a guide line.
    fn as_guided_string(&self) -> String {
        self.0.as_guided_string()
    }

    fn __eq__(&self, other: &Bard) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        self.0.as_u64()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Bard.from_u64({})", self.0.as_u64())
    }
}

// ============================================================
// SevenFiveBCM
// ============================================================

/// A binary card map entry for a 5- or 7-card hand.
///
/// Stores the `Bard` of the full hand (`bc`), the `Bard` of the best 5-card
/// sub-hand (`best`), and the hand rank value (`rank`). For a 5-card hand,
/// `bc` and `best` are identical.
///
/// `rank` follows the standard Cactus Kev convention: lower is stronger
/// (1 = royal flush, 7462 = worst high card).
///
/// The `generate_csv` method produces the `bcm.csv` data file (~5 GB) consumed
/// by pkcore's precomputed lookup table. Only call it when you need to regenerate
/// that file.
///
/// Examples:
///     >>> from pkcore import Cards, SevenFiveBCM
///     >>> bcm = SevenFiveBCM.from_cards(Cards.parse("As Ks Qs Js Ts"))
///     >>> bcm.rank
///     1
#[pyclass(from_py_object, name = "SevenFiveBCM")]
#[derive(Clone)]
pub struct SevenFiveBCM(PkSevenFiveBCM);

#[pymethods]
impl SevenFiveBCM {
    /// Build a SevenFiveBCM from a 5- or 7-card Cards collection.
    /// Raises ValueError for any other count.
    #[staticmethod]
    fn from_cards(cards: &Cards) -> PyResult<Self> {
        let v: Vec<PkCard> = cards.0.to_vec();
        PkSevenFiveBCM::try_from(v)
            .map(SevenFiveBCM)
            .map_err(to_py_err)
    }

    /// The Bard (bitset) of the full hand (5 or 7 cards).
    #[getter]
    fn bc(&self) -> Bard {
        Bard(self.0.bc)
    }

    /// The Bard (bitset) of the best 5-card sub-hand.
    #[getter]
    fn best(&self) -> Bard {
        Bard(self.0.best)
    }

    /// The hand rank value (lower = stronger; 1 = royal flush, 7462 = worst high card).
    #[getter]
    fn rank(&self) -> u16 {
        self.0.rank
    }

    /// Generate the BCM CSV data file at the given path.
    /// Enumerates all 5- and 7-card combinations (~5 GB output). This is slow.
    #[staticmethod]
    fn generate_csv(path: &str) -> PyResult<()> {
        PkSevenFiveBCM::generate_csv(path).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// The default CSV path used when the PKCORE_75BCM_CSV_PATH env var is not set.
    #[classattr]
    fn default_csv_path() -> &'static str {
        PkSevenFiveBCM::DEFAULT_PKCORE_75BCM_CSV_PATH
    }

    fn __eq__(&self, other: &SevenFiveBCM) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!(
            "SevenFiveBCM(rank={}, bc={}, best={})",
            self.0.rank,
            self.0.bc.as_u64(),
            self.0.best.as_u64()
        )
    }
}

// ============================================================
// IndexCardMap
// ============================================================

/// A human-readable card map entry for a 5- or 7-card hand.
///
/// Like `SevenFiveBCM` but stores card hands as display strings rather than
/// `Bard` bitsets. Useful for CSV output that needs to be human-inspectable.
///
/// Examples:
///     >>> from pkcore import Cards, IndexCardMap
///     >>> icm = IndexCardMap.from_cards(Cards.parse("As Ks Qs Js Ts"))
///     >>> icm.rank
///     1
///     >>> icm.best
///     'A♠ K♠ Q♠ J♠ T♠'
#[pyclass(from_py_object, name = "IndexCardMap")]
#[derive(Clone)]
pub struct IndexCardMap(PkIndexCardMap);

#[pymethods]
impl IndexCardMap {
    /// Build an IndexCardMap from a 5- or 7-card Cards collection.
    /// Raises ValueError for any other count.
    #[staticmethod]
    fn from_cards(cards: &Cards) -> PyResult<Self> {
        let v: Vec<PkCard> = cards.0.to_vec();
        PkIndexCardMap::try_from(v)
            .map(IndexCardMap)
            .map_err(to_py_err)
    }

    /// String representation of the full hand.
    #[getter]
    fn cards(&self) -> String {
        self.0.cards.clone()
    }

    /// String representation of the best 5-card sub-hand.
    #[getter]
    fn best(&self) -> String {
        self.0.best.clone()
    }

    /// The hand rank value (lower = stronger; 1 = royal flush, 7462 = worst high card).
    #[getter]
    fn rank(&self) -> u16 {
        self.0.rank
    }

    /// Generate the index card map CSV data file at the given path.
    #[staticmethod]
    fn generate_csv(path: &str) -> PyResult<()> {
        PkIndexCardMap::generate_csv(path)
            .map_err(|e: Box<dyn std::error::Error>| PyValueError::new_err(e.to_string()))
    }

    fn __eq__(&self, other: &IndexCardMap) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!(
            "IndexCardMap(rank={}, cards='{}', best='{}')",
            self.0.rank, self.0.cards, self.0.best
        )
    }
}

// ============================================================
// Deck
// ============================================================

/// A standard 52-card deck.
///
/// All methods are static — `Deck` is a namespace for deck-level operations.
///
/// Examples:
///     >>> from pkcore import Deck
///     >>> deck = Deck.poker_cards()
///     >>> len(deck)
///     52
///     >>> shuffled = Deck.poker_cards_shuffled()
///     >>> Deck.len()
///     52
#[pyclass(name = "Deck")]
pub struct Deck;

#[pymethods]
impl Deck {
    /// Returns a full 52-card ordered deck as a Cards collection.
    #[staticmethod]
    fn poker_cards() -> Cards {
        Cards(PkDeck::poker_cards())
    }

    /// Returns a full 52-card deck in random order.
    #[staticmethod]
    fn poker_cards_shuffled() -> Cards {
        Cards(PkDeck::poker_cards_shuffled())
    }

    /// Returns the card at the given index (0–51) in deck order.
    #[staticmethod]
    fn get(index: usize) -> Card {
        Card(PkDeck::get(index))
    }

    /// Returns the number of cards in a standard deck (always 52).
    #[staticmethod]
    fn len() -> usize {
        PkDeck::len()
    }

    fn __repr__(&self) -> String {
        "Deck".to_string()
    }
}

// ============================================================
// ForcedBets
// ============================================================

/// Blind and ante structure for a poker table.
///
/// Examples:
///     >>> from pkcore import ForcedBets
///     >>> fb = ForcedBets(50, 100)
///     >>> fb.small_blind
///     50
///     >>> fb.big_blind
///     100
#[pyclass(from_py_object, name = "ForcedBets")]
#[derive(Clone)]
pub struct ForcedBets(pub(crate) PkForcedBets);

#[pymethods]
impl ForcedBets {
    #[new]
    #[pyo3(signature = (small_blind, big_blind, ante = 0))]
    fn new(small_blind: usize, big_blind: usize, ante: usize) -> Self {
        if ante == 0 {
            ForcedBets(PkForcedBets::new(small_blind, big_blind))
        } else {
            ForcedBets(PkForcedBets::new_with_ante(small_blind, big_blind, ante))
        }
    }

    #[getter]
    fn small_blind(&self) -> usize {
        self.0.small_blind
    }

    #[getter]
    fn big_blind(&self) -> usize {
        self.0.big_blind
    }

    #[getter]
    fn ante(&self) -> usize {
        self.0.ante
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!(
            "ForcedBets({}, {}, ante={})",
            self.0.small_blind, self.0.big_blind, self.0.ante
        )
    }
    fn __eq__(&self, other: &ForcedBets) -> bool {
        self.0 == other.0
    }
}

// ============================================================
// PlayerState
// ============================================================

/// The current action state of a player at the table.
///
/// Query with `kind()` (returns a string) and `amount()` (returns the chip
/// value for states that carry one, or 0 otherwise).
///
/// Possible kinds: "Ready", "YetToAct", "Check", "Blind", "Bet", "Call",
/// "Raise", "ReRaise", "AllIn", "Showdown", "Fold", "Out"
#[pyclass(from_py_object, name = "PlayerState")]
#[derive(Clone)]
pub struct PlayerState(PkPlayerState);

#[pymethods]
impl PlayerState {
    /// The name of this state as a string.
    fn kind(&self) -> &'static str {
        match self.0 {
            PkPlayerState::Ready => "Ready",
            PkPlayerState::YetToAct => "YetToAct",
            PkPlayerState::Check => "Check",
            PkPlayerState::Blind(_) => "Blind",
            PkPlayerState::Bet(_) => "Bet",
            PkPlayerState::Call(_) => "Call",
            PkPlayerState::Raise(_) => "Raise",
            PkPlayerState::ReRaise(_) => "ReRaise",
            PkPlayerState::AllIn(_) => "AllIn",
            PkPlayerState::Showdown(_) => "Showdown",
            PkPlayerState::Fold => "Fold",
            PkPlayerState::Out => "Out",
        }
    }

    /// The chip amount associated with this state (0 for states without an amount).
    fn amount(&self) -> usize {
        match self.0 {
            PkPlayerState::Blind(n)
            | PkPlayerState::Bet(n)
            | PkPlayerState::Call(n)
            | PkPlayerState::Raise(n)
            | PkPlayerState::ReRaise(n)
            | PkPlayerState::AllIn(n)
            | PkPlayerState::Showdown(n) => n,
            _ => 0,
        }
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
    fn is_fold(&self) -> bool {
        matches!(self.0, PkPlayerState::Fold)
    }
    fn is_ready(&self) -> bool {
        matches!(self.0, PkPlayerState::Ready)
    }
    fn is_yet_to_act(&self) -> bool {
        matches!(self.0, PkPlayerState::YetToAct)
    }

    fn __str__(&self) -> String {
        self.kind().to_string()
    }
    fn __repr__(&self) -> String {
        if self.amount() > 0 {
            format!("PlayerState.{}({})", self.kind(), self.amount())
        } else {
            format!("PlayerState.{}", self.kind())
        }
    }
    fn __eq__(&self, other: &PlayerState) -> bool {
        self.0 == other.0
    }
}

// ============================================================
// Stack
// ============================================================

/// A chip stack with interior mutability.
///
/// Most mutation methods do not require the object to be explicitly mutable —
/// the stack tracks its count internally.
///
/// Examples:
///     >>> from pkcore import Stack
///     >>> s = Stack(1000)
///     >>> s.count()
///     1000
#[pyclass(unsendable, name = "Stack")]
pub struct Stack(PkStack);

#[pymethods]
impl Stack {
    #[new]
    fn new(amount: usize) -> Self {
        Stack(PkStack::new(amount))
    }

    /// The current chip count.
    fn count(&self) -> usize {
        self.0.count()
    }

    /// True if this stack has no chips.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0.count())
    }
    fn __repr__(&self) -> String {
        format!("Stack({})", self.0.count())
    }
}

// ============================================================
// Player
// ============================================================

/// A poker player with a name, chip stack, and action state.
///
/// Examples:
///     >>> from pkcore import Player
///     >>> p = Player("Alice", 1000)
///     >>> p.handle
///     'Alice'
///     >>> p.chips()
///     1000
#[pyclass(unsendable, name = "Player")]
pub struct Player(PkPlayer);

#[pymethods]
impl Player {
    /// Create a player with the given name and chip count.
    #[new]
    fn new(handle: String, chips: usize) -> Self {
        Player(PkPlayer::new_with_chips(handle, chips))
    }

    /// The player's display name.
    #[getter]
    fn handle(&self) -> String {
        self.0.handle.clone()
    }

    /// Current chip count (stack only, not including active bet).
    fn chips(&self) -> usize {
        self.0.chips.count()
    }

    /// Total chips (stack + any active bet).
    fn total_chips(&self) -> usize {
        self.0.total_chip_count()
    }

    /// Chips committed to the current hand.
    fn chips_in_play(&self) -> usize {
        self.0.get_chips_in_play()
    }

    /// Current action state.
    fn state(&self) -> PlayerState {
        PlayerState(self.0.state.get())
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
    fn is_ready(&self) -> bool {
        self.0.is_ready()
    }
    fn is_out(&self) -> bool {
        self.0.is_out()
    }
    fn is_tapped_out(&self) -> bool {
        self.0.is_tapped_out()
    }
    fn has_bet(&self) -> bool {
        self.0.has_bet()
    }

    fn __str__(&self) -> String {
        format!("{} ({})", self.0.handle, self.0.chips.count())
    }
    fn __repr__(&self) -> String {
        format!("Player('{}', {})", self.0.handle, self.0.chips.count())
    }
}

// ============================================================
// Seatbit
// ============================================================

/// A bitfield indicating which seats are included in a result or pot.
///
/// Each bit corresponds to one seat (bit 0 = seat 0, bit 1 = seat 1, etc.).
#[pyclass(from_py_object, name = "Seatbit")]
#[derive(Clone)]
pub struct Seatbit(PkSeatbit);

#[pymethods]
impl Seatbit {
    /// True if the given seat number is set in this bitfield.
    fn contains(&self, seat_number: u8) -> bool {
        self.0.contains(seat_number)
    }

    /// Number of seats set in this bitfield.
    fn count(&self) -> usize {
        self.0.count_ones()
    }

    /// The raw u16 value.
    fn as_u16(&self) -> u16 {
        self.0 .0
    }

    fn __str__(&self) -> String {
        format!("{:016b}", self.0 .0)
    }
    fn __repr__(&self) -> String {
        format!("Seatbit(0b{:016b})", self.0 .0)
    }
    fn __eq__(&self, other: &Seatbit) -> bool {
        self.0 == other.0
    }
}

// ============================================================
// SeatEquity
// ============================================================

/// Chip equity for a group of seats — how many chips a set of seats won.
#[pyclass(from_py_object, name = "SeatEquity")]
#[derive(Clone)]
pub struct SeatEquity(PkSeatEquity);

#[pymethods]
impl SeatEquity {
    /// Chip amount.
    #[getter]
    fn chips(&self) -> usize {
        self.0.chips
    }

    /// Which seats share this equity.
    #[getter]
    fn seats(&self) -> Seatbit {
        Seatbit(self.0.seats)
    }

    /// Number of seats sharing this equity.
    fn count(&self) -> usize {
        self.0.count_ones()
    }

    /// True if this equity is zero.
    fn is_nada(&self) -> bool {
        self.0.is_nada()
    }

    fn __str__(&self) -> String {
        format!("{} chips to {} seat(s)", self.0.chips, self.0.count_ones())
    }
    fn __repr__(&self) -> String {
        format!(
            "SeatEquity(chips={}, seats={:b})",
            self.0.chips, self.0.seats.0
        )
    }
    fn __eq__(&self, other: &SeatEquity) -> bool {
        self.0 == other.0
    }
}

// ============================================================
// PotWin
// ============================================================

/// A single pot-win record from a completed hand (chips awarded and winning hand rank).
#[pyclass(from_py_object, name = "PotWin")]
#[derive(Clone)]
pub struct PotWin(PkPotWin);

#[pymethods]
impl PotWin {
    /// The seat equity (chips and which seats won).
    #[getter]
    fn equity(&self) -> SeatEquity {
        SeatEquity(self.0.equity)
    }

    /// The winning hand rank value.
    #[getter]
    fn hand_rank(&self) -> HandRank {
        HandRank(self.0.eval.hand_rank)
    }

    fn __repr__(&self) -> String {
        format!(
            "PotWin(chips={}, rank={})",
            self.0.equity.chips, self.0.eval.hand_rank.value
        )
    }
}

// ============================================================
// Winnings
// ============================================================

/// The payout results from a completed hand — a list of Win records.
#[pyclass(from_py_object, name = "Winnings")]
#[derive(Clone)]
pub struct Winnings(pub(crate) PkWinnings);

#[pymethods]
impl Winnings {
    /// Number of winners.
    fn __len__(&self) -> usize {
        self.0.len()
    }

    /// True if there are no winners recorded.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// All win records as a Python list.
    fn to_list(&self) -> Vec<PotWin> {
        self.0.vec().iter().map(|w| PotWin(*w)).collect()
    }

    /// The primary winner (first PotWin record).
    fn first(&self) -> PotWin {
        PotWin(self.0.first())
    }

    fn __repr__(&self) -> String {
        format!("Winnings(len={})", self.0.len())
    }
}

// ============================================================
// TableAction
// ============================================================

/// A single event in the table event log.
///
/// Query with `kind()` for the event name, `seat()` for which seat it
/// concerns (if any), and `amount()` for the chip value (if any).
#[pyclass(from_py_object, name = "TableAction")]
#[derive(Clone)]
pub struct TableAction(PkTableAction);

#[pymethods]
impl TableAction {
    /// The event type name (e.g. "Bet", "Fold", "DealtFlop", "EndHand").
    fn kind(&self) -> String {
        match &self.0 {
            PkTableAction::Pause => "Pause".into(),
            PkTableAction::TableOpen(_) => "TableOpen".into(),
            PkTableAction::PlayerSeated(_, _) => "PlayerSeated".into(),
            PkTableAction::NewHand => "NewHand".into(),
            PkTableAction::ShuffleDeck => "ShuffleDeck".into(),
            PkTableAction::SetButton(_) => "SetButton".into(),
            PkTableAction::MoveButton(_) => "MoveButton".into(),
            PkTableAction::ForcedBets => "ForcedBets".into(),
            PkTableAction::ForcedBet(_, _) => "ForcedBet".into(),
            PkTableAction::ForcedBetSmallBlind(_, _) => "ForcedBetSmallBlind".into(),
            PkTableAction::ForcedBetBigBlind(_, _) => "ForcedBetBigBlind".into(),
            PkTableAction::BetAnteForced(_, _) => "BetAnteForced".into(),
            PkTableAction::DealingXCards(_) => "DealingXCards".into(),
            PkTableAction::Dealt(_, _) => "Dealt".into(),
            PkTableAction::DealtFlop(_) => "DealtFlop".into(),
            PkTableAction::DealtTurn(_) => "DealtTurn".into(),
            PkTableAction::DealtRiver(_) => "DealtRiver".into(),
            PkTableAction::DealtPlayers => "DealtPlayers".into(),
            PkTableAction::ForceDealt(_, _) => "ForceDealt".into(),
            PkTableAction::BringItIn(_) => "BringItIn".into(),
            PkTableAction::ActionTo(_) => "ActionTo".into(),
            PkTableAction::Check(_) => "Check".into(),
            PkTableAction::Bet(_, _) => "Bet".into(),
            PkTableAction::Call(_, _) => "Call".into(),
            PkTableAction::Raise(_, _) => "Raise".into(),
            PkTableAction::AllIn(_, _) => "AllIn".into(),
            PkTableAction::Fold(_) => "Fold".into(),
            PkTableAction::PotSize(_) => "PotSize".into(),
            PkTableAction::SplitPots() => "SplitPots".into(),
            PkTableAction::MainPot(_) => "MainPot".into(),
            PkTableAction::SidePot(_) => "SidePot".into(),
            PkTableAction::SplitPot(_, _) => "SplitPot".into(),
            PkTableAction::MuckCards(_) => "MuckCards".into(),
            PkTableAction::MuckPlayerCards(_, _) => "MuckPlayerCards".into(),
            PkTableAction::TakePlayerCards(_, _) => "TakePlayerCards".into(),
            PkTableAction::TakeBoardCards(_) => "TakeBoardCards".into(),
            PkTableAction::ClosesTheAction(_) => "ClosesTheAction".into(),
            PkTableAction::CloseItOut(_) => "CloseItOut".into(),
            PkTableAction::EndHand => "EndHand".into(),
            PkTableAction::ChipAuditFailed(_, _) => "ChipAuditFailed".into(),
            PkTableAction::ResetTable => "ResetTable".into(),
            _ => "Other".into(),
        }
    }

    /// The seat number this event concerns, or None.
    fn seat(&self) -> Option<u8> {
        self.0.get_seat()
    }

    /// The chip amount this event concerns, or None.
    fn amount(&self) -> Option<usize> {
        self.0.get_amount()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    fn __repr__(&self) -> String {
        format!("TableAction('{}')", self.kind())
    }
}

// ============================================================
// TableLog
// ============================================================

/// The event log for a poker table — an ordered list of TableAction records.
#[pyclass(unsendable, name = "TableLog")]
pub struct TableLog(PkTableLog);

#[pymethods]
impl TableLog {
    /// All events as a Python list of TableAction objects.
    fn entries(&self) -> Vec<TableAction> {
        self.0.entries().into_iter().map(TableAction).collect()
    }

    /// The most recent event, or None.
    fn last(&self) -> Option<TableAction> {
        self.0.last().map(TableAction)
    }

    /// The most recent player action (bet/fold/call/raise/check), or None.
    fn last_player_action(&self) -> Option<TableAction> {
        self.0.last_player_action().map(TableAction)
    }

    /// True if the blinds have been posted this hand.
    fn have_posted_blinds(&self) -> bool {
        self.0.have_posted_blinds()
    }

    /// Number of events in the log.
    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __repr__(&self) -> String {
        format!("TableLog(len={})", self.0.len())
    }
}

// ============================================================
// Dealer
// ============================================================

/// The primary interface for running a Texas Hold'em table simulation.
///
/// Create a dealer with `ForcedBets` and a seat count, seat players,
/// then drive the hand lifecycle with `start_hand`, individual action
/// methods, `advance_street`, and `end_hand`.
///
/// Examples:
///     >>> from pkcore import Dealer, ForcedBets, Player
///     >>> dealer = Dealer(ForcedBets(50, 100), 6)
///     >>> dealer.seat_player(Player("Alice", 1000))
///     0
///     >>> dealer.seat_player(Player("Bob", 1000))
///     1
#[pyclass(unsendable, name = "Dealer")]
pub struct Dealer(PkDealer);

fn dealer_err(e: PkDealerError) -> PyErr {
    PyValueError::new_err(format!("{e:?}"))
}

#[pymethods]
impl Dealer {
    /// Create a new dealer with the given blind structure and number of seats.
    #[new]
    fn new(forced: &ForcedBets, seat_count: u8) -> Self {
        Dealer(PkDealer::new(forced.0, seat_count))
    }

    // ---- Seating ----

    /// Seat a player at the next available seat. Returns the seat number.
    fn seat_player(&self, player: &mut Player) -> PyResult<u8> {
        // Take the player out of the Python wrapper to give ownership to the table
        let p = std::mem::replace(
            &mut player.0,
            PkPlayer::new_with_chips("__taken__".into(), 0),
        );
        self.0.seat_player(p).map_err(dealer_err)
    }

    /// Seat a player at a specific seat number (0-indexed).
    fn seat_player_at(&self, player: &mut Player, seat: u8) -> PyResult<()> {
        let p = std::mem::replace(
            &mut player.0,
            PkPlayer::new_with_chips("__taken__".into(), 0),
        );
        self.0.seat_player_at(p, seat).map_err(dealer_err)
    }

    /// Remove the player from the given seat. Returns the removed Player.
    fn remove_player(&self, seat: u8) -> PyResult<Player> {
        self.0.remove_player(seat).map(Player).map_err(dealer_err)
    }

    // ---- Hand lifecycle ----

    /// Shuffle the deck, post blinds, and deal hole cards to all seated players.
    fn start_hand(&mut self) -> PyResult<()> {
        self.0.start_hand().map_err(dealer_err)
    }

    /// Consolidate bets and deal the next street (flop → turn → river).
    fn advance_street(&mut self) -> PyResult<()> {
        self.0.advance_street().map_err(dealer_err)
    }

    /// Run the showdown and pay out the pot. Returns the Winnings.
    fn end_hand(&mut self) -> PyResult<Winnings> {
        self.0.end_hand().map(Winnings).map_err(dealer_err)
    }

    // ---- Player actions ----

    /// Player at `seat` bets `amount` chips.
    fn bet(&self, seat: u8, amount: usize) -> PyResult<()> {
        use pkcore::casino::dealer::DealerAction;
        self.0
            .act(DealerAction::Bet { seat, amount })
            .map_err(dealer_err)
    }

    /// Player at `seat` calls the current bet.
    fn call(&self, seat: u8) -> PyResult<()> {
        use pkcore::casino::dealer::DealerAction;
        self.0.act(DealerAction::Call { seat }).map_err(dealer_err)
    }

    /// Player at `seat` checks.
    fn check(&self, seat: u8) -> PyResult<()> {
        use pkcore::casino::dealer::DealerAction;
        self.0.act(DealerAction::Check { seat }).map_err(dealer_err)
    }

    /// Player at `seat` raises to `amount` chips.
    fn raise_to(&self, seat: u8, amount: usize) -> PyResult<()> {
        use pkcore::casino::dealer::DealerAction;
        self.0
            .act(DealerAction::Raise { seat, amount })
            .map_err(dealer_err)
    }

    /// Player at `seat` goes all-in.
    fn all_in(&self, seat: u8) -> PyResult<()> {
        use pkcore::casino::dealer::DealerAction;
        self.0.act(DealerAction::AllIn { seat }).map_err(dealer_err)
    }

    /// Player at `seat` folds.
    fn fold(&self, seat: u8) -> PyResult<()> {
        use pkcore::casino::dealer::DealerAction;
        self.0.act(DealerAction::Fold { seat }).map_err(dealer_err)
    }

    /// Mark player at `seat` as ready to play.
    fn ready(&self, seat: u8) -> PyResult<()> {
        use pkcore::casino::dealer::DealerAction;
        self.0.act(DealerAction::Ready { seat }).map_err(dealer_err)
    }

    // ---- Table state ----

    /// The table's unique ID as a string.
    fn table_id(&self) -> String {
        self.0.table_id().to_string()
    }

    /// True if a hand is currently in progress.
    fn is_hand_in_progress(&self) -> bool {
        self.0.is_hand_in_progress()
    }

    /// The seat number that is next to act.
    fn next_to_act(&self) -> u8 {
        self.0.next_to_act()
    }

    /// The current pot size in chips.
    fn pot(&self) -> usize {
        self.0.pot()
    }

    /// The chip count for the player at the given seat, or None if the seat is empty.
    fn chips_at(&self, seat: u8) -> Option<usize> {
        self.0.chips_at(seat)
    }

    /// A copy of the full event log.
    fn event_log(&self) -> TableLog {
        TableLog(self.0.event_log().clone())
    }

    fn __repr__(&self) -> String {
        format!(
            "Dealer(id={}, in_progress={})",
            self.0.table_id(),
            self.0.is_hand_in_progress()
        )
    }
}

// ============================================================
// Constants
// ============================================================

/// Number of unique 5-card hands from a 52-card deck.
#[pyfunction]
fn unique_5_card_hands() -> usize {
    pkcore::UNIQUE_5_CARD_HANDS
}

/// Number of distinct 5-card hand rankings.
#[pyfunction]
fn distinct_5_card_hands() -> usize {
    pkcore::DISTINCT_5_CARD_HANDS
}

/// Number of unique 2-card starting hands.
#[pyfunction]
fn unique_2_card_hands() -> usize {
    pkcore::UNIQUE_2_CARD_HANDS
}

/// Number of distinct 2-card starting hand types.
#[pyfunction]
fn distinct_2_card_hands() -> usize {
    pkcore::DISTINCT_2_CARD_HANDS
}

// ============================================================
// PotOdds
// ============================================================

/// Pot odds and breakeven equity for a single decision point.
///
/// Examples:
///     >>> from pkpy import PotOdds
///     >>> po = PotOdds(200, 100)
///     >>> po.breakeven()
///     0.3333...
///     >>> po.is_profitable(0.40)
///     True
#[pyclass(from_py_object, name = "PotOdds")]
#[derive(Clone)]
pub struct PotOdds(PkPotOdds);

#[pymethods]
impl PotOdds {
    /// Create a PotOdds from the current pot size and the call amount.
    #[new]
    fn new(pot: u64, call: u64) -> Self {
        PotOdds(PkPotOdds::new(pot, call))
    }

    /// Chips already in the pot before the call.
    #[getter]
    fn pot(&self) -> u64 {
        self.0.pot
    }

    /// Amount the player must put in to call.
    #[getter]
    fn call(&self) -> u64 {
        self.0.call
    }

    /// The fraction of the total pot the player is risking: call / (pot + call).
    fn ratio(&self) -> f64 {
        self.0.ratio()
    }

    /// Minimum equity (0.0–1.0) needed to make calling profitable.
    fn breakeven(&self) -> f64 {
        self.0.breakeven()
    }

    /// True if the given equity (0.0–1.0) justifies a call.
    fn is_profitable(&self, equity: f64) -> bool {
        self.0.is_profitable(equity)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("PotOdds({}, {})", self.0.pot, self.0.call)
    }

    fn __eq__(&self, other: &PotOdds) -> bool {
        self.0 == other.0
    }
}

// ============================================================
// Ev
// ============================================================

/// Expected value of a poker call, derived from outcome counts and pot geometry.
///
/// Examples:
///     >>> from pkpy import Ev, WinLoseDraw, PotOdds
///     >>> odds = WinLoseDraw(7, 3, 0)
///     >>> po = PotOdds(200, 100)
///     >>> ev = Ev(odds, po)
///     >>> ev.is_positive()
///     True
///     >>> ev.as_chips()
///     110.0
#[pyclass(from_py_object, name = "Ev")]
#[derive(Clone)]
pub struct Ev(PkEv);

#[pymethods]
impl Ev {
    /// Create an Ev from a WinLoseDraw and PotOdds.
    #[new]
    fn new(odds: &WinLoseDraw, pot_odds: &PotOdds) -> Self {
        Ev(PkEv::new(odds.0, pot_odds.0))
    }

    /// True if calling is +EV.
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    /// EV in chip units as a float. Returns 0.0 if there are no outcomes.
    fn as_chips(&self) -> f64 {
        self.0.as_chips()
    }

    /// The signed EV numerator: wins × pot - losses × call.
    fn numerator(&self) -> i64 {
        self.0.numerator()
    }

    /// Total number of outcomes (wins + losses + draws).
    fn total(&self) -> u64 {
        self.0.total()
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("Ev(chips={:.2})", self.0.as_chips())
    }

    fn __eq__(&self, other: &Ev) -> bool {
        self.0 == other.0
    }
}

// ============================================================
// Percentage
// ============================================================

/// A simple fraction expressed as number / total, with a percentage calculation.
///
/// Examples:
///     >>> from pkpy import Percentage
///     >>> p = Percentage(7, 10)
///     >>> p.calculate()
///     70.0
#[pyclass(from_py_object, name = "Percentage")]
#[derive(Clone)]
pub struct Percentage(PkPercentage);

#[pymethods]
impl Percentage {
    #[new]
    fn new(number: usize, total: usize) -> Self {
        Percentage(PkPercentage::new(number, total))
    }

    #[getter]
    fn number(&self) -> usize {
        self.0.number
    }

    #[getter]
    fn total(&self) -> usize {
        self.0.total
    }

    /// The percentage value (0.0–100.0).
    fn calculate(&self) -> f32 {
        self.0.calculate()
    }

    fn __repr__(&self) -> String {
        format!("Percentage({}, {})", self.0.number, self.0.total)
    }

    fn __eq__(&self, other: &Percentage) -> bool {
        self.0 == other.0
    }
}

// ============================================================
// Ranks
// ============================================================

/// An ordered collection of ranks, parseable from a whitespace-separated string.
///
/// Examples:
///     >>> from pkpy import Ranks
///     >>> r = Ranks.parse("A K Q")
///     >>> r.count()
///     3
#[pyclass(from_py_object, name = "Ranks")]
#[derive(Clone)]
pub struct Ranks(PkRanks);

#[pymethods]
impl Ranks {
    /// Parse ranks from a whitespace-separated string (e.g. "A K Q J").
    #[staticmethod]
    fn parse(s: &str) -> PyResult<Self> {
        PkRanks::from_str(s).map(Ranks).map_err(to_py_err)
    }

    /// All ranks as a Python list of Rank objects.
    fn to_list(&self) -> Vec<Rank> {
        self.0.vec().iter().map(|r| Rank(*r)).collect()
    }

    /// Number of ranks in this collection.
    fn count(&self) -> usize {
        self.0.vec().len()
    }

    /// The OR-sum of all rank bit flags.
    fn sum_or(&self) -> u16 {
        self.0.sum_or()
    }

    fn __len__(&self) -> usize {
        self.0.vec().len()
    }

    fn __repr__(&self) -> String {
        format!("Ranks(count={})", self.0.vec().len())
    }
}

// ============================================================
// RangeEquity
// ============================================================

/// Range-vs-range equity on a given board.
///
/// Computes the combined WinLoseDraw across all hero hands against a villain range.
/// Requires at least the flop to be dealt.
///
/// Examples:
///     >>> from pkpy import RangeEquity, Combos, Board
///     >>> hero = Combos.parse("QQ+")
///     >>> villain = Combos.parse("AKs,AKo")
///     >>> board = Board.parse("As Kh 2d 7c 3c")
///     >>> re = RangeEquity(hero, villain, board)
///     >>> odds = re.combined_odds()
#[pyclass(skip_from_py_object, name = "RangeEquity")]
pub struct RangeEquity(PkRangeEquity);

#[pymethods]
impl RangeEquity {
    /// Create a RangeEquity for hero range vs villain range on a board.
    #[new]
    fn new(hero: &Combos, villain: &Combos, board: &Board) -> Self {
        RangeEquity(PkRangeEquity::new(
            hero.0.clone(),
            villain.0.clone(),
            board.0,
        ))
    }

    /// Compute the aggregated WinLoseDraw for hero range vs villain range.
    /// Raises ValueError if no board is set (preflop not yet supported).
    fn combined_odds(&self) -> PyResult<WinLoseDraw> {
        self.0.combined_odds().map(WinLoseDraw).map_err(to_py_err)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        "RangeEquity(...)".to_string()
    }
}

// ============================================================
// DealEval
// ============================================================

/// Preflop (deal) evaluation of all players' win percentages across all runouts.
///
/// This is the most computationally intensive evaluation — it enumerates every
/// possible 5-card board. Use for preflop equity analysis.
///
/// Examples:
///     >>> from pkpy import HoleCards, DealEval
///     >>> hc = HoleCards.parse("As Ah 2d 2c")
///     >>> deal = DealEval(hc)
///     >>> print(deal)
#[pyclass(skip_from_py_object, name = "DealEval")]
pub struct DealEval(PkDealEval);

#[pymethods]
impl DealEval {
    /// Create a DealEval for the given hole cards, enumerating all runouts.
    #[new]
    fn new(hands: &HoleCards) -> PyResult<Self> {
        PkDealEval::new(hands.0.clone())
            .map(DealEval)
            .map_err(to_py_err)
    }

    /// Total number of board runout combinations evaluated.
    #[classattr]
    #[allow(non_snake_case)]
    fn HEADSUP_PREFLOP_COMBO_COUNT() -> usize {
        PkDealEval::HEADSUP_PREFLOP_COMBO_COUNT
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        "DealEval(...)".to_string()
    }
}

// ============================================================
// RiverEval
// ============================================================

/// Hand evaluation at the river stage (complete board).
///
/// The board is fully dealt, so there is a single deterministic outcome.
///
/// Examples:
///     >>> from pkpy import HoleCards, Board, Game, RiverEval
///     >>> hc = HoleCards.parse("6s 6h 5d 5c")
///     >>> board = Board.parse("9c 6d 5h 5s 8s")
///     >>> game = Game(hc, board)
///     >>> river = RiverEval.from_game(game)
#[pyclass(from_py_object, name = "RiverEval")]
#[derive(Clone)]
pub struct RiverEval(PkRiverEval);

#[pymethods]
impl RiverEval {
    /// Create a RiverEval from a Game with a complete board. Raises ValueError if river is not dealt.
    #[staticmethod]
    fn from_game(game: &Game) -> PyResult<Self> {
        PkRiverEval::try_from(game.0.clone())
            .map(RiverEval)
            .map_err(to_py_err)
    }

    /// Returns the best-hand Eval for the player at the given index (0-based).
    /// Raises ValueError if index is out of range.
    fn rank_for_player(&self, index: usize) -> PyResult<Eval> {
        self.0.rank_for_player(index).map(Eval).map_err(to_py_err)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        "RiverEval(...)".to_string()
    }
}

// ============================================================
// BetSize
// ============================================================

/// A GTO bet size expressed as a fraction of the pot (numerator/denominator).
///
/// Examples:
///     >>> from pkpy import BetSize
///     >>> b = BetSize.half_pot()
///     >>> b.as_fraction()
///     (1, 2)
///     >>> b.chips(100)
///     50
#[pyclass(from_py_object, name = "BetSize")]
#[derive(Clone)]
pub struct BetSize(PkBetSize);

#[pymethods]
impl BetSize {
    /// Create a BetSize from a fraction. Raises ValueError if denominator is 0.
    #[staticmethod]
    fn new(numerator: u32, denominator: u32) -> PyResult<Self> {
        PkBetSize::new(numerator, denominator)
            .map(BetSize)
            .map_err(to_py_err)
    }

    #[staticmethod]
    fn third_pot() -> Self {
        BetSize(PkBetSize::third_pot())
    }

    #[staticmethod]
    fn half_pot() -> Self {
        BetSize(PkBetSize::half_pot())
    }

    #[staticmethod]
    fn two_thirds_pot() -> Self {
        BetSize(PkBetSize::two_thirds_pot())
    }

    #[staticmethod]
    fn three_quarters_pot() -> Self {
        BetSize(PkBetSize::three_quarters_pot())
    }

    #[staticmethod]
    fn pot() -> Self {
        BetSize(PkBetSize::pot())
    }

    #[staticmethod]
    fn one_and_half_pot() -> Self {
        BetSize(PkBetSize::one_and_half_pot())
    }

    #[staticmethod]
    fn two_pot() -> Self {
        BetSize(PkBetSize::two_pot())
    }

    /// Chip amount for this bet size given the current pot.
    fn chips(&self, pot: u64) -> u64 {
        self.0.chips(pot)
    }

    /// The fraction as a (numerator, denominator) tuple.
    fn as_fraction(&self) -> (u32, u32) {
        self.0.as_fraction()
    }

    fn __repr__(&self) -> String {
        let (n, d) = self.0.as_fraction();
        format!("BetSize({}/{})", n, d)
    }
}

// ============================================================
// BetSizings
// ============================================================

/// Bet sizes available on each street (flop, turn, river).
///
/// Examples:
///     >>> from pkpy import BetSizings, BetSize
///     >>> bs = BetSizings.uniform([BetSize.half_pot(), BetSize.pot()])
#[pyclass(skip_from_py_object, name = "BetSizings")]
#[allow(dead_code)]
pub struct BetSizings(PkBetSizings);

#[pymethods]
impl BetSizings {
    /// Create BetSizings with the same sizes on all streets.
    #[staticmethod]
    fn uniform(sizes: Vec<BetSize>) -> Self {
        BetSizings(PkBetSizings::uniform(
            sizes.into_iter().map(|b| b.0).collect(),
        ))
    }

    /// Create BetSizings with different sizes per street.
    #[staticmethod]
    fn new(flop: Vec<BetSize>, turn: Vec<BetSize>, river: Vec<BetSize>) -> Self {
        BetSizings(PkBetSizings::new(
            flop.into_iter().map(|b| b.0).collect(),
            turn.into_iter().map(|b| b.0).collect(),
            river.into_iter().map(|b| b.0).collect(),
        ))
    }

    fn __repr__(&self) -> String {
        "BetSizings(...)".to_string()
    }
}

// ============================================================
// SolverConfig
// ============================================================

/// Configuration for the GTO CFR solver.
///
/// Examples:
///     >>> from pkpy import SolverConfig, Combos, Board
///     >>> hero = Combos.parse("KK+")
///     >>> villain = Combos.parse("AKs,AKo")
///     >>> board = Board.parse("2h 3d 4c 5s 6h")
///     >>> config = SolverConfig(hero, villain, board, 1000, 200)
#[pyclass(from_py_object, name = "SolverConfig")]
#[derive(Clone)]
pub struct SolverConfig(PkSolverConfig);

#[pymethods]
impl SolverConfig {
    /// Create a SolverConfig with hero range, villain range, board, effective stack, and pot.
    #[new]
    fn new(
        hero_range: &Combos,
        villain_range: &Combos,
        board: &Board,
        effective_stack: u64,
        pot: u64,
    ) -> Self {
        SolverConfig(PkSolverConfig::new(
            hero_range.0.clone(),
            villain_range.0.clone(),
            board.0,
            effective_stack,
            pot,
        ))
    }

    fn __repr__(&self) -> String {
        "SolverConfig(...)".to_string()
    }
}

// ============================================================
// ActionFrequencies
// ============================================================

/// Action frequency distribution for a GTO strategy node.
///
/// Each element is the probability of taking a specific action (0.0–1.0, sums to 1.0).
#[pyclass(from_py_object, name = "ActionFrequencies")]
#[derive(Clone)]
pub struct ActionFrequencies(PkActionFrequencies);

#[pymethods]
impl ActionFrequencies {
    /// Number of actions.
    fn __len__(&self) -> usize {
        self.0.len()
    }

    /// True if there are no actions.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Frequency for action at index, or None.
    fn get(&self, idx: usize) -> Option<f64> {
        self.0.get(idx)
    }

    /// All frequencies as a Python list.
    fn to_list(&self) -> Vec<f64> {
        self.0.as_slice().to_vec()
    }

    fn __repr__(&self) -> String {
        format!("ActionFrequencies(len={})", self.0.len())
    }
}

// ============================================================
// SolverResult
// ============================================================

/// Output from a completed GTO solve — contains the equilibrium strategy.
#[pyclass(skip_from_py_object, name = "SolverResult")]
pub struct SolverResult(pkcore::analysis::gto::solver::SolverResult);

#[pymethods]
impl SolverResult {
    /// Number of iterations the solver ran.
    #[getter]
    fn iterations(&self) -> usize {
        self.0.iterations
    }

    /// Final exploitability (milli-big-blinds per hand).
    #[getter]
    fn exploitability(&self) -> f64 {
        self.0.exploitability
    }

    fn __repr__(&self) -> String {
        format!(
            "SolverResult(iterations={}, exploitability={:.4})",
            self.0.iterations, self.0.exploitability
        )
    }
}

// ============================================================
// Solver
// ============================================================

/// GTO CFR solver for river spots.
///
/// Run `iterate()` one or more times and then `solve()` to get the equilibrium.
///
/// Examples:
///     >>> from pkpy import Solver, SolverConfig, Combos, Board
///     >>> config = SolverConfig(Combos.parse("KK+"), Combos.parse("AKs"), Board.parse("2h 3d 4c 5s 6h"), 1000, 200)
///     >>> solver = Solver(config)
///     >>> result = solver.solve()
///     >>> result.iterations
#[pyclass(skip_from_py_object, name = "Solver")]
pub struct Solver(PkSolver);

#[pymethods]
impl Solver {
    /// Create a Solver from a SolverConfig.
    #[new]
    fn new(config: SolverConfig) -> Self {
        Solver(PkSolver::new(config.0))
    }

    /// Run one CFR iteration. Returns the exploitability estimate.
    fn iterate(&mut self) -> f64 {
        self.0.iterate()
    }

    /// Run until convergence or max iterations and return the result.
    fn solve(&mut self) -> SolverResult {
        SolverResult(self.0.solve())
    }

    /// Current iteration count.
    fn iteration(&self) -> usize {
        self.0.iteration()
    }

    fn __repr__(&self) -> String {
        format!("Solver(iteration={})", self.0.iteration())
    }
}

// ============================================================
// Kuhn Poker
// ============================================================

/// One of the three cards in Kuhn poker's minimal deck: Jack < Queen < King.
///
/// Use the class attributes ``KuhnCard.JACK``, ``KuhnCard.QUEEN``, and
/// ``KuhnCard.KING`` to obtain instances.
#[pyclass(from_py_object, name = "KuhnCard")]
#[derive(Clone)]
pub struct KuhnCard(PkKuhnCard);

#[pymethods]
impl KuhnCard {
    #[classattr]
    #[allow(non_snake_case)]
    fn JACK() -> Self {
        KuhnCard(PkKuhnCard::Jack)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn QUEEN() -> Self {
        KuhnCard(PkKuhnCard::Queen)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn KING() -> Self {
        KuhnCard(PkKuhnCard::King)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
    fn __repr__(&self) -> String {
        format!("KuhnCard.{:?}", self.0)
    }
    fn __eq__(&self, other: &KuhnCard) -> bool {
        self.0 == other.0
    }
    fn __hash__(&self) -> u8 {
        self.0 as u8
    }
    fn __lt__(&self, other: &KuhnCard) -> bool {
        self.0 < other.0
    }
    fn __le__(&self, other: &KuhnCard) -> bool {
        self.0 <= other.0
    }
    fn __gt__(&self, other: &KuhnCard) -> bool {
        self.0 > other.0
    }
    fn __ge__(&self, other: &KuhnCard) -> bool {
        self.0 >= other.0
    }
}

/// An action a player can take in Kuhn poker: Check, Bet, Call, or Fold.
///
/// Use the class attributes ``KuhnAction.CHECK``, ``KuhnAction.BET``,
/// ``KuhnAction.CALL``, and ``KuhnAction.FOLD``.
#[pyclass(from_py_object, name = "KuhnAction")]
#[derive(Clone)]
pub struct KuhnAction(PkKuhnAction);

#[pymethods]
impl KuhnAction {
    #[classattr]
    #[allow(non_snake_case)]
    fn CHECK() -> Self {
        KuhnAction(PkKuhnAction::Check)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn BET() -> Self {
        KuhnAction(PkKuhnAction::Bet)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn CALL() -> Self {
        KuhnAction(PkKuhnAction::Call)
    }
    #[classattr]
    #[allow(non_snake_case)]
    fn FOLD() -> Self {
        KuhnAction(PkKuhnAction::Fold)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
    fn __repr__(&self) -> String {
        format!("KuhnAction.{:?}", self.0)
    }
    fn __eq__(&self, other: &KuhnAction) -> bool {
        self.0 == other.0
    }
    fn __hash__(&self) -> u8 {
        match self.0 {
            PkKuhnAction::Check => 0,
            PkKuhnAction::Bet => 1,
            PkKuhnAction::Call => 2,
            PkKuhnAction::Fold => 3,
        }
    }
}

/// The immutable sequence of actions taken so far in a Kuhn hand.
///
/// ``KuhnHistory`` is a value type: ``push`` returns a new history with the
/// action appended, leaving the original unchanged.
#[pyclass(from_py_object, name = "KuhnHistory")]
#[derive(Clone)]
pub struct KuhnHistory(PkKuhnHistory);

#[pymethods]
impl KuhnHistory {
    #[new]
    fn new() -> Self {
        KuhnHistory(PkKuhnHistory::new())
    }

    /// Returns a new history with the given action appended.
    fn push(&self, action: &KuhnAction) -> Self {
        KuhnHistory(self.0.push(action.0))
    }

    fn len(&self) -> usize {
        self.0.len()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The most recent action, or ``None`` if the history is empty.
    fn last(&self) -> Option<KuhnAction> {
        self.0.last().map(KuhnAction)
    }

    /// All actions as a list, in order.
    fn actions(&self) -> Vec<KuhnAction> {
        self.0.as_slice().iter().map(|&a| KuhnAction(a)).collect()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
    fn __repr__(&self) -> String {
        format!("KuhnHistory({})", self.0)
    }
    fn __eq__(&self, other: &KuhnHistory) -> bool {
        self.0 == other.0
    }
    fn __len__(&self) -> usize {
        self.0.len()
    }
    fn __hash__(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }
}

/// The information visible to one player: their hole card plus the betting history.
///
/// Info sets are the keys used by strategy tables — a strategy maps each info set
/// to a probability distribution over legal actions.
#[pyclass(from_py_object, name = "KuhnInfoSet")]
#[derive(Clone)]
pub struct KuhnInfoSet(PkKuhnInfoSet);

#[pymethods]
impl KuhnInfoSet {
    #[new]
    fn new(card: &KuhnCard, history: &KuhnHistory) -> Self {
        KuhnInfoSet(PkKuhnInfoSet::new(card.0, history.0.clone()))
    }

    #[getter]
    fn card(&self) -> KuhnCard {
        KuhnCard(self.0.card)
    }
    #[getter]
    fn history(&self) -> KuhnHistory {
        KuhnHistory(self.0.history.clone())
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
    fn __repr__(&self) -> String {
        format!("KuhnInfoSet({})", self.0)
    }
    fn __eq__(&self, other: &KuhnInfoSet) -> bool {
        self.0 == other.0
    }
    fn __hash__(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }
}

/// The complete, immutable game state for a Kuhn poker hand.
///
/// ``KuhnState`` is a pure value type: ``apply`` returns a new state rather
/// than mutating in place.
///
/// Example::
///
///     state = KuhnState(KuhnCard.JACK, KuhnCard.KING)
///     terminal = state.apply(KuhnAction.BET).apply(KuhnAction.CALL)
///     assert terminal.is_terminal()
///     assert terminal.payoff() == [-2, 2]  # King beats Jack
#[pyclass(from_py_object, name = "KuhnState")]
#[derive(Clone)]
pub struct KuhnState(PkKuhnState);

#[pymethods]
impl KuhnState {
    #[new]
    fn new(card_p0: &KuhnCard, card_p1: &KuhnCard) -> PyResult<Self> {
        PkKuhnState::new(card_p0.0, card_p1.0)
            .map(KuhnState)
            .map_err(to_py_err)
    }

    /// The hole card dealt to the given player (0 or 1).
    fn card(&self, player: usize) -> KuhnCard {
        KuhnCard(self.0.card(player))
    }

    /// The current betting history.
    fn history(&self) -> KuhnHistory {
        KuhnHistory(self.0.history().clone())
    }

    /// ``True`` if the hand is over and no further actions are possible.
    fn is_terminal(&self) -> bool {
        self.0.is_terminal()
    }

    /// Index of the player who must act next, or ``None`` at terminal nodes.
    fn current_player(&self) -> Option<usize> {
        self.0.current_player()
    }

    /// Legal actions available to the current player (empty list at terminal nodes).
    fn legal_actions(&self) -> Vec<KuhnAction> {
        self.0.legal_actions().into_iter().map(KuhnAction).collect()
    }

    /// Apply an action and return the resulting game state.
    ///
    /// Raises ``ValueError`` if the action is not legal in the current state.
    fn apply(&self, action: &KuhnAction) -> PyResult<KuhnState> {
        self.0.apply(action.0).map(KuhnState).map_err(to_py_err)
    }

    /// Net chip payoff ``[player_0, player_1]`` at a terminal node.
    ///
    /// Raises ``ValueError`` if called on a non-terminal state.
    fn payoff(&self) -> PyResult<Vec<i32>> {
        self.0.payoff().map(|p| p.to_vec()).map_err(to_py_err)
    }

    /// The information set visible to the given player in the current state.
    fn info_set(&self, player: usize) -> KuhnInfoSet {
        KuhnInfoSet(self.0.info_set(player))
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
    fn __repr__(&self) -> String {
        format!("KuhnState({})", self.0)
    }
    fn __eq__(&self, other: &KuhnState) -> bool {
        self.0 == other.0
    }
}

/// A strategy for Kuhn poker: a mapping from info sets to action probabilities.
///
/// Use ``KuhnStrategy.gto(alpha)`` for the analytical Nash equilibrium, or
/// ``KuhnStrategy.default()`` for the maximum-bluff-frequency GTO (alpha=1/3).
#[pyclass(from_py_object, name = "KuhnStrategy")]
#[derive(Clone)]
pub struct KuhnStrategy(PkKuhnStrategy);

#[pymethods]
impl KuhnStrategy {
    /// Builds the analytical Nash equilibrium parameterized by ``alpha ∈ [0, 1/3]``.
    ///
    /// ``alpha`` controls Player 0's bluffing frequency with a Jack.
    /// Raises ``ValueError`` if alpha is outside ``[0, 1/3]``.
    #[staticmethod]
    fn gto(alpha: f64) -> PyResult<Self> {
        PkKuhnStrategy::gto(alpha)
            .map(KuhnStrategy)
            .map_err(to_py_err)
    }

    /// Returns the GTO strategy with ``alpha = 1/3`` (maximum bluff frequency).
    #[staticmethod]
    #[allow(clippy::should_implement_trait)]
    fn default() -> Self {
        KuhnStrategy(PkKuhnStrategy::default())
    }

    /// Probability distribution over legal actions for the given info set.
    ///
    /// Returns a list of ``(KuhnAction, probability)`` tuples. Probabilities sum
    /// to 1.0. Returns an empty list for terminal or unknown info sets.
    fn action_probs(&self, info_set: &KuhnInfoSet) -> Vec<(KuhnAction, f64)> {
        self.0
            .action_probs(&info_set.0)
            .iter()
            .map(|&(a, p)| (KuhnAction(a), p))
            .collect()
    }

    fn __repr__(&self) -> String {
        "KuhnStrategy(...)".to_string()
    }
}

/// Vanilla CFR trainer for Kuhn poker.
///
/// Implements counterfactual regret minimization over the full Kuhn game tree.
/// After enough iterations, ``average_strategy`` converges to Nash equilibrium
/// and ``exploitability`` approaches zero.
///
/// Example::
///
///     cfr = KuhnCfr()
///     cfr.train(10_000)
///     print(cfr.exploitability())   # → near 0
///     strategy = cfr.average_strategy()
#[pyclass(name = "KuhnCfr")]
pub struct KuhnCfr(PkKuhnCfr);

#[pymethods]
impl KuhnCfr {
    #[new]
    fn new() -> Self {
        KuhnCfr(PkKuhnCfr::new())
    }

    /// Run ``iterations`` of vanilla CFR (traverses all 6 deals each iteration).
    ///
    /// Calling ``train`` multiple times accumulates on top of prior training.
    fn train(&mut self, iterations: u32) {
        self.0.train(iterations);
    }

    /// Average strategy accumulated over all training iterations.
    ///
    /// Converges to Nash equilibrium as iterations increase.
    fn average_strategy(&self) -> KuhnStrategy {
        KuhnStrategy(self.0.average_strategy())
    }

    /// Exploitability of the current average strategy (0.0 at Nash equilibrium).
    fn exploitability(&self) -> f64 {
        self.0.exploitability()
    }

    fn __repr__(&self) -> String {
        "KuhnCfr(...)".to_string()
    }
}

// ============================================================
// Module
// ============================================================

#[pymodule]
fn _pkpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rank>()?;
    m.add_class::<Suit>()?;
    m.add_class::<Card>()?;
    m.add_class::<Cards>()?;
    m.add_class::<Deck>()?;
    m.add_class::<Bard>()?;
    m.add_class::<SevenFiveBCM>()?;
    m.add_class::<IndexCardMap>()?;
    m.add_class::<HoleCards>()?;
    m.add_class::<PluribusEvent>()?;
    m.add_class::<Pluribus>()?;
    m.add_class::<ForcedBets>()?;
    m.add_class::<PlayerState>()?;
    m.add_class::<Stack>()?;
    m.add_class::<Player>()?;
    m.add_class::<Seatbit>()?;
    m.add_class::<SeatEquity>()?;
    m.add_class::<PotWin>()?;
    m.add_class::<Winnings>()?;
    m.add_class::<TableAction>()?;
    m.add_class::<TableLog>()?;
    m.add_class::<Dealer>()?;
    m.add_class::<Board>()?;
    m.add_class::<HandRankClass>()?;
    m.add_class::<HandRank>()?;
    m.add_class::<Eval>()?;
    m.add_class::<CaseEvals>()?;
    m.add_class::<Outs>()?;
    m.add_class::<Game>()?;
    m.add_class::<FlopEval>()?;
    m.add_class::<TurnEval>()?;
    m.add_class::<WinLoseDraw>()?;
    m.add_class::<HUPResult>()?;
    m.add_class::<ComboPairs>()?;
    m.add_class::<Versus>()?;
    m.add_class::<Qualifier>()?;
    m.add_class::<Combo>()?;
    m.add_class::<Two>()?;
    m.add_class::<Twos>()?;
    m.add_class::<Combos>()?;
    m.add_class::<PotOdds>()?;
    m.add_class::<Ev>()?;
    m.add_class::<Percentage>()?;
    m.add_class::<Ranks>()?;
    m.add_class::<RangeEquity>()?;
    m.add_class::<DealEval>()?;
    m.add_class::<RiverEval>()?;
    m.add_class::<BetSize>()?;
    m.add_class::<BetSizings>()?;
    m.add_class::<SolverConfig>()?;
    m.add_class::<ActionFrequencies>()?;
    m.add_class::<SolverResult>()?;
    m.add_class::<Solver>()?;
    m.add_class::<KuhnCard>()?;
    m.add_class::<KuhnAction>()?;
    m.add_class::<KuhnHistory>()?;
    m.add_class::<KuhnInfoSet>()?;
    m.add_class::<KuhnState>()?;
    m.add_class::<KuhnStrategy>()?;
    m.add_class::<KuhnCfr>()?;
    m.add_function(wrap_pyfunction!(unique_5_card_hands, m)?)?;
    m.add_function(wrap_pyfunction!(distinct_5_card_hands, m)?)?;
    m.add_function(wrap_pyfunction!(unique_2_card_hands, m)?)?;
    m.add_function(wrap_pyfunction!(distinct_2_card_hands, m)?)?;
    hand_history::register(m)?;
    stats::register(m)?;
    bot::register(m)?;
    session::register(m)?;
    table_no_cell::register(m)?;
    Ok(())
}

#[cfg(test)]
mod table_action_kind_tests {
    use super::*;

    #[test]
    fn chip_audit_failed_returns_explicit_kind() {
        let ta = TableAction(PkTableAction::ChipAuditFailed(100, 99));
        assert_eq!("ChipAuditFailed", ta.kind());
    }

    #[test]
    fn reset_table_returns_explicit_kind() {
        let ta = TableAction(PkTableAction::ResetTable);
        assert_eq!("ResetTable", ta.kind());
    }
}
