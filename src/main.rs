use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::io::Write;


const START_GOLD: u64 = 100;
const FOOD_PRICE: u64 = 5;
const MAX_SEED: u64 = 9999;

fn print_menu() {
    println!("\n--- Trail Trader Menu ---");
    println!("1. Buy food");
    println!("2. Hunt");
    println!("3. Trade with caravan");
    println!("4. View status");
    println!("5. Quit");
}
fn read_number(min: u64, max: u64) -> u64 {
    loop {
        print!("Enter a number ({}-{}): ", min, max);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input. Please try again.");
            continue;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            println!("Input cannot be empty. Please try again.");
            continue;
        }

        match trimmed.parse::<u64>() {
            Ok(num) => {
                if num >= min && num <= max {
                    return num;
                } else {
                    println!("Number out of range. Must be between {} and {}.", min, max);
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid whole number.");
            }
        }
    }
}

fn roll_die(seed: u64) -> u64 {
    let mut rng = StdRng::seed_from_u64(seed);
    rng.random_range(1..=6)
}
fn main() {
    println!("Hello, world!");
}
