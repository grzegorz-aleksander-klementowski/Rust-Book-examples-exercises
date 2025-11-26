#[allow(unused_variables)]
fn main() {
    let wynik_weź_plik = std::fs::File::open("Mądrusia!");

    let wynik_weź_plik = match wynik_weź_plik {
        Ok(plik) => plik,
        Err(błąd) => panic!("Nie można otworzyć pliku! Z powodu: {błąd:?}"),
    };
}
