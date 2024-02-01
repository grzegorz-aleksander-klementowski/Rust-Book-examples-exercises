use std::io;

fn main() {
    
    let mut fibonaci_number: u64   = 0;
    let mut fibonaci_number_1: u64 = 0;
    let mut fibonaci_number_2: u64 = 1; 
    let mut counter: u64 = 1;
    //let mut fibonaci_numer_wanted  = 1;

    println!("Enter Fibonacci number: ");
    let mut fibonaci_numer_wanted = String::new();
    io::stdin().read_line(&mut fibonaci_numer_wanted).expect("Failed to read line.");
    let fibonaci_numer_wanted = fibonaci_numer_wanted.trim().parse::<u64>().expect("E: Invalid type numer.");



    while counter <= fibonaci_numer_wanted {

        if fibonaci_numer_wanted == counter { println!("{fibonaci_number}"); }
        fibonaci_number   = fibonaci_number_1 + fibonaci_number_2;
        fibonaci_number_1 = fibonaci_number_2;
        fibonaci_number_2 = fibonaci_number;
        
        counter += 1;
    }


}
