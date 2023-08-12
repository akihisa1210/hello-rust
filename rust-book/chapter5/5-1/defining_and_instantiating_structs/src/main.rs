fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let mut black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    black.0 = 1;

    let subject = AlwaysEqual;

    let mut p = Point2 { x: 0, y: 0 };
    let x = &mut p.x;
    *x += 1;
    println!("{}, {}", p.x, p.y);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

struct Point2 {
    x: i32,
    y: i32,
}
