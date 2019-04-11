use crate::blackjack::hand_value::{HandValue, WithHandValue};
use crate::blackjack::player::PlayerType;
use crate::blackjack::wager::{Wager, WithWager};
use crate::cards::{Card, Hand};
use std::sync::Arc;

pub enum HandState {
    Default,
    WagerPlaced,
    CardsDealt,
    HandPlayed,
}

pub enum ResultsState {
    Default,
    Natural,
    Pushed,
    Lost,
    Won,
    Busted,
}

// @todo likely need to split up the states even more -- consider individual wager state + pay state??

pub struct BlackjackHand {
    hand: Hand,
    player_type: PlayerType,
    state: HandState,
    result_state: ResultsState,
    wager: Wager,
}

impl BlackjackHand {
    pub fn new(player_type: PlayerType) -> BlackjackHand {
        BlackjackHand {
            player_type,
            hand: Hand::new(),
            state: HandState::Default,
            result_state: ResultsState::Default,
            wager: Wager::new(),
        }
    }

    pub fn with_cards(
        player_type: PlayerType,
        cards: Vec<Arc<Card>>,
    ) -> BlackjackHand {
        BlackjackHand {
            player_type,
            hand: Hand::with_cards(cards),
            state: HandState::Default,
            result_state: ResultsState::Default,
            wager: Wager::new(),
        }
    }

    pub fn get_state(&self) -> &HandState {
        &self.state
    }

    pub fn set_state(&mut self, state: HandState) {
        self.state = state;
    }

    pub fn add_card(&mut self, card: Arc<Card>) {
        self.hand.add_card(card)
    }

    pub fn add_cards(&mut self, cards: Vec<Arc<Card>>) {
        self.hand.add_cards(cards);
    }

    pub fn get_cards(&self) -> &Vec<Arc<Card>> {
        self.hand.get_cards()
    }

    pub fn get_card_count(&self) -> usize {
        self.hand.get_card_count()
    }

    pub fn can_split(&self) -> bool {
        match self.player_type {
            PlayerType::Dealer => false,
            PlayerType::Player => {
                self.hand.get_card_count() == 2
                    && self.hand.get_cards().get(0)
                        == self.hand.get_cards().get(1)
            }
        }
    }

    pub fn split(&mut self) -> Option<[BlackjackHand; 2]> {
        if self.can_split() && !self.is_dealer() {
            let player_type = self.player_type.clone();
            let cards = self.hand.get_cards();

            return Some([
                BlackjackHand::with_cards(
                    player_type.clone(),
                    vec![cards.get(0).unwrap().clone()],
                ),
                BlackjackHand::with_cards(
                    player_type,
                    vec![cards.get(1).unwrap().clone()],
                ),
            ]);
        }
        None
    }

    pub fn can_double_down(&self) -> bool {
        match &self.player_type {
            PlayerType::Dealer => false,
            PlayerType::Player => self.hand.get_card_count() == 2,
        }
    }

    pub fn can_hit(&self) -> bool {
        match self.player_type {
            PlayerType::Player => match self.get_value() {
                Some(HandValue::V(n)) => n < 21,
                Some(HandValue::Ace(_, high)) => high < 21,
                _ => false,
            },
            PlayerType::Dealer => match self.get_value() {
                Some(HandValue::V(n)) => n < 17,
                Some(HandValue::Ace(low, high)) => low <= 17 || high < 17, // hit on soft 17
                _ => false,
            },
        }
    }

    pub fn is_dealer(&self) -> bool {
        self.player_type == PlayerType::Dealer
    }

    pub fn reset(&mut self) {
        self.hand.reset_cards();
        self.state = HandState::Default;
        self.result_state = ResultsState::Default;
        self.wager.reset_wager();
    }
}

pub trait ResultState {
    fn set_result_state(&mut self, state: ResultsState);
    fn get_result_state(&self) -> &ResultsState;

    fn set_natural(&mut self) {
        self.set_result_state(ResultsState::Natural);
    }

    fn set_busted(&mut self) {
        self.set_result_state(ResultsState::Busted);
    }

    fn set_lost(&mut self) {
        self.set_result_state(ResultsState::Lost);
    }

    fn set_won(&mut self) {
        self.set_result_state(ResultsState::Won);
    }

    fn set_pushed(&mut self) {
        self.set_result_state(ResultsState::Pushed);
    }

