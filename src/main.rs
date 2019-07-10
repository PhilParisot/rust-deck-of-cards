#[derive(Copy, Clone, Debug)]
enum Suits {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}

#[derive(Copy, Clone, Debug)]
struct Card {
    card_num: u8,
    card_suit: Suits,
}

fn generate_deck() {
    let deck: [Option<Card>; 52] = [None; 52];

    for mut i in deck.iter() {
        i = &Some(Card {
            card_num: 1,
            card_suit: Suits::Hearts,
        });
    }

    for i in deck.iter() {
        println!("{:?}", i);
    }
}

fn main() {
    generate_deck();
}
