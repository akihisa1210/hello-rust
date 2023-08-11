fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("first word: {}", word);

    let s = String::from("hello world");
    let slice = &s[0..5];
    println!("slice: {}", slice);

    let len = s.len();
    let slice = &s[3..len];
    println!("slice: {}", slice);
    let slice = &s[3..];
    println!("slice: {}", slice);

    let s = String::from("hello world");
    let word = first_word_slice(&s);
    println!("first word: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
