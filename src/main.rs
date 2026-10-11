use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::io::{self, Write};

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
fn roll_many(seed: u64, count: u64) -> u64 {
    let mut total = 0;
    for i in 0..count {
        total += roll_die(seed + i);
    }
    total
}

fn hunt_food(roll: u64) -> u64 {
    if roll == 1 {
        0
    } else if roll == 2 || roll == 3 {
        10
    } else if roll == 4 || roll == 5 {
        20
    } else if roll == 6 {
        40
    } else {
        0
    }
}

fn can_afford(gold: u64, cost: u64) -> bool {
    gold >= cost
}

fn print_status(gold: u64, food: u64, day: u64) {
    println!("\n--- Status ---");
    println!("Day: {}", day);
    println!("Gold: {}", gold);
    println!("Food: {} units", food);
    println!("--------------");
}

fn main() {
    println!("Welcome to Trail Trader!");
    let seed = read_number(1, MAX_SEED);

    let mut gold = START_GOLD;
    let mut food = 0;
    let mut day = 1;
    let mut roll_count = 0;

    loop {
        print_menu();
        let choice = read_number(1, 5);

        match choice {
            1 => {
                if !can_afford(gold, FOOD_PRICE) {
                    println!(
                        "You don't have enough gold to buy food! (Need at least {} gold)",
                        FOOD_PRICE
                    );
                } else {
                    let max_affordable = gold / FOOD_PRICE;
                    println!("You can afford up to {} units of food.", max_affordable);
                    println!("How many units of food would you like to buy?");
                    let units = read_number(1, max_affordable);

                    let cost = units * FOOD_PRICE;
                    gold -= cost;
                    food += units;
                    day += 1;
                    println!(
                        "Successfully bought {} units of food for {} gold.",
                        units, cost
                    );
                }
            }
            2 => {
                let roll = roll_die(seed + roll_count);
                roll_count += 1;
                let gained = hunt_food(roll);
                food += gained;
                day += 1;
                println!(
                    "You went hunting and rolled a {}. Gained {} units of food!",
                    roll, gained
                );
            }
            3 => {
                let total_rolled = roll_many(seed + roll_count, 3);
                roll_count += 3;
                let earned = 2 * total_rolled;
                gold += earned;
                day += 1;
                println!(
                    "Traded with caravan! Total dice sum: {}. Earned {} gold!",
                    total_rolled, earned
                );
            }
            4 => {
                print_status(gold, food, day);
            }
            5 => {
                println!("\nQuitting game. Final Status:");
                print_status(gold, food, day);
                println!("Thanks for playing Trail Trader!");
                break;
            }
            _ => unreachable!(),
        }
    }
}
