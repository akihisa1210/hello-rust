fn read(y: bool) {
    if y {
        println!("y is true");
    }
}

fn main() {
    let x = true;
    read(x);

    let n = 5;
    let y = plus_one(n);
    println!("The value of y is: {y}");

    // let a = [0; 1_000_000];
    let a = Box::new([0; 1_000_000]);
    let b = a;
    println!("b[0] = {}", b[0]);

    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}");
    println!("{first}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
