use std::{fs::File, io::ErrorKind};

#[allow(unused_variables)]
fn main() {
    let wynik_weź_plik = std::fs::File::open("Mądrusia!");

    let wynik_weź_plik = match wynik_weź_plik {
        Ok(plik) => plik,
        Err(błąd) => match błąd.kind() {
            ErrorKind::NotFound => match File::create("Witaj.txt") {
                Ok(new_plik) => new_plik,
                Err(błąd_utworzenia) => {
                    panic!("Nie można utworzyć pliku, z powodu: {błąd_utworzenia:?}")
                }
            },
            _ => panic!("Nie można otworzyć pliku! Z powodu: {błąd:?}"),
        },
    };
}
