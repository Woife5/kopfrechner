use colored::Colorize;

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
