use colored::Colorize;
use kopfrechner::run;
use self_update::cargo_crate_version;

fn main() {
    println!(
        "{} v{}",
        "Kopfrechner".green().bold(),
        cargo_crate_version!()
    );

    run();

    // wait for a keypress before exiting
    println!("Press any key to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}
