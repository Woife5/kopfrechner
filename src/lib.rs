pub mod config;

use colored::Colorize;
use config::Config;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{self, Write};

use rand::Rng;

use crate::config::Mode;

pub fn run() {
    println!("Welcome to {}!", "Kopfrechner".green().bold());

    let range = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select variant")
        .items(&Config::get_variants())
        .default(0)
        .interact();

    let range = match range {
        Ok(v) => Config::get_variant(v),
        Err(..) => {
            println!("No variant selected! Defaulting to Small (1-10).");
            1..=10
        }
    };

    let config = Config {
        mode: Mode::Multiply,
        range,
    };

    run_multiply(config);
}

fn run_multiply(config: Config) {
    let mut rng = rand::thread_rng();

    let mut correct: usize = 0;
    let mut incorrect: usize = 0;
    let mut time_ms_sum: u128 = 0;

    loop {
        let n1: usize = rng.gen_range(config.range.clone()).into();
        let n2: usize = rng.gen_range(config.range.clone()).into();
        let result = n1 * n2;

        print!("{:2} * {:2} = ", n1, n2);
        let _ = io::stdout().flush();

        let start_time = std::time::Instant::now();

        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read from stdin");
        let answer = match input_text.trim().parse::<usize>() {
            Ok(i) => i,
            Err(..) => {
                println!("Not a valid number, breaking.");
                break;
            }
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
            "Wrong!".red()
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
    println!("{}: {}", "Correct".green().italic(), correct,);
    println!("{}: {}", "Incorrect".red().italic(), incorrect,);
    println!(
        "{}: {:.2}s",
        "Average time".italic(),
        time_ms_sum as f64 / (correct + incorrect) as f64 / 1000.0
    );
}
