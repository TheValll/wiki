struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let user3 = build_user("todo@todo.com".to_string(), "todo".to_string());
    println!(
        "{} {} {} {}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", origin.0, origin.1, origin.2);
}
