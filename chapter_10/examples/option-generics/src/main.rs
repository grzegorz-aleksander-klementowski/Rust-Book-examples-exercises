// A Polish Example of option using genrics
// Wybór → option
// `Istnieje` → Some
// `Brak` → None
enum Możność<T> {
    Istnieje(T),
    Brak,
}

fn main() {
    let my_int = Możność::Istnieje(5);
    let my_flot = Możność::Istnieje(5.0);
}
