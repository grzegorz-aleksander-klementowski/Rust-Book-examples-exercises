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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
