/*
  OPTIONS AND ENUMS
*/
fn main() {
    let cents = Coin::value_in_cents(Coin::Quarter(USState::California));
    println!("{cents:?}");

    // Options
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{six:?}, {none:?}");

    if let Some(3) = five {
        println!("three")
    }
}

#[derive(Debug)]
enum USState {
    Oregon,
    Vermont,
    Arizona,
    Washington,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

impl Coin {
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}");
                25
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None, // underscore matches anything that ISNT the exact thing we want.
    }
}
