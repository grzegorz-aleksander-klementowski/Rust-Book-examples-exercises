use std::collections::HashMap;

fn main() {
    let mut wyniki = HashMap::new();

    wyniki.insert(String::from("Stella Lubomierz"), 10);
    wyniki.insert(String::from("Orzeł Klementowice"), 50);
    // println!("HashMap 1: {:?}", X?);

    let wybrana_drużyna = String::from("Stella Lubomierz");
    let wybierz_wynik = wyniki.get(&wybrana_drużyna).copied().unwrap();
    println!("Wynik Stelli Lubomierz to: {:?}", wybierz_wynik);
}
