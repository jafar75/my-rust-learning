
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    println!("Hello, world!");

    let some_coin = Coin::Penny;
    let cents : u8 = value_in_cents(some_coin);
    let test_quarter = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", cents);
    println!("{}", test_quarter);
}
