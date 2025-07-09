fn main() {
    let owned_string = no_dangle();
    println!("Zwrócona wartość: {}", owned_string);
}

// despite it's an example from the Rust Book – we return String instead &String to avoid errors
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

