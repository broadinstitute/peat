fn main() {
    match peat::run() {
        Err(error) => println!("Error: {}", error),
        Ok(()) => println!("Done")
    }
}