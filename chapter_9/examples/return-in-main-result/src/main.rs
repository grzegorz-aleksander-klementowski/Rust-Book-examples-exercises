use std::fs::File;

fn run() -> Result<String, std::io::Error> {
    File::open("działalnik.txt")?;
    Ok("Działanik został przeczytany!".to_string())
}

fn main() -> std::io::Result<()> {
    println!("{}", run()?);
    Ok(())
}
