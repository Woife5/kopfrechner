use colored::Colorize;
use kopfrechner::run;
use self_update::cargo_crate_version;

fn main() {
    println!(
        "{} v{}",
        "Kopfrechner".green().bold(),
        cargo_crate_version!()
    );

    loop {
        run();
    }
}
