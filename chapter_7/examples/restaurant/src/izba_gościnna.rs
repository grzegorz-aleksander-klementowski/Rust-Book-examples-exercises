#[allow(dead_code)]
pub mod hosting {
    //having only `hosting` module public, the contents of hosting are still private; making the module public doesn’t make its contents public
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
}

#[allow(dead_code)]
mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payments() {}
}
