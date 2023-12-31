fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of y in the inner scope is: {x}");
    }
    println!("The value of y is: {x}");

    let spaces = "   ";
    println!("Spaces: {spaces}");
    let spaces = spaces.len();
    println!("Spaces: {spaces}")
}
