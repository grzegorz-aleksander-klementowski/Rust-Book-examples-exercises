use std::fmt::format;

#[allow(unused_variables)]
fn main() {
    // Similarity with vectors
    let mut s = String::new();
    s.push_str("przykład");

    // It transform slice type string (`&str`) into String type
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");

    // The operation is possible because `s2` isn't an object as `s1`
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // It is possbile to add a char to a string
    let mut s = String::from("Ro");
    s.push('d');
    println!("{s}");

    // Concatanation using `+` operator (works for many Strings)
    let s1 = String::from("Perun i ");
    // let s1_ref = &s1; `fn add(self, s: &str) -> String`–requires String, not &String
    let s2 = String::from("Rod");
    let s3 = s1 + &s2; // ← &s2 as a reference `&String` (it coerces the &String argument into a &str)

    // Better wersion of Concatanations of many Strings – `format!()` macro.
    let s1 = String::from("Perun");
    let s2 = String::from("Rod");
    let s3 = String::from("Wales");
    let s = format!("{s1}, {s2} oraz {s3}"); // format! macro uses references so that this call doesn’t take ownership of any of its parameters.

    // Iterating over String (the correct way – indexing doesn't wokr)
    for c in "नमस्ते".chars() {
        println!("{c}");
    }

    for c in "Зд".chars() {
        println!("{c}");
    }

    // Printing bytes of a String
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
