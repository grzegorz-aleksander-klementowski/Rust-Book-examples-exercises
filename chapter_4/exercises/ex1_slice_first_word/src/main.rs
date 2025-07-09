fn main() {
    let my_string = String::from("hello world");
    println!("First world: {}", first_word(&my_string));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
