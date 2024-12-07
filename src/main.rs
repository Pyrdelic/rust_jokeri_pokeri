use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{char, env, io::{self, stdout, Write}};
use console::Term;
use console::style;



struct Card {
    value: u8,
    suit: char,
}
impl Card{
    fn new(value: u8, suit: char)->Card{
        Card{
            value: value,
            suit: suit,
        }
    }
    pub fn print(&self){
        println!("Value: {} Suit: {}", self.value, self.suit);
    }
}

struct Deck {
    cards: Vec<Card>,
    rng: rand::rngs::ThreadRng
}
impl Deck{
    /// Creates a filled and shuffled Deck.
    fn new()->Self{
        let mut deck: Deck = Self { 
            cards: Vec::new(),
            rng: thread_rng(),
        };
        deck.fill_deck();
        //deck.print();
        deck.shuffle_deck();
        return deck;
    }

    fn print(&mut self){
        for i in &self.cards{
            println!("{} {}", i.value, i.suit);
        }
    }
    fn shuffle_deck(&mut self){
        self.cards.shuffle(&mut self.rng);
    }
    fn fill_deck(&mut self){
        // add base cards
        // values
        for value in 1..=13{
            //suites
            for character_code in 0x2660..=0x2663{
                self.cards.push(Card::new(u8::from(value)
                , char::from_u32(character_code).unwrap()))
            }
        }
        // TODO: add 2 joker cards (and implement)
        // self.cards.push(Card::new(0, 'J'));
        // self.cards.push(Card::new(0, 'J'));
    }
}

struct Hand{
    //cards: Vec<Card>,
    cards: [Option<Card>; 5],
    selections: Vec<u8>,
}
impl Hand{
    pub fn new()->Self{
        Self { 
            cards: [None, None, None, None, None],
            selections: Vec::new(),
        }
    }

    /// Returns true, if any of the card slots are None
    fn has_nones(&self)->bool{
        for element in &self.cards{
            match element{
                Some(_)=>{ continue; }
                None=>{
                    return true
                }
            }
        }
        return false
    }

    fn is_straight_flush(&self)->bool{
        if self.is_straight() && self.is_flush(){
            return true;
        }
        return false;
    }

    fn is_four_of_a_kind(&self)->bool{
        // loop through cards
        for i in &self.cards{
            match i{
                Some(card)=>{
                    // check how many times that cards value is found
                    let count = self.cards.iter()
                    .filter(|&x| x.as_ref().unwrap().value == card.value).count();
                    if count >= 4{
                        return true;
                    }
                }
                None=>{
                    return false;
                }
            }
        }
        return false;
    }

    fn is_full_house(&self)->bool{
        if self.is_three_of_a_kind(){
            // check that there are only two kinds of values in hand
            // (four of a kind is checked before full house)
            let mut unique_values = Vec::<u8>::new();
            for element in &self.cards{
                match element{
                    Some(card)=>{
                        if !unique_values.contains(&card.value){
                            unique_values.push(card.value);
                        }
                    }
                    None=>{
                        return false;
                    }
                }
                if unique_values.len() > 2{
                    return false;
                }
            }
            return true;
        }
        return false;
    }

    fn is_three_of_a_kind(&self)->bool{
        // loop through cards
        for i in &self.cards{
            match i{
                Some(card)=>{
                    // check how many times that cards value is found
                    let count = self.cards.iter()
                    .filter(|&x| x.as_ref().unwrap().value == card.value).count();
                    if count >= 3{
                        return true;
                    }
                }
                None=>{
                    return false;
                }
            }
        }
        return false;
    }

    fn is_flush(&self)->bool{
        let first_suit = match &self.cards[0]{
            Some(card)=>{
                card.suit
            }
            None=>{
                return false;
            }
        };
        for i in 1..self.cards.len(){
            let _ = match &self.cards[i]{
                Some(card)=>{
                    if first_suit == card.suit{
                        continue;
                    }
                    else {
                        return false;
                    }
                }
                None=>{
                    return false;
                }
            };
        }
        return true;
    }

