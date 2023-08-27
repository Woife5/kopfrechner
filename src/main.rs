use colored::Colorize;
use kopfrechner::run;

fn main() {
    println!(
        "{} v{}",
        "Kopfrechner".green().bold(),
        env!("CARGO_PKG_VERSION")
    );

    run();

    // wait for a keypress before exiting
    println!("Press any key to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}
