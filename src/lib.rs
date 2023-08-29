pub mod main_menu;
pub mod modes;

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use main_menu::{get_mode, get_modes, Mode};
use modes::{multiplication, number_tower};
use self_update::cargo_crate_version;

pub fn run() {
    let mode = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Main menu")
        .items(&get_modes())
        .default(0)
        .interact();

    let mode = match mode {
        Ok(v) => get_mode(v),
        _ => return,
    };

    match mode {
        Mode::Multiply => multiplication::prepare_and_run(),
        Mode::NumberTower => number_tower::prepare_and_run(),
        Mode::Update => do_update(),
        Mode::Exit => std::process::exit(0),
    }

    println!();
}

fn do_update() {
    println!("Checking for updates...");

    let updater = self_update::backends::github::Update::configure()
        .repo_owner("woife5")
        .repo_name("kopfrechner")
        .bin_name("kopfrechner")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build();

    if let Err(e) = updater {
        println!("Failed to check for updates: {}", e);
        return;
    }

    let status = updater.unwrap().update();

    if let Err(e) = status {
        println!("Failed to check for updates: {}", e);
        return;
    }

    let status = status.unwrap();

    if status.updated() {
        println!("Updated to version {}", status.version().blue().bold());
        println!("The program will exit now, please restart it to use the new version.");

        #[cfg(windows)]
        println!(
            "{}",
            "Rerun the update check to clean up any temporary files."
                .yellow()
                .bold()
        );

        let _ = std::io::stdin().read_line(&mut String::new());
        std::process::exit(0);
    }

    #[cfg(windows)]
    println!("{}", "Temp directory has been cleaned up.".yellow().bold());
}