    fn is_straight(&self)->bool{
        let mut hand_vec  = Vec::<&Card>::new();
        for element in &self.cards{
            match element{
                Some(card)=>{
                    hand_vec.push(card);
                }
                None=>{
                    // Not enough cards in hand for a straight
                    return false;
                }
            }
        }
        hand_vec.sort_unstable_by_key(|card| card.value);
        let mut curr_value: u8 = hand_vec.get(0).unwrap().value;
        for i in 1..hand_vec.len(){
            let next_value = hand_vec.get(i).unwrap().value;
            if next_value - curr_value != 1{
                return false;
            }
            else {
                curr_value = next_value;
            }
        }
        return true;
    }

    fn is_two_pairs(&self)->bool{
        let mut pairs_count: u8 = 0;
        let mut first_pair_value: u8 = 0;
        for i in &self.cards{
            match i{
                Some(card)=>{
                    // check how many times that cards value is found
                    let count = self.cards.iter()
                    .filter(|&x| x.as_ref().unwrap().value == card.value).count();
                    if count >= 2 && card.value != first_pair_value{
                        pairs_count += 1;
                        first_pair_value = card.value;
                    }
                }
                None=>{
                    return false;
                }
            }
            if pairs_count >= 2{
                return true;
            }
        }
        return false;
    }

    

    fn print(&self){
        // suits row
        for element in &self.cards{
            match element{
                Some(card) =>{
                    print!(" {:<2}", card.suit);
                }
                None =>{
                    print!(" _ ");
                }
            }
        }
        println!();
        // values row
        for element in &self.cards{
            match element{
                Some(card) =>{
                    print!(" {:<2}", card.value);
                }
                None =>{
                    print!(" _ ");
                }
            }
        }
        println!();
    }
    // fn is_wincondition1...
    // fn is_wincondition2...
}

#[derive(PartialEq)]
enum GameState{
    Betting,
    HandSelection,
    PayOut,
    GameOver,
}

enum Prize{
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
}
impl Prize{
    fn as_str(&self)->&'static str{
        match self{
            Prize::StraightFlush => "Straight flush",
            Prize::FourOfAKind => "Four-of-a-kind",
            Prize::FullHouse => "Full house",
            Prize::Flush => "Flush",
            Prize::Straight => "Straight",
            Prize::ThreeOfAKind => "Three-of-a-kind",
            Prize::TwoPairs => "Two pairs",
        }
    }
}
/// Struct to hold all of the game's data and functionality
struct JokeriPokeri{
    deck: Deck,
    hand: Hand,
    discarded: Vec<Card>,
    funds: u32,
    round: u32,
    bet_amount: u32,
    latest_prize: Option<Prize>,
    latest_payout: u32,
    state: GameState,
    playing: bool,
    selector: usize,
    selected: [bool; 5],
}
impl JokeriPokeri{
    fn new()->Self{
        let mut game: JokeriPokeri = JokeriPokeri { 
            deck: Deck::new(), 
            hand: Hand::new(),
            discarded: Vec::new(),
            funds: 100, 
            round: 1, 
            bet_amount: 20,
            latest_payout: 0,
            state: GameState::Betting,
            playing: true, 
            selector: 0,
            selected: [false, false, false, false, false],
            latest_prize: None,
        };
        return game;
    }

    fn reset_game(&mut self){
        self.deck = Deck::new();
        self.hand = Hand::new();
        self.discarded = Vec::new();
        self.funds = 100;
        self.round = 1;
        self.bet_amount = 20;
        self.latest_payout = 0;
        self.state = GameState::Betting;
        self.playing = true;
        self.selector = 0;
        self.selected = [false, false, false, false, false];
        self.latest_prize = None;
    }

    /// Deals cards from deck to every unselected slot in hand.
    /// Discards all unselected cards if in hand.
    fn deal(&mut self){
        for i in 0..self.hand.cards.len(){
            // skip those cards that are selected to hold
            if !self.selected[i]{
                // discard if unselected card in hand
                let element = self.hand.cards[i].take();
                match element{
                    Some(card) =>{
                        self.discarded.push(card);
                    }
                    _ =>{

                    }
                }
                // draw a new card to hand.
                self.hand.cards[i] = Some(self.deck.cards.remove(0));
            }
        }
    }

