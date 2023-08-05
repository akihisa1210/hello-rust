use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {guess}",);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {x}, y: {y}");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!(
        "sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, truncated: {truncated}, remainder: {remainder}",
    );

    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}",);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months: {}", months[0]);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
