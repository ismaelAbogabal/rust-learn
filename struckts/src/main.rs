

#[derive(Debug)]
struct User

{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User{
    fn new() -> User {
        User {
            active: false,
            username: String::from(""),
            email: String::from(""),
            sign_in_count: 0,
        }
    }


    fn set_active(&self, active: bool) {
        // self.active = active;

        println!("Active: {}", active);
        // User {
        //     active: self.active,
        //     username: self.username.clone(),
        //     email: self.email.clone(),
        //     sign_in_count: self.sign_in_count,
        // }
    }
}

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

 
    let o = Some(3).unwrap_or(0);

    let sum = x + y.unwrap_or(0);

}