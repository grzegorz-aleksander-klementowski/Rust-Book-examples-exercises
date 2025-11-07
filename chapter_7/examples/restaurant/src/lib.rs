// answers of the quis (https://rust-book.cs.brown.edu/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
// FIRST TEST:
// 1. crate;
// 2. Do NOT compile.
// 3. he output of this program will be: b
// SECOND TEST:
// 1. c2;
// 2. Do NOT compile.

#![allow(unused_variables)]

// Defined API
pub use crate::izba_gościnna::hosting;

#[path = "izba_gościnna.rs"]
mod izba_gościnna;

fn dostarczać_zamówienie() {}

// added as example to fullfill `mod back_of_house`
#[allow(dead_code)]
mod zaplecze {
    pub enum Przystawka {
        Polewka,
        Sałatka,
    }
    pub struct Śniadanie {
        pub zapiekanka: String,
        owoce_porokowe: String,
    }

    impl Śniadanie {
        pub fn lato(zapiekanka: &str) -> Śniadanie {
            Śniadanie {
                zapiekanka: String::from(zapiekanka),
                owoce_porokowe: String::from("brzoskwinie"),
            }
        }
    }

    fn popraw_nieprawidłową_kolejność() {
        zamówienie_kuchni();
        super::dostarczać_zamówienie(); // Use a function for the parrent module (crate)
    }
    fn zamówienie_kuchni() {}
}

// Extended example thus mentioned in the book
#[allow(dead_code)]
mod doświadczenia_usługobiorcy {
    pub fn jadanie_w_gospodzie() {
        // Ścieżka całkowita (absolutna, ang. absolute path)
        // crate::izba_gościnna::hosting::add_to_waitlist();

        //using path by `use` keyword;
        use crate::izba_gościnna::hosting;
        hosting::add_to_waitlist();

        // Ścieżka pokrewna (ang. relative path)
        // izba_gościnna::hosting::add_to_waitlist(); // if not in `customer_experience` module

        // Zamawianie śniadania z ryżową zapiekanką
        let mut posiłek = super::zaplecze::Śniadanie::lato("Żytnią");
        // Zmieniamy zdanie na temat rodzaju pieczywa
        posiłek.zapiekanka = String::from("Pszenica");
        println!("Poproszę {} zapiekankę", posiłek.zapiekanka);

        // We can't assign `owoce_porokowe` to `posiłek` as `owoce_porokowe` is a private field.
        // posiłek.owoce_porokowe = String::from("jagody");

        let zamówienie1 = super::zaplecze::Przystawka::Polewka;
        let zamówienie2 = super::zaplecze::Przystawka::Sałatka;
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
