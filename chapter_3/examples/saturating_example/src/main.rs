fn main() {
    let number1: u8 = 250; // This is like our cup that's almost full.
    let number2: u8 = 10;  // We want to add this much water.

    // Let's try to add them using saturating_add
    let result = number1.saturating_add(number2);

    println!("The result is: {}", result); // This will show us what's in our cup.
}



