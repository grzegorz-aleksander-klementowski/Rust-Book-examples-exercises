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
    println!("HashMap 1: {wyniki:?}");

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

    // Adding a Key and Value Only If a Key Isn’t Present
    println!("Dodawanie klucza z wartością, tylko przykadku klucz jeszcze nie istnieje.");
    println!("Dodawanie Stelli Lubomierz (już istnieje)");
    wyniki
        .entry(String::from("Stella Lubomierz"))
        .or_insert(100);
    println!("Dodawanie „Orzeł Wojcieszów”, który jeszcze nie istnieje.");
    //let dru_orzel = String::from("Orzeł Wojcieszów");
    wyniki
        .entry(String::from("Orzeł Wojcieszów"))
        .or_insert(100);
    println!(
        "W „Stelli Lubomierz” nie powinna się zmienić punktacja ani nadpisać. Orzeł powinien być utworzony.\nWyniki: {wyniki:?}"
    );

    // Updating a value based on the old value
    println!("Aktualizowanie wartośći opierając się natarej wartości");
    let napis = "Witaj świecie przepiękny świecie";
    let mut map = HashMap::new();

    for słowo in napis.split_whitespace() {
        // jeśli NIE znalazł słowa takiego, to zapisuje i przypisuje wartość 0
        let licznik = map.entry(słowo).or_insert(0);
        // czy znalazł, czy nie zawsze będzie +1. A więc gdy nie znajdzie, będzie 1, a gdy będzie+1
        *licznik += 1;
    }
    println!("Ilość słów: {map:?}");
}
