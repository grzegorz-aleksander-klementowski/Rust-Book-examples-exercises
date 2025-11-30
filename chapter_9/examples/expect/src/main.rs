use std::fs::File;

#[allow(unused)]
fn main() {
    let odczytaj_wiadomość = File::open("Witaj.txt").expect("Nie ma takiego pliku!");
}
