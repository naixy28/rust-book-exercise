fn main() {
    println!("Hello, world!");

    let v = value_in_cents(Coin::Quarter);

    println!("v: {}", v);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,  // 1 cent
        Coin::Nickel => 5, // 5 cents
        Coin::Dime => 10,  // 10 cents
        _other => 0,       // 25 cents
    }
}
