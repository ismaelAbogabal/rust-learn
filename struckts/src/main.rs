#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn new() -> User {
        User {
            active: false,
            username: String::from(""),
            email: String::from(""),
            sign_in_count: 0,
        }
    }

    fn set_active(&self, active: bool) {
        println!("Active: {}", active);
    }
}

fn main() {
    let mut user = User {
        sign_in_count: 0,
        email: String::from("gabalismael@gmail.com"),
        username: "ismael".to_string(),
        active: false,
    };

    user.set_active(true);
    user.email = "ahmed".to_string();

    dbg!(user);
}
