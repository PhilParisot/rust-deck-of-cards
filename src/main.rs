use std::io;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Suits {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Card {
    card_num: u8,
    card_suit: Suits,
}

impl Card {
    fn generate_deck() -> [Option<Card>; 52] {
        let mut deck: [Option<Card>; 52] = [None; 52];

        let mut temp_deck_index = 0;

        for i in 1..14 {
            for j in 0..4 {
                match j {
                    0 => {
                        deck[temp_deck_index] = Some(Card {
                            card_num: i,
                            card_suit: Suits::Hearts,
                        })
                    }
                    1 => {
                        deck[temp_deck_index] = Some(Card {
                            card_num: i,
                            card_suit: Suits::Spades,
                        })
                    }
                    2 => {
                        deck[temp_deck_index] = Some(Card {
                            card_num: i,
                            card_suit: Suits::Clubs,
                        })
                    }
                    3 => {
                        deck[temp_deck_index] = Some(Card {
                            card_num: i,
                            card_suit: Suits::Diamonds,
                        })
                    }
                    _ => println!("Error in iterator"),
                }
                temp_deck_index += 1;
            }
        }

        for i in deck.iter() {
            println!("{:#?}", i.unwrap());
        }

        deck
    }

    fn match_card(self, deck: &[Option<Card>]) -> bool {
        for i in deck.iter() {
            if self == i.unwrap() {
                println!("\nFound card in deck!\nCard found is {:#?}\n", i.unwrap());
                return true;
            }
        }
        println!("\nCould not find card in deck\n");
        false
    }
}

fn main() {
    let deck = Card::generate_deck();
    let mut user_card_number = String::new();
    let mut user_suit = String::new();

    println!("\nPlease enter card number: ");
    io::stdin()
        .read_line(&mut user_card_number)
        .expect("Failed to read line");
    let user_card_number = user_card_number.trim();

    println!("\nPlease pick a suit (1 for Hearts, 2 for Spades, 3 for Clubs, 4 for Diamonds): ");
    io::stdin()
        .read_line(&mut user_suit)
        .expect("Failed to read line");
    let user_suit = user_suit.trim();

    let user_suit = match user_suit.parse().unwrap() {
        1 => Suits::Hearts,
        2 => Suits::Spades,
        3 => Suits::Clubs,
        4 => Suits::Diamonds,
        _ => panic!("Invalid entry for Suit: {:?}!", user_suit),
    };

    let user_card = Card {
        card_num: user_card_number.parse().unwrap(),
        card_suit: user_suit,
    };

    user_card.match_card(&deck);
}
