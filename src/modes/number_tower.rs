use colored::Colorize;

use super::get_number_input;

pub fn prepare_and_run() {
    let input = get_number_input("Enter start number: ".to_string());
    match input {
        Some(number) => run(number),
        None => println!("No number entered."),
    }
}

pub fn run(start: usize) {
    let mut current = start;

    for i in 2..=9 {
        let padding = format!("| × {}", i).yellow();
        println!("{:>20} {}", current, padding);
        current *= i;
    }

    for i in 2..=9 {
        let padding = format!("| ÷ {}", i).yellow();
        println!("{:>20} {}", current, padding);
        current /= i;
    }

    println!("{:>20} {}", current, "| Result".yellow());
}
