//having only `hosting` module public, the contents of hosting are still private; making the module public doesn’t make its contents public
pub mod hosting;

#[allow(dead_code)]
mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payments() {}
}
