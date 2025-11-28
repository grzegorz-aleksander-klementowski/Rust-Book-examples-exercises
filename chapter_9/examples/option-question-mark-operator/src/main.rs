fn zwróć_ostatni_znak_p_linii(napis: &str) -> Option<char> {
    napis.lines().next()?.chars().last()
}

fn main() {
    let napis = "napis";
    let znak = zwróć_ostatni_znak_p_linii(napis);
    println!("{znak:?}");
}
