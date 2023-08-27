use colored::Colorize;
use kopfrechner::run;

fn main() {
    println!("Welcome to {}!", "Kopfrechner".green().bold());

    run();

    // wait for a keypress before exiting
    println!("Press any key to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}
