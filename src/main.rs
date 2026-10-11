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

fn main() {
    println!("Hello, world!");
}
