fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    change(r1);

    println!("output: {}", r1);


    println!("{}", r1);
    
    println!("{}", r1);
    let r2 = &mut s;

    println!("{}", r2);


}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
