fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("{}", s);

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    *x += 1;

    let r1: &Box<i32> = &x;
    let b: i32 = **r1;

    let r2: &i32 = &*x;
    let c: i32 = *r2;

    println!("{} {} {}", a, b, c);

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len = str::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len, s_len2);

    let mut vec: Vec<i32> = vec![1, 2, 3];
    vec.push(4);
    for i in &vec {
        println!("{}", i);
    }

    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &vec[2];
    println!("Third element is {}", *num);
    vec.push(4);

    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);

    let mut c1 = vec!['a'];
    let c1 = &mut c1;
    ascii_capitalize(c1);
    println!("c1 is now {:?}", c1);

    let mut c2 = vec!['B'];
    let c2 = &mut c2;
    ascii_capitalize(c2);
    println!("c2 is now {:?}", c2);

    let s = String::from("Hello");
    let v = &vec![s];
    println!("{}", first(v));
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized!: {:?}", v);
    }
}

fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    return s_ref;
}

fn first_or(strings: &Vec<String>, default: &String) -> &String {
    if strings.len() > 0 {
        &strings[0]
    } else {
        default
    }
}