    fn did_win(&self) -> bool {
        match self.get_result_state() {
            ResultsState::Won => true,
            ResultsState::Natural => true,
            _ => false,
        }
    }
}

impl ResultState for BlackjackHand {
    fn set_result_state(&mut self, state: ResultsState) {
        self.result_state = state;
    }

    fn get_result_state(&self) -> &ResultsState {
        &self.result_state
    }
}

impl WithWager for BlackjackHand {
    fn get_mut_wager(&mut self) -> &mut Wager {
        &mut self.wager
    }

    fn get_wager(&self) -> &Wager {
        &self.wager
    }

    fn set_wagered_value(&mut self, wager: u32) {
        self.wager.set_wager(wager);
        self.set_state(HandState::WagerPlaced)
    }
}

impl WithHandValue for BlackjackHand {
    fn get_cards(&self) -> &Vec<Arc<Card>> {
        self.hand.get_cards()
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::{Card, Hand, Rank, Suit};

    use super::*;

    #[test]
    fn it_can_split() {
        let card = Arc::new(Card::from(Suit::Club, Rank::Five));
        let card2 = Arc::new(Card::from(Suit::Club, Rank::Five));

        let mut player = BlackjackHand::new(PlayerType::Player);
        let mut dealer = BlackjackHand::new(PlayerType::Dealer);

        player.add_card(card.clone());
        player.add_card(card2.clone());

        dealer.add_card(card);
        dealer.add_card(card2);

        assert!(player.can_split());
        assert!(!dealer.can_split());
    }

    #[test]
    fn it_does_split() {
        let card1 = Arc::new(Card::from(Suit::Club, Rank::Eight));
        let card2 = Arc::new(Card::from(Suit::Club, Rank::Eight));

        let mut hand = BlackjackHand::new(PlayerType::Player);

        hand.add_card(card1);
        hand.add_card(card2);

        let hands = hand.split();

        assert!(hands.is_some());

        let hands = hands.unwrap();
        let hand1 = hands.get(0).unwrap();
        let hand2 = hands.get(1).unwrap();

        assert_eq!(hand1.get_card_count(), 1);
        assert_eq!(hand2.get_card_count(), 1);

        let card1 = hand1.get_cards().first().unwrap();
        let card2 = hand2.get_cards().first().unwrap();

        assert_eq!(card1, card2);
    }

    #[test]
    fn can_double_down() {
        let card1 = Arc::new(Card::from(Suit::Club, Rank::Five));
        let card2 = Arc::new(Card::from(Suit::Club, Rank::Six));

        let player = BlackjackHand::with_cards(
            PlayerType::Player,
            vec![card1.clone(), card2.clone()],
        );
        let dealer =
            BlackjackHand::with_cards(PlayerType::Dealer, vec![card1, card2]);

        assert!(player.can_double_down());
        assert!(!dealer.can_double_down());
    }

    #[test]
    fn can_hit_player() {
        let card1 = Arc::new(Card::from(Suit::Club, Rank::Five));
        let card2 = Arc::new(Card::from(Suit::Club, Rank::Seven));

        let mut player =
            BlackjackHand::with_cards(PlayerType::Player, vec![card1, card2]);

        assert!(player.can_hit());

        let card3 = Arc::new(Card::from(Suit::Club, Rank::King));
        player.add_card(card3);

        assert!(!player.can_hit());
    }

    #[test]
    fn can_hit_dealer() {
        let card1 = Arc::new(Card::from(Suit::Club, Rank::Three));
        let card2 = Arc::new(Card::from(Suit::Club, Rank::Four));

        let mut dealer =
            BlackjackHand::with_cards(PlayerType::Dealer, vec![card1, card2]);

        assert!(dealer.can_hit()); // 7

        let card3 = Arc::new(Card::from(Suit::Club, Rank::King));
        dealer.add_card(card3.clone());

        assert!(!dealer.can_hit()); // 17

        dealer.reset();

        let card1 = Arc::new(Card::from(Suit::Club, Rank::Ace));
        let card2 = Arc::new(Card::from(Suit::Club, Rank::Six));

        dealer.add_cards(vec![card1, card2]);

        assert!(dealer.can_hit()); // soft 17

        dealer.add_card(card3);

        assert!(!dealer.can_hit()); // hard 17
    }
}
