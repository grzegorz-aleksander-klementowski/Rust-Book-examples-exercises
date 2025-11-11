use std::collections::HashMap;

fn main() {
    // Creating a HashMap
    println!("Tworzenie HashMap.");
    let mut wyniki = HashMap::new();

    // Entering values to the HashMap
    println!("Wprowadzanie wartości do HashMap.");
    let nazwa_drużyny_1 = String::from("Stella Lubomierz");
    let wynik_drużyny_1 = 10;
    let wynik_drużyny_2 = 50;
    let nazwa_drużyny_2 = String::from("Orzeł Klementowice");
    wyniki.insert(nazwa_drużyny_1, wynik_drużyny_1);
    wyniki.insert(nazwa_drużyny_2, wynik_drużyny_2);
    // println!("HashMap 1: {:?}", X?);

    // Getting a value
    println!("Pobieranie pojedyńczej wartości.");
    let wybrana_drużyna = String::from("Stella Lubomierz");
    // wybierając odnośnik &wyniki pozwalamy dalej używać bezpośrednio `wyniki`
    let wybierz_wynik = &wyniki.get(&wybrana_drużyna).copied().unwrap_or(0);
    println!("Wynik Stelli Lubomierz to: {:?}", wybierz_wynik);

    // Getting a value in a loop
    println!("Pobieranie wartości w pętli:");
    for (klucz, wartość) in &wyniki {
        println!("{klucz}: {wartość}");
    }

    // Ownership behavior in HashMaps
    println!("Zachowanie własności w HashMapach.");
    // println!("Własność Łancuchów: {nazwa_drużyny_1}");
    println!(
        "Nazwa drużyny (rodzaj: łańcuch) został przeniesiony do HashMap. Chyba, żeby użyto odnośnika."
    );
    println!("Liczba (wynik) została skopiowana. Wynik wynik_drużyny_1: {wynik_drużyny_1}");
    println!("wynik_drużyny_2: {wynik_drużyny_2}");

    // Overwriting a Value – Replacing a value stored with a particular key
    println!("Nadpisywanie wartości już istniejącego klucza.");
    println!("Wynik przed napisywaniem drużyny Stella Lubomierz: {wybierz_wynik}");
    println!("Nadpisuje wynik klucza „Stella Lubomierz.");
    wyniki.insert(String::from("Stella Lubomierz"), 25);
    let po_nadpisaniu_lubo = &wyniki.get(&wybrana_drużyna).copied().unwrap_or(0);
    println!("Wynik po nadpisaniu drużyny Stella Lubomierz: {po_nadpisaniu_lubo}");
}
