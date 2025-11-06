use std::ops::RangeInclusive;

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use rand::random_range;

use crate::modes::get_number_input;

fn get_variant(idx: usize) -> RangeInclusive<u16> {
    match idx {
        0 => 2..=10,
        1 => 11..=100,
        2 => 2..=100,
        _ => 2..=10,
    }
}

fn get_must_include_numbers(range: &RangeInclusive<u16>) -> Vec<u16> {
    let mut must_include = Vec::new();

    println!("Enter numbers to always include (press Enter with empty input to finish):");

    loop {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Number")
            .allow_empty(true)
            .interact()
            .unwrap_or_default();

        if input.trim().is_empty() {
            break;
        }

        match input.trim().parse::<u16>() {
            Ok(num) if range.contains(&num) => {
                if !must_include.contains(&num) {
                    must_include.push(num);
                    println!("Added: {}", num);
                } else {
                    println!("Number {} already added.", num);
                }
            }
            Ok(num) => {
                println!(
                    "Number {} is outside the selected range ({}-{}).",
                    num,
                    range.start(),
                    range.end()
                );
            }
            Err(_) => {
                println!("Invalid number. Please try again.");
            }
        }
    }

    if must_include.is_empty() {
        println!("No specific numbers selected.");
    } else {
        println!("Selected numbers: {:?}", must_include);
    }

    must_include
}

pub fn prepare_and_run() {
    let variants = vec!["Small (2-10)", "Big (11-100)", "All (2-100)"];

    let range_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select variant")
        .items(&variants)
        .default(0)
        .interact_opt();

    if let Ok(Some(r)) = range_selection {
        let range = get_variant(r);
        let must_include = get_must_include_numbers(&range);
        run(range, &must_include);
    } else {
        println!("Invalid selection.");
    }
}

pub fn run(range: RangeInclusive<u16>, must_include: &[u16]) {
    let mut correct: usize = 0;
    let mut incorrect: usize = 0;
    let mut time_ms_sum: u128 = 0;

    println!("At any point you can enter 'q' to quit and display statistics.");
    loop {
        let n1: usize = if !must_include.is_empty() {
            must_include[random_range(0..must_include.len())] as usize
        } else {
            random_range(range.clone()).into()
        };

        let n2: usize = random_range(range.clone()).into();
        let result = n1 * n2;

        let start_time = std::time::Instant::now();

        let answer = match get_number_input(format!("{:2} * {:2} = ", n1, n2)) {
            Some(v) => v,
            None => break,
        };

        let elapsed = start_time.elapsed();
        time_ms_sum += elapsed.as_millis();

        // Move cursor up one line and clear the line
        print!("\x1B[1A\x1B[2K\r");

        let result_text = if answer == result {
            correct += 1;
            "Correct!".green()
        } else {
            incorrect += 1;
            format!("Wrong! ({:3})", result).red()
        };

        println!(
            "{:2} * {:2} = {:3} {} ({:.2}s)",
            n1,
            n2,
            answer,
            result_text,
            elapsed.as_secs_f64()
        );
    }

    println!("\n{}", "Results:".bold());
    println!("{}: {}", "Correct".green().italic(), correct);
    println!("{}: {}", "Incorrect".red().italic(), incorrect);
    println!(
        "{}: {:.2}s",
        "Average time".italic(),
        time_ms_sum as f64 / (correct + incorrect) as f64 / 1000.0
    );
}
