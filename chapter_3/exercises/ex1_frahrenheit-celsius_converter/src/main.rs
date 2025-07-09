use ::std::io;

fn main() {
    println!(
        "Exercise 1 of Rust Book example: \"Convert temperatures between Fahrenheit and Celsius.\""
    );

    let choose = choose_convert();

    println!("Enter value conver: ");

    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line.");

    let value: f32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please enter a valid number. ");
            return;
        }
    };

    if choose {
        println!(
            "Fahrenheit to celsius: {}",
            convert_fahrenheit_to_celsius(value)
        )
    } else {
        println!(
            "Celsius to Frahrenheit: {}",
            convert_celsius_to_fahrenheit(value)
        )
    }
}

fn convert_fahrenheit_to_celsius(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn convert_celsius_to_fahrenheit(fahrenheit: f32) -> f32 {
    5.0 / 9.0 * (fahrenheit - 32.0)
}

fn choose_convert() -> bool {
    println!("Enter fc to convert Frahrenheit to Celsius, or cf to convert oposit: ");

    loop {
        let mut choise = String::new();

        io::stdin()
            .read_line(&mut choise)
            .expect("Failed to read line.");

        let choise = choise.trim().to_lowercase();

        match choise.as_str() {
            "fc" => {
                println!("Fahrenheit to Celsius convertation.");
                return true;
            }
            "cf" => {
                println!("Celsius to Fahrenheit convertation.");
                return false;
            }
            _ => {
                println!("Entered inccorect value, expected fc or cf.");
                continue;
            }
        }
    }
}
