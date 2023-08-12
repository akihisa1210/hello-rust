fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Max: {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Max: {}", max);
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(String),
    }

    let mut count = 0;
    let coin = Coin::Quarter(String::from("New York"));
    if let Coin::Quarter(s) = coin {
        println!("State quarter from {:?}!", s);
    } else {
        count += 1;
    }
}
