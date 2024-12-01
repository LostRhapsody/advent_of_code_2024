pub mod day_one;

fn main() {
    match day_one::main() {
        Ok(_) => println!("Puzzle one done"),
        Err(err) => println!("Puzzle one hit an error: {}", err),
    };
}
