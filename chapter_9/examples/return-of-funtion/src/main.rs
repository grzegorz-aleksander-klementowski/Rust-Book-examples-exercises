use std::fs::{self, File, read_to_string};
use std::io::{self, Read};

fn czytaj_działalnik_najnajkrótsze() -> Result<String, io::Error> {
    fs::read_to_string("nazwa_użytkownika.txt")
}

fn czytaj_nazwę_użytkownika_najkrótsza() -> Result<String, io::Error> {
    let mut nazwa_użytkownika = String::new();
    File::open("nazwa_użytkownika.txt")?.read_to_string(&mut nazwa_użytkownika)?;
    Ok(nazwa_użytkownika)
}

fn czytaj_nazwę_użytkownika_z_działanika_z_decydajnikiem_znakiem_zapytania()
-> Result<String, io::Error> {
    let mut działanik_nazwa_użytkownika = File::open("nazwa_użytkownika.txt")?;
    let mut nazwa_użytkownika = String::new();

    działanik_nazwa_użytkownika.read_to_string(&mut nazwa_użytkownika)?;

    Ok(nazwa_użytkownika)
}

fn czytaj_nazwę_użytkownika_z_działalnika() -> Result<String, io::Error> {
    let wynik_czytaj_działalnik = File::open("nazwa_użytkownika.txt");

    let mut działalnik_nazwa_użytkownika = match wynik_czytaj_działalnik {
        Ok(działalnik) => działalnik,
        Err(błąd) => return Err(błąd),
    };

    let mut nazwa_użytkwnika = String::new();

    match działalnik_nazwa_użytkownika.read_to_string(&mut nazwa_użytkwnika) {
        Ok(_) => Ok(nazwa_użytkwnika),
        Err(błąd) => Err(błąd),
    }
}

#[allow(unused)]
fn main() {
    let wyniki = [
        czytaj_nazwę_użytkownika_z_działalnika(),
        czytaj_nazwę_użytkownika_z_działanika_z_decydajnikiem_znakiem_zapytania(),
        czytaj_nazwę_użytkownika_najkrótsza(),
        czytaj_nazwę_użytkownika_najkrótsza(),
    ];

    for (i, wynik) in wyniki.iter().enumerate() {
        println!("{i}.: {wynik:?}", i = i + 1);
    }
}
