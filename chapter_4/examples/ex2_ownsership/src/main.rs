fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // s1 traci dostęp do swoich danych

    println!("The length of '{}' is {}.", s2, len); // używamy s2, ponieważ ma teraz własność danych. NIE MOŻNA WYPISAĆ S1!!! DLATEGO POTRZEBNE SĄ REFERENCJE.

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // obliczamy długość

    (s, length) // zwracamy String i jego długość
}

