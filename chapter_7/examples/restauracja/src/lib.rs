#[allow(dead_code)]
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payments() {}
    }
}

// added as example to fullfill `mod back_of_house`
#[allow(dead_code)]
mod back_of_house {
    mod cooking {
        fn receive_orders() {}
        fn prepearing_stuff() {}
        fn cooking() {}
        fn washing_dishes() {}
    }

    mod managing {
        fn sending_payments() {}
        fn calclulating_costs() {}
        fn employing() {}
    }
}

pub fn eat_at_restaurant() {
    // Ścieżka całkowita (absolutna, ang. absolute path)
    crate::front_of_house::hosting::add_to_waitlist();

    // Ścieżka pokrewna (ang. relative path)
    front_of_house::hosting::add_to_waitlist();
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
