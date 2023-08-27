use std::ops::RangeInclusive;

use colored::Colorize;
use rand::Rng;

use crate::modes::get_number_input;

pub fn run(range: RangeInclusive<u16>) {
    let mut rng = rand::thread_rng();

    let mut correct: usize = 0;
    let mut incorrect: usize = 0;
    let mut time_ms_sum: u128 = 0;

    println!("At any point you can enter 'q' to quit and display statistics.");
    loop {
        let n1: usize = rng.gen_range(range.clone()).into();
        let n2: usize = rng.gen_range(range.clone()).into();
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
    println!("{}: {}", "Correct".green().italic(), correct,);
    println!("{}: {}", "Incorrect".red().italic(), incorrect,);
    println!(
        "{}: {:.2}s",
        "Average time".italic(),
        time_ms_sum as f64 / (correct + incorrect) as f64 / 1000.0
    );
}

pub fn get_variants() -> Vec<&'static str> {
    vec!["Small (2-10)", "Big (11-100)", "All (2-100)"]
}

pub fn get_variant(idx: usize) -> RangeInclusive<u16> {
    match idx {
        0 => 2..=10,
        1 => 11..=100,
        2 => 2..=100,
        _ => 2..=10,
    }
}
