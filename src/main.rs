use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{char, env, io::{self, Write}};
use console::Term;



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
                self.cards.push(Card::new(u8::from(value), char::from_u32(character_code).unwrap()))
            }
        }
        // TODO: add 2 (and implement) joker cards
        // self.cards.push(Card::new(0, 'J'));
        // self.cards.push(Card::new(0, 'J'));
    }
}

enum WinCondition {
    //FiveOfAKind,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
    NoWin,
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
}

/// Struct to hold all of the game's data and functionality
struct JokeriPokeri{
    deck: Deck,
    hand: Hand,
    discarded: Vec<Card>,
    funds: u32,
    round: u32,
    bet_amount: u32,
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
        };
        return game;
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

    fn promt_and_input(&self)->String{
        self.print_prompt();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input
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

    fn print_prompt(&self){
        match self.state{
            GameState::Betting => {
                println!("Place your bet (1-100)");
            }
            GameState::HandSelection =>{
                println!("Select cards to hold (for example: 1 3 5)");
            }
            GameState::PayOut =>{
                println!("Input anything for a new round");
            }
            _=>{()}
        }
        print!(">");
        let _ = std::io::stdout().flush();
    }

    fn print_screen(&self){
        println!();
        self.print_stats();
        println!();
        println!();
        println!();
        match self.state{
            GameState::Betting=>{
                println!("b - cycle bet amount");
                println!("enter - start game");
            }
            GameState::HandSelection=>{
                println!("left/right - move selector");
                println!("space - select card");
                println!("enter - continue");
                self.hand.print();
                // print selected row
                for i in self.selected{
                    if i{
                        print!("HLD");
                    } else {
                        print!("   ");
                    }
                }
                //std::io::stdout().flush();
                println!();
                // print selector cursor position
                for i in 0..self.hand.cards.len(){
                    if self.selector == i{
                        print!(" ^ ");
                    } else{
                        print!("   ");
                    }
                }
                println!();

            }
            GameState::PayOut=>{
                if self.funds == 0{
                    println!("GAME OVER");
                    println!("OUT OF FUNDS");
                    println!("Continue to new game...");
                }
                self.hand.print();
            }
        }
    }

    /// The actual state machine. If input is valid, proceeds GameState and updates data accordingly.
    /// If invalid, returns early without modifying data.
    fn process(&mut self){
        match self.state{
            GameState::Betting =>{
                //println!("GameState: Betting");
                let input = self.promt_and_input();
                self.query_quit(&input);
                let bet_amount: u32 = match input.trim().parse::<u32>(){
                    Ok(amt) =>{
                        amt
                    }
                    Err(..) =>{
                        println!("Invalid input.");
                        return;
                    }
                };
    
                if bet_amount < 1 || bet_amount > 100{
                    println!("Invalid bet, must be between 1 and 100.");
                    return;
                }
                if self.funds < bet_amount{
                    println!("Insufficient funds!");
                    return;
                }
    
                // bet valid, proceed to next state
                self.funds -= bet_amount;
                self.bet_amount = bet_amount;
    
                self.deck.shuffle_deck();
                // draw 5 cards to hand
                // print cards in hand
                // println!("Cards in hand before deal:");
                // self.hand.print();
    
                for i in 0..5{
                    self.hand.cards[i] = Some(self.deck.cards.remove(0));
                }
    
                // println!("Cards after hand transfer:");
                // for card in &self.hand{
                //     card.print();
                // }
                self.state = GameState::HandSelection;
                // println!("Hand len at the end of betting: {}", self.hand.cards.len());
                // println!("End of betting.");
            }
            GameState::HandSelection =>{
                //println!("Hand len at the start of handselection {}", self.hand.cards.len());
                //println!("State: handselection");
                // print cards in hand
                //println!("Cards in hand before selection:");
                self.hand.print();
                // split input String
                let input = self.promt_and_input();
                let parts = input.trim().split(" ");

                // validate parts
                if parts.clone().count() > 5{
                    println!("Invalid input.");
                    return;
                }
                // check that each part parses into u8
                let mut selections: Vec<u8> = Vec::new();
                for part in parts{
                    match part.parse(){
                        Ok( value ) =>{
                            if !selections.contains(&value){
                                selections.push(value);
                            }
                        }
                        Err(_)=>{
                            return;
                        }
                    }
                }
                // check range of each selection 1..=5
                for selection in selections.clone(){
                    if selection < 1 || selection > 5{
                        println!("Card selection out of range (1..=5)");
                        return;
                    }
                }

                // Selections are valid.
                // Discard unselected cards,
                // Draw new in place of them.
                for i in 0..self.hand.cards.len(){
                    let selection_value = u8::try_from(i+1).unwrap();
                    if !selections.contains(&selection_value){
                        // card not selected, discard and draw new from deck
                        let element_to_discard = self.hand.cards[i].take();
                        match element_to_discard{
                            Some(card_to_discard)=>{
                                self.discarded.push(card_to_discard);
                                self.hand.cards[i] = Some(self.deck.cards.remove(0));
                            }
                            None=>{
                                // Something went wrong,
                                // revert to prevent a Card from getting destroyed.
                                self.hand.cards[i] = element_to_discard;
                            }
                        }
                    }
                }
                // Proceed to next state.
                self.state = GameState::PayOut;
            }
            GameState::PayOut=>{
                self.hand.print();

                // check if hand meets win conditions
                // maintain order from highest to lowest
                if self.hand.is_straight_flush(){
                    println!("Straight flush!");
                    self.funds += 40 * self.bet_amount;
                }
                else if self.hand.is_four_of_a_kind(){
                    println!("Four of a kind!");
                    self.funds += 15 * self.bet_amount;
                }
                else if self.hand.is_full_house(){
                    println!("Full house!");
                    self.funds += 7 * self.bet_amount;
                }
                else if self.hand.is_flush(){
                    println!("Flush!");
                    self.funds += 4 * self.bet_amount;
                }
                else if self.hand.is_straight(){
                    println!("Straight!");
                    self.funds += 3 * self.bet_amount;
                }
                else if self.hand.is_three_of_a_kind(){
                    println!("Three of a kind!");
                    self.funds += 2 * self.bet_amount;
                }
                else if self.hand.is_two_pairs(){
                    println!("Two pairs!");
                    self.funds += 2 * self.bet_amount;
                }
                else{
                    println!("No win :(");
                }

                // transfer cards from hand back to deck
                for i in 0..self.hand.cards.len(){
                    match self.hand.cards[i].take(){
                        Some(card)=>{
                            self.deck.cards.push(card);
                        }
                        None=>{}
                    }
                }
                // transfer cards from discarded back to deck
                for i in 0..self.discarded.len(){
                    self.deck.cards.push(self.discarded.remove(0));
                }
                self.round += 1;
                self.state = GameState::Betting;
                //let input = self.promt_and_input();
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
                            self.hand.print();
                            self.deal();
                            // check wins
                            if self.hand.is_straight_flush(){
                                println!("Straight flush!");
                                self.funds += 40 * self.bet_amount;
                            }
                            else if self.hand.is_four_of_a_kind(){
                                println!("Four of a kind!");
                                self.funds += 15 * self.bet_amount;
                            }
                            else if self.hand.is_full_house(){
                                println!("Full house!");
                                self.funds += 7 * self.bet_amount;
                            }
                            else if self.hand.is_flush(){
                                println!("Flush!");
                                self.funds += 4 * self.bet_amount;
                            }
                            else if self.hand.is_straight(){
                                println!("Straight!");
                                self.funds += 3 * self.bet_amount;
                            }
                            else if self.hand.is_three_of_a_kind(){
                                println!("Three of a kind!");
                                self.funds += 2 * self.bet_amount;
                            }
                            else if self.hand.is_two_pairs(){
                                println!("Two pairs!");
                                self.funds += 2 * self.bet_amount;
                            }
                            else{
                                println!("No win :(");
                            }
                            self.state = GameState::PayOut;

                            
                        }
                        GameState::PayOut =>{
                            // confirm new round
                            //self.print_prompt();
                            self.state = GameState::Betting;
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
                _ => {}
            }


            // if let Ok(character) = stdout.read_char(){
            //     match character {
            //         'q' | 'Q' => {
            //             self.playing = false;
            //             break;
            //         }
            //         'b' | 'B' =>{
            //             if self.state == GameState::Betting{
            //                 if self.bet_amount < 100{
            //                     self.bet_amount += 20;
            //                 }
            //                 else {
            //                     self.bet_amount = 20;
            //                 }
            //             }
            //         }

            //         _ => {
            //             println!("{}", character);
            //         }
            //     }
            // }
            

            // read user input
            //self.print_stats();
            //self.process();
        }
    }

    fn print_stats(&self){
        println!("Funds: {}", self.funds);
        println!("Round: {}", self.round);
        println!("Bet: {}", self.bet_amount);
    }
}


fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    println!("Rust JokeriPokeri, a Pyrdelic excercise");

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