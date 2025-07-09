fn main() {
    let s = "hello"; // This is our string
    let bytes = s.as_bytes(); // Convert the string into ASCII codes

    println!("ASCII codes of '{}':", s);
    for &byte in bytes {
        println!("{}", byte);
    }
}
