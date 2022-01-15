// User - defines the User data type.
#[derive(Debug, PartialEq)]
struct User {
    active: bool,
    sign_in_count: u64,
    username: String,
    email: String,
}

fn main() {
    println!("structs are lit bruh");

    let user_one = User {
        email: String::from("brendi@brendi.com"),
        username: String::from("brendi boi"),
        active: true,
        sign_in_count: 2,
    };
    println!("\n{user_one:#?}");

    let user_two = build_user(String::from("brendiboi@brendi.com"), String::from("brendo"));
    println!("{user_two:#?}");

    // struct update syntax

    let user_three = User {
        email: String::from("another@example.com"),
        username: String::from("brendoboyo"),
        ..user_one
    };
    println!("{user_three:#?}");
}

// build_user - builds a user struct from input args. -> User struct.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
