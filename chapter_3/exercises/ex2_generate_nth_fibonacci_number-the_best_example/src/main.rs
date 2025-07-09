fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    // source: https://iteroni.com/watch?v=YgnhaFH6Bg8
    println!("{}", fibonacci(2));
    println!("{}", fibonacci(4));
    println!("{}", fibonacci(7));
}
