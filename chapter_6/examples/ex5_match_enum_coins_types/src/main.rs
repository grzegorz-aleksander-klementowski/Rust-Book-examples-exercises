#[derive(Debug)]
enum UsStates {
    Alaska,
    Alabama,
    California,
    Texsas,
    // -- snip --
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cent(Coin::Penny);
    value_in_cent(Coin::Quarter(UsStates::Alaska));
}
