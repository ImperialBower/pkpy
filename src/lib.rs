use pkcore::analysis::case_evals::CaseEvals as PkCaseEvals;
use pkcore::analysis::class::HandRankClass as PkHandRankClass;
use pkcore::analysis::eval::Eval as PkEval;
use pkcore::analysis::gto::combo::{Combo as PkCombo, Qualifier as PkQualifier};
use pkcore::analysis::gto::combo_pairs::ComboPairs as PkComboPairs;
use pkcore::analysis::gto::combos::Combos as PkCombos;
use pkcore::analysis::gto::odds::WinLoseDraw as PkWinLoseDraw;
use pkcore::analysis::gto::twos::Twos as PkTwos;
use pkcore::analysis::gto::vs::Versus as PkVersus;
use pkcore::analysis::hand_rank::HandRank as PkHandRank;
use pkcore::analysis::outs::Outs as PkOuts;
use pkcore::analysis::store::db::hup::HUPResult as PkHUPResult;
use pkcore::arrays::two::Two as PkTwo;
use pkcore::card::Card as PkCard;
use pkcore::cards::Cards as PkCards;
use pkcore::play::board::Board as PkBoard;
use pkcore::play::game::Game as PkGame;
use pkcore::play::hole_cards::HoleCards as PkHoleCards;
use pkcore::play::stages::flop_eval::FlopEval as PkFlopEval;
use pkcore::play::stages::turn_eval::TurnEval as PkTurnEval;
use pkcore::rank::Rank as PkRank;
use pkcore::suit::Suit as PkSuit;
use pkcore::{Pile, GTO};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

fn to_py_err(e: impl std::fmt::Display) -> PyErr {
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
// Module
// ============================================================

#[pymodule]
fn _pkpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rank>()?;
    m.add_class::<Suit>()?;
    m.add_class::<Card>()?;
    m.add_class::<Cards>()?;
    m.add_class::<HoleCards>()?;
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
    m.add_function(wrap_pyfunction!(unique_5_card_hands, m)?)?;
    m.add_function(wrap_pyfunction!(distinct_5_card_hands, m)?)?;
    m.add_function(wrap_pyfunction!(unique_2_card_hands, m)?)?;
    m.add_function(wrap_pyfunction!(distinct_2_card_hands, m)?)?;
    Ok(())
}