    fn reset_deck_and_hand(&mut self){
        // discard hand to discard pile
        for i in 0.. self.hand.cards.len(){
            let element = self.hand.cards[i].take();
            match element{
                Some(card) => {
                    self.discarded.push(card);
                }
                None => {
                }
            }
        }

        // discard discard back to deck
        for i in 0..self.discarded.len(){
            self.deck.cards.push(self.discarded.remove(0));
        }
        // shuffle deck
        self.deck.shuffle_deck();
    }

    fn toggle_selection(&mut self){
        self.selected[self.selector] = !self.selected[self.selector];
    }

    fn reset_selections(&mut self){
        for i in 0..self.selected.len(){
            self.selected[i] = false;
        }
    }

    fn cycle_bet_amount(&mut self){
        if self.bet_amount < 100 && self.funds >= self.bet_amount + 20{
            self.bet_amount += 20;
        }
        else {
            self.bet_amount = 20;
        }
    }

    fn query_quit(&mut self, input: &String){
        let input_lowercase = input.to_lowercase();
        match input_lowercase.as_str(){
            "quit" | "exit" =>{
                println!("Quitting.");
                self.playing = false;
            }
            _ => {

            }
        }
    }

    fn bet_money(&mut self, bet_amount: u32){
        if bet_amount >= 1 && bet_amount <= 100
        && self.funds >= bet_amount{
            self.bet_amount = bet_amount;
            self.funds -= bet_amount;
            self.state = GameState::HandSelection;
        } else{
            println!("Invaid bet!")
        }
    }

    fn print_hand_and_selector(&self){
        self.hand.print();
        // print hand selector row
        if self.state == GameState::HandSelection{
            // print selected row
            for i in self.selected{
                if i{
                    print!("HLD");
                } else {
                    print!("   ");
                }
            }
            println!();
            // print selector
            for i in 0..self.hand.cards.len(){
                if self.selector == i{
                    print!(" ^ ");
                } else{
                    print!("   ");
                }
            }
            println!();
        }
        else {
            println!();
            println!();
        }
    }

    fn print_prizes(&self){
        println!("{:<25}{:<10}", Prize::StraightFlush.as_str(), 40*self.bet_amount);
        println!("{:<25}{:<10}", Prize::FourOfAKind.as_str(), 15*self.bet_amount);
        println!("{:<25}{:<10}", Prize::FullHouse.as_str(), 7*self.bet_amount);
        println!("{:<25}{:<10}", Prize::Flush.as_str(), 4*self.bet_amount);
        println!("{:<25}{:<10}", Prize::Straight.as_str(), 3*self.bet_amount);
        println!("{:<25}{:<10}", Prize::ThreeOfAKind.as_str(), 2*self.bet_amount);
        println!("{:<25}{:<10}", Prize::TwoPairs.as_str(), 2*self.bet_amount);
    }

    fn print_screen(&self){
        self.print_prizes();
        println!();
        self.print_hand_and_selector();
        self.print_stats();
        println!();
        match self.state{
            GameState::Betting=>{
                //self.hand.print();
                println!("b - cycle bet amount");
                println!("enter - start game");
            }
            GameState::HandSelection=>{

                //self.hand.print();
                
                //std::io::stdout().flush();

                println!();

                // print selector cursor row position
                // for i in 0..self.hand.cards.len(){
                //     if self.selector == i{
                //         print!(" ^ ");
                //     } else{
                //         print!("   ");
                //     }
                // }
                println!("left/right - move selector");
                println!("space - select card");
                println!("enter - continue");

            }
            GameState::PayOut=>{
                match &self.latest_prize{
                    Some(prize) => {
                        println!("{}!", prize.as_str());
                    }
                    _=>{println!("No win.");}
                }

                if self.funds != 0 {
                    println!("enter - new round");
                }
                //self.print_deck_and_selector();
            }
            GameState::GameOver =>{
                println!("OUT OF FUNDS");
                println!("You made it round {}", self.round);
                println!("New game y/n?");
            }
        }
    }

