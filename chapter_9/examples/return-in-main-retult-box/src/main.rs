fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::File::open("działalnik.txt")?;

    Ok(())
}
