use rand::{random, Rng};

pub mod garden;

pub fn main() {
    /***/

    for _ in 0..1000 {
        let random_number = rand::thread_rng().gen_range(0..=i32::MAX);
        print!("{random_number} ");
    }
}
