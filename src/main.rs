use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};



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
            for character_code in 0x2660..=0x2665{
                self.cards.push(Card::new(u8::from(value), char::from_u32(character_code).unwrap()))
            }
        }
        // add 2 joker cards
        self.cards.push(Card::new(0, 'J'));
        self.cards.push(Card::new(0, 'J'));
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
                Some(_)=>{

                }
                None=>{
                    return true
                }
            }
        }
        return false
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
    playing: bool
}
impl JokeriPokeri{
    fn new()->Self{
        let mut game: JokeriPokeri = JokeriPokeri { 
            deck: Deck::new(), 
            hand: Hand::new(),
            discarded: Vec::new(),
            funds: 100, 
            round: 0, 
            bet_amount: 0,
            latest_payout: 0,
            state: GameState::Betting,
            playing: true, 
        };
        return game
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
                println!("PayOut prompt");
            }
            _=>{()}
        }
        print!(">");
        let _ = std::io::stdout().flush();
    }

    /// The actual state machine. If input is valid, changes proceeds GameState and updates data accordingly.
    /// If invalid, returns early without modifying data.
    fn process(&mut self){
        match self.state{
            GameState::Betting =>{
                //println!("GameState: Betting");
                let input = self.promt_and_input();
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
                let input = self.promt_and_input();
            }   
        }
    }

    pub fn play(&mut self){

        let mut playing: bool = true;
        
        while playing {

            // read user input
            self.print_stats();
            self.process();
        }
        
    }

    fn print_stats(&self){
        println!("Funds: {}", self.funds);
        println!("Round: {}", self.round);
        println!("Bet: {}", self.bet_amount);
    }
}


fn main() {
    println!("Rust JokeriPokeri, a Pyrdelic excercise");

    let mut game = JokeriPokeri::new();
    game.play();
}
