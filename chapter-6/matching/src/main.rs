#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let value = value_in_cents(&Coin::Dime);
    println!("Value of a dime is {value}");

    let value = value_in_cents(&Coin::Quarter(UsState::Alaska));
    println!("Value of a quarter in alaska is {value}");

    let optional_none: Option<i32> = Option::None;
    let result = plus_one(optional_none);
    println!("Result of plus one on None is {:?}", result);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25 + us_state_coin_tax(state),
    }
}

fn us_state_coin_tax(us_state: &UsState) -> u8 {
    match us_state {
        UsState::Alabama => 1,
        UsState::Alaska => 17,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/* This will not compile as we need to handle all cases
fn invalid_plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
*/
