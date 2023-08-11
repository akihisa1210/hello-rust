fn main() {
    let s = return_a_string();
    println!("{}", s);

    let name = vec![String::from("Ferris"), String::from("Jr.")];
    println!("{}", stringify_name_with_title(&name));

    let mut dst = vec![String::from("Ferris"), String::from("BBB")];
    let src = vec![
        String::from("AAAAAAAA"),
        String::from("Rust"),
        String::from("Jr."),
    ];
    add_big_strings(&mut dst, &src);
    println!("{:?}", dst);

    out_of_a_collection();

    // let mut name = (String::from("Ferris"), String::from("Rustacean"));
    // let first = get_first(&name);
    // name.1.push_str(", Esq.");
    // println!("{} {}", first, name.1);
}

// Fixing an Unsafe Program: Returning a Reference to the Stack

// unsafe
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

// safe
// move ownership fo the string out of the function
fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}

// safe
// return a string literal
// fn return_a_string() -> &'static str {
//     "Hello world"
// }

// safe
// defer borrow-checking to runtime
// use std::rc::Rc;
// fn return_a_string() -> Rc<String> {
//     let s = Rc::new(String::from("Hello world"));
//     Rc::clone(&s)
// }

// safe
// fn return_a_string(output: &mut String) {
//     output.replace_range(.., "Hello world");
// }

// Fixing an Unsafe Program: Not Enough Permissions

// unsafe
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

// safe
// clone the input
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     let mut name_clone = name.clone();
//     name_clone.push(String::from("Esq."));
//     let full = name_clone.join(" ");
//     full
// }

// safe
// avoid unnecessary copy
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

// Fixing an Unsafe Program: Aliasing and Mutating a Data Structure

// unsafe
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();

//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }

// safe
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

// Fixing an Unsafe Program: Copying vs. Moving Out of a Collection

// unsafe
// fn out_of_a_collection() {
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let s_ref: &String = &v[0];
//     let s: String = *s_ref;
// }

// safe
// fn out_of_a_collection() {
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let s_ref: &String = &v[0];
//     println!("{s_ref}");
// }

// safe
// fn out_of_a_collection() {
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let mut s: String = v[0].clone();
//     s.push('!');
//     println!("{s}");
// }

// safe
fn out_of_a_collection() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}

// Fixing a Safe Program: Mutating Different Tuple Fields

// unsafe
// fn get_first(name: &(String, String)) -> &String {
//     &name.0
// }

// Fixing a Safe Program: Mutating Different Array Elements
