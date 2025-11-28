use std::fs::File;
use std::io::ErrorKind;

#[allow(unused)]
fn main() {
    let otwarty_plik = File::open("Witaj.txt").unwrap_or_else(|błąd_otwarcia_pliku| {
        if błąd_otwarcia_pliku.kind() == ErrorKind::NotFound {
            File::create("Witaj.txt").unwrap_or_else(|błąd_utworzenia_pliku| {
                panic!("Nie mogę utworzyć pliku z powodu {błąd_otwarcia_pliku:?}");
            })
        } else {
            panic!("Nie mogę otworzyć pliku z powodu: {błąd_otwarcia_pliku:?}");
        }
    });
}
