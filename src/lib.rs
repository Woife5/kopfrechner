pub mod main_menu;
pub mod modes;

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use main_menu::Config;
use self_update::cargo_crate_version;

use crate::{
    main_menu::Mode,
    modes::{multiplication, number_tower},
};

pub fn run() {
    loop {
        let mode = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Main menu")
            .items(&Config::get_modes())
            .default(0)
            .interact_opt();

        let mode = match mode {
            Ok(Some(v)) => Config::get_mode(v),
            _ => {
                println!("No mode selected, exiting program.");
                return;
            }
        };

        match mode {
            Mode::Multiply => {
                let range_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select variant")
                    .items(&multiplication::get_variants())
                    .default(0)
                    .interact_opt();

                if let Ok(Some(r)) = range_selection {
                    let range = multiplication::get_variant(r);
                    multiplication::run(range);
                } else {
                    println!("Invalid selection.");
                }
            }
            Mode::NumberTower => {
                let input = modes::get_number_input("Enter start number: ".to_string());
                match input {
                    Some(number) => number_tower::run(number),
                    None => println!("No number entered."),
                }
            }
            Mode::Update => {
                let _ = do_update();

                #[cfg(windows)]
                println!("{}", "Temp directory has been cleaned up.".yellow().bold());
            }
            Mode::Exit => std::process::exit(0),
        }

        println!();
    }
}

fn do_update() -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking for updates...");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("woife5")
        .repo_name("kopfrechner")
        .bin_name("kopfrechner")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

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

    Ok(())
}
