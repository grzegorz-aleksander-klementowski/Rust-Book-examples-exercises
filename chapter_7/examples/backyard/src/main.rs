pub mod garden;

use crate::garden::vegetables;

fn main() {
    let plant = vegetables::Tomato(42);
    println!("I'm growing {plant:?}!");
}
