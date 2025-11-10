#[allow(dead_code)]
#[derive(Debug)]
enum Good {
    Grain,
    Beer,
    Cloth,
    Timber,
}

#[allow(dead_code)]
#[derive(Debug)]
enum HanzaticTradeRecords {
    City(String),
    Gold(i32),
    Reputation(f32),
    Quantity(u32),
    PricePerUnit(f32),
    Good(Good),
}

// The correct version of suplicate the vector in place
fn dup_in_place(v: &mut Vec<u8>) {
    for i in 0..v.len() {
        // i → 0, 1, 2, …
        // 0..v → Range<usize>
        v.push(v[i]);
    }
}

fn main() {
    println!("Creatgin a new vector");
    #[allow(unused_variables)]
    let v: Vec<u8> = Vec::new();

    println!("Creating a new vectore with values");
    let mut v: Vec<u8> = vec![1, 2, 3];

    // Adding a new values to the vector (`v` needs to be `mut`)
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    // Borrowing a value from a vactor.
    let trzeci: &u8 = &v[2]; // mniej bezpieczny – w przypadku błędu program pada.
    println!("Trzeci element to: {trzeci}");

    let trzeci: Option<&u8> = v.get(2); // bezpieczniejszy sposób.
    match trzeci {
        Some(trzeci) => println!("Trzeci element to {trzeci}"),
        None => println!("Nie ma trzeciego elementu"),
    }

    println!("Vector in loop to read the data");
    for i in &v {
        println!("{i}");
    }

    println!("\nVector in loop to change the data:");
    for j in &mut v {
        *j += 2;
        println!("{j}");
    }

    println!("Duplicate the vector…");
    dup_in_place(&mut v);
    println!("Now the vector looks like: {:?}", v);

    println!("The example with vector with different types:");
    let entry = vec![
        HanzaticTradeRecords::City(String::from("Wrocław")),
        HanzaticTradeRecords::Gold(3000),
        HanzaticTradeRecords::Reputation(3.5),
        HanzaticTradeRecords::Quantity(25),
        HanzaticTradeRecords::PricePerUnit(2.5),
        HanzaticTradeRecords::Good(Good::Beer),
    ];
    println!(
        "Endry of Wrocław Hanzatic Trage Records vector: {:?}",
        entry
    );
}

// Incorrect version of duplicate the vector in place
/* fn incorrect_dup_in_place(v: &mut Vec<u8>) {
    for n_ref in v.iter() { // the pointer `iter()` immutable points to (immutable borrow) `v[0]` → `v` become immutable
        v.push(*n_ref); // `push()` try to add dereference of `n_ref` (which is v[0]). However, till the pointer poinsts to `v[0]`, it cannot be change (muted).
    }
} */
