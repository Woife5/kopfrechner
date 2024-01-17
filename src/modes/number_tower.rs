use colored::{ColoredString, Colorize};

use super::get_number_input;

pub fn prepare_and_run() {
    let input = get_number_input("Enter start number: ".to_string());
    match input {
        Some(number) => run(number),
        None => println!("No number entered."),
    }
}

fn run_iteration(current: &usize, padding: ColoredString) -> Result<(), ()> {
    let input = match get_number_input("".to_string()) {
        Some(v) => v,
        None =>  return Err(()),
    };

    let input_str = format!("{:>20}", input);
    let result_text = if input == *current {
        "Correct!".green()
    } else {
        format!("Wrong! ({})", current).red()
    };

    // Move cursor up one line and clear the line
    print!("\x1B[1A\x1B[2K\r");
    println!("{:>20} {} {}", input_str, padding, result_text);
    Ok(())
}

pub fn run(start: usize) {
    let start_time = std::time::Instant::now();
    let mut current = start;

    let padding = format!("| × {}", 2).yellow();
    println!("{:>20} {}", current, padding);
    current *= 2;

    for i in 3..=9 {
        let padding = format!("| × {}", i).yellow();
        match run_iteration(&current, padding) {
            Err(_) => return,
            _ => (),
        };
        current *= i;
    }

    for i in 2..=9 {
        let padding = format!("| ÷ {}", i).yellow();
        match run_iteration(&current, padding) {
            Err(_) => return,
            _ => (),
        };
        current /= i;
    }

    println!("{:>20} {}", current, "| Result".yellow());
    let elapsed = start_time.elapsed();
    println!("took {}", elapsed.as_secs());
}
