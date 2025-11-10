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

    // Concatanation using `+` operator
    let s1 = String::from("Perun i ");
    // let s1_ref = &s1; `fn add(self, s: &str) -> String`–requires String, not &String
    let s2 = String::from("Rod");
    let s3 = s1 + &s2; // ← &s2 as a reference `&String` (it coerces the &String argument into a &str)
}
