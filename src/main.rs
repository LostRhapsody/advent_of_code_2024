use advent_of_code_2024::day_one;
use advent_of_code_2024::day_two;

fn main() {
    println!("Select which puzzle to solve: ");
    let input = 2;
    match input {
        1 => match day_one::solve() {
            Ok(_) => println!("Puzzle one done"),
            Err(err) => println!("Puzzle one hit an error: {}", err),
        },
        2 => match day_two::solve() {
            Ok(_) => println!("Puzzle two done"),
            Err(err) => println!("Puzzle two hit an error: {}", err),
        },
        _ => (),
    };
}