    pub fn play(&mut self){

        //let mut playing: bool = true;
        //let stdout = Term::buffered_stdout();
        let term = Term::stdout();
        
        loop {
            // print screen
            let _ = term.clear_screen();
            //println!("DEBUG: deck len {}", self.deck.cards.len());
            self.print_screen();

            // handle input
            match term.read_key().unwrap(){
                console::Key::Escape =>{
                    println!("Exiting...");
                    break;
                }
                console::Key::Enter =>{
                    match self.state{
                        GameState::Betting =>{
                            self.funds -= self.bet_amount;

                            self.deck.shuffle_deck();
                            self.reset_selections();
                            self.deal();
                            self.state = GameState::HandSelection;
                        }
                        GameState::HandSelection =>{
                            //self.hand.print();
                            self.deal();
                            self.state = GameState::PayOut;
                            // check wins
                            if self.hand.is_straight_flush(){
                                self.latest_prize = Some(Prize::StraightFlush);
                                //println!("Straight flush!");
                                self.funds += 40 * self.bet_amount;
                            }
                            else if self.hand.is_four_of_a_kind(){
                                self.latest_prize = Some(Prize::FourOfAKind);
                                //println!("Four of a kind!");
                                self.funds += 15 * self.bet_amount;
                            }
                            else if self.hand.is_full_house(){
                                self.latest_prize = Some(Prize::FullHouse);
                                //println!("Full house!");
                                self.funds += 7 * self.bet_amount;
                            }
                            else if self.hand.is_flush(){
                                self.latest_prize = Some(Prize::Flush);
                                //println!("Flush!");
                                self.funds += 4 * self.bet_amount;
                            }
                            else if self.hand.is_straight(){
                                self.latest_prize = Some(Prize::Straight);
                                //println!("Straight!");
                                self.funds += 3 * self.bet_amount;
                            }
                            else if self.hand.is_three_of_a_kind(){
                                self.latest_prize = Some(Prize::ThreeOfAKind);
                                //println!("Three of a kind!");
                                self.funds += 2 * self.bet_amount;
                            }
                            else if self.hand.is_two_pairs(){
                                self.latest_prize = Some(Prize::TwoPairs);
                                //println!("Two pairs!");
                                self.funds += 2 * self.bet_amount;
                            }
                            else{
                                println!("No win");
                            }
                        }
                        GameState::PayOut =>{
                            
                            if self.funds == 0{
                                self.state = GameState::GameOver;
                            }
                            else{
                                self.state = GameState::Betting;
                                self.latest_prize = None;
                                self.round += 1;
                                self.reset_deck_and_hand();
                            }
                        }
                        _ =>{
                            
                        }
                    }
                }
                console::Key::Char('b') =>{
                    if self.state == GameState::Betting{
                        self.cycle_bet_amount();
                    }
                }
                console::Key::Char(' ') =>{
                    if self.state == GameState::HandSelection{
                        self.toggle_selection();
                    }
                }
                console::Key::ArrowLeft =>{
                    if self.state == GameState::HandSelection{
                        // move selector left
                        if self.selector > 0{
                            self.selector -= 1;
                        }
                    }
                }
                console::Key::ArrowRight =>{
                    if self.state == GameState::HandSelection{
                        // move selector right
                        if self.selector < 4{
                            self.selector += 1;
                        }
                    }
                }
                console::Key::Char('y') =>{
                    if self.state == GameState::GameOver{
                        self.reset_game();
                    }
                }
                console::Key::Char('n') =>{
                    if self.state == GameState::GameOver{
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    fn print_stats(&self){
        println!("Funds: {:<10}Bet: {:<10}Round: {:<10}", 
        self.funds, self.bet_amount, self.round,);
    }
}


fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    println!("Rust JokeriPokeri, a programming excercise");

    let mut game = JokeriPokeri::new();
    game.play();
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn straight(){
        let mut hand = Hand::new();
        let uni_spade = char::from_u32(0x2660).unwrap();
        let uni_hearts = char::from_u32(0x2661).unwrap();
        let uni_diamond = char::from_u32(0x2662).unwrap();
        let uni_clubs = char::from_u32(0x2663).unwrap();

        hand.cards[0] = Some(Card::new(2, uni_spade));
        hand.cards[1] = Some(Card::new(3, uni_hearts));
        hand.cards[2] = Some(Card::new(4, uni_clubs));
        hand.cards[3] = Some(Card::new(5, uni_clubs));
        hand.cards[4] = Some(Card::new(6, uni_clubs));

        assert_eq!(hand.is_straight(), true);

        hand.cards[0] = Some(Card::new(10, uni_spade));

        assert_eq!(hand.is_straight(), false);
    }
    #[test]
    fn flush(){
        let mut hand = Hand::new();
        let uni_spade = char::from_u32(0x2660).unwrap();
        let uni_hearts = char::from_u32(0x2661).unwrap();
        let uni_diamond = char::from_u32(0x2662).unwrap();
        let uni_clubs = char::from_u32(0x2663).unwrap();

        hand.cards[0] = Some(Card::new(10, uni_spade));
        hand.cards[1] = Some(Card::new(6, uni_spade));
        hand.cards[2] = Some(Card::new(4, uni_spade));
        hand.cards[3] = Some(Card::new(13, uni_spade));
        hand.cards[4] = Some(Card::new(6, uni_spade));

        assert_eq!(hand.is_flush(), true);

        hand.cards[0] = Some(Card::new(10, uni_spade));
        hand.cards[1] = Some(Card::new(6, uni_spade));
        hand.cards[2] = Some(Card::new(4, uni_hearts));
        hand.cards[3] = Some(Card::new(13, uni_spade));
        hand.cards[4] = Some(Card::new(6, uni_spade));
        
        assert_eq!(hand.is_flush(), false);
    }
    #[test]
    fn straight_flush(){
        let mut hand = Hand::new();
        let uni_spade = char::from_u32(0x2660).unwrap();
        let uni_hearts = char::from_u32(0x2661).unwrap();
        let uni_diamond = char::from_u32(0x2662).unwrap();
        let uni_clubs = char::from_u32(0x2663).unwrap();

        hand.cards[0] = Some(Card::new(2, uni_spade));
        hand.cards[1] = Some(Card::new(3, uni_spade));
        hand.cards[2] = Some(Card::new(4, uni_spade));
        hand.cards[3] = Some(Card::new(5, uni_spade));
        hand.cards[4] = Some(Card::new(6, uni_spade));

        assert_eq!(hand.is_straight_flush(), true);

        hand.cards[0] = Some(Card::new(2, uni_spade));
        hand.cards[1] = Some(Card::new(3, uni_spade));
        hand.cards[2] = Some(Card::new(4, uni_hearts));
        hand.cards[3] = Some(Card::new(5, uni_spade));
        hand.cards[4] = Some(Card::new(6, uni_spade));

        assert_eq!(hand.is_straight_flush(), false);

        hand.cards[0] = Some(Card::new(2, uni_spade));
        hand.cards[1] = Some(Card::new(3, uni_spade));
        hand.cards[2] = Some(Card::new(10, uni_spade));
        hand.cards[3] = Some(Card::new(5, uni_spade));
        hand.cards[4] = Some(Card::new(6, uni_spade));

        assert_eq!(hand.is_straight_flush(), false);
    }
    #[test]
    fn four_of_a_kind(){
        let mut hand = Hand::new();
        let uni_spade = char::from_u32(0x2660).unwrap();
        let uni_hearts = char::from_u32(0x2661).unwrap();
        let uni_diamond = char::from_u32(0x2662).unwrap();
        let uni_clubs = char::from_u32(0x2663).unwrap();

        hand.cards[0] = Some(Card::new(2, uni_spade));
        hand.cards[1] = Some(Card::new(2, uni_hearts));
        hand.cards[2] = Some(Card::new(2, uni_clubs));
        hand.cards[3] = Some(Card::new(2, uni_diamond));
        hand.cards[4] = Some(Card::new(6, uni_spade));

        assert_eq!(hand.is_four_of_a_kind(), true);

        hand.cards[2] = Some(Card::new(5, uni_clubs));

        assert_eq!(hand.is_four_of_a_kind(), false);
    }

    #[test]
    fn three_of_a_kind(){
        let mut hand = Hand::new();
        let uni_spade = char::from_u32(0x2660).unwrap();
        let uni_hearts = char::from_u32(0x2661).unwrap();
        let uni_diamond = char::from_u32(0x2662).unwrap();
        let uni_clubs = char::from_u32(0x2663).unwrap();

        hand.cards[0] = Some(Card::new(2, uni_spade));
        hand.cards[1] = Some(Card::new(2, uni_hearts));
        hand.cards[2] = Some(Card::new(2, uni_clubs));
        hand.cards[3] = Some(Card::new(12, uni_diamond));
        hand.cards[4] = Some(Card::new(6, uni_spade));

        assert_eq!(hand.is_three_of_a_kind(), true);

        hand.cards[2] = Some(Card::new(5, uni_clubs));

        assert_eq!(hand.is_three_of_a_kind(), false);
    }
    #[test]
    fn full_house(){
        let mut hand = Hand::new();
        let uni_spade = char::from_u32(0x2660).unwrap();
        let uni_hearts = char::from_u32(0x2661).unwrap();
        let uni_diamond = char::from_u32(0x2662).unwrap();
        let uni_clubs = char::from_u32(0x2663).unwrap();

        hand.cards[0] = Some(Card::new(2, uni_spade));
        hand.cards[1] = Some(Card::new(2, uni_hearts));
        hand.cards[2] = Some(Card::new(2, uni_clubs));
        hand.cards[3] = Some(Card::new(4, uni_diamond));
        hand.cards[4] = Some(Card::new(4, uni_spade));

        assert_eq!(hand.is_full_house(), true);

        hand.cards[4] = Some(Card::new(5, uni_clubs));

        assert_eq!(hand.is_full_house(), false);
    }
    #[test]
    fn two_pairs(){
        let mut hand = Hand::new();
        let uni_spade = char::from_u32(0x2660).unwrap();
        let uni_hearts = char::from_u32(0x2661).unwrap();
        let uni_diamond = char::from_u32(0x2662).unwrap();
        let uni_clubs = char::from_u32(0x2663).unwrap();

        hand.cards[0] = Some(Card::new(2, uni_spade));
        hand.cards[1] = Some(Card::new(2, uni_hearts));
        hand.cards[2] = Some(Card::new(4, uni_clubs));
        hand.cards[3] = Some(Card::new(4, uni_diamond));
        hand.cards[4] = Some(Card::new(6, uni_spade));

        assert_eq!(hand.is_two_pairs(), true);

        hand.cards[0] = Some(Card::new(7, uni_clubs));

        assert_eq!(hand.is_two_pairs(), false);
    }
    #[test]
    fn deal(){
        let mut game = JokeriPokeri::new();
        // hand must be empty before deal
        for option_card in &game.hand.cards{
            match option_card{
                Some(..) => {
                    panic!("Cards in hand before deal");
                }
                None => {}
            }
        }
        // has_nones
        assert_eq!(game.hand.has_nones(), true);

        let deck_len_before_deal = game.deck.cards.len();
        // deal
        game.deal();
        assert_eq!(game.hand.has_nones(), false);

        let deck_len_after_deal = game.deck.cards.len();
        assert_eq!(deck_len_before_deal - deck_len_after_deal, game.hand.cards.len());
        assert_eq!(game.discarded.len(), 0);
        
        game.selected[2] = true;
        game.selected[3] = true;

        game.deal();
        // discarding from hand
        assert_eq!(game.discarded.len(), game.hand.cards.len() - 2);
        let deck_len_after_hand_selection_deal = game.deck.cards.len();
        assert_eq!(deck_len_after_deal - deck_len_after_hand_selection_deal, game.hand.cards.len()-2);
    }
    #[test]
    fn reset_deck_and_hand(){
        let mut game = JokeriPokeri::new();
        let deck_size_before_deal = game.deck.cards.len();
        game.deal();
        game.selected[1] = true;
        game.selected[2] = true;
        game.deal();
        game.reset_deck_and_hand();
        assert_eq!(deck_size_before_deal, game.deck.cards.len());
        assert_eq!(game.discarded.len(), 0);
    }
}