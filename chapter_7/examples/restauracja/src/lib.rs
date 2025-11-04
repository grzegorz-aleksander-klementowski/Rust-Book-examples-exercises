// answers of the quis (https://rust-book.cs.brown.edu/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
// 1. crate;
// 2. This program does not compile.
// 3. he output of this program will be: b

#[allow(dead_code)]
mod front_of_house {
    //having only `hosting` module public, the contents of hosting are still private; making the module public doesn’t make its contents public
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payments() {}
    }
}

fn dostarczać_zamówienie() {}

// added as example to fullfill `mod back_of_house`
#[allow(dead_code)]
mod zaplecze {
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

pub fn jadanie_w_gospodzie() {
    // Zamawianie śniadania z ryżową zapiekanką
    let mut posiłek = zaplecze::Śniadanie::lato("Żytnią");
    // Zmieniamy zdanie na temat rodzaju pieczywa
    posiłek.zapiekanka = String::from("Pszenica");
    println!("Poproszę {} zapiekankę", posiłek.zapiekanka);

    //posiłek.zapiekanka = String::from("jagody");
}

// Extended example thus mentioned in the book
#[allow(dead_code)]
mod customer_experience {
    pub fn eat_at_restaurant() {
        // Ścieżka całkowita (absolutna, ang. absolute path)
        crate::front_of_house::hosting::add_to_waitlist();

        // Ścieżka pokrewna (ang. relative path)
        // Front_of_house::hosting::add_to_waitlist(); // if not in `customer_experience` module
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
