pub mod config;
pub mod modes;

use config::Config;
use dialoguer::{theme::ColorfulTheme, Select};
use self_update::cargo_crate_version;

use crate::{
    config::Mode,
    modes::{multiplication, number_tower},
};

pub fn run() {
    loop {
        let mode = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select mode")
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
                let range = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select variant")
                    .items(&multiplication::get_variants())
                    .default(0)
                    .interact_opt();

                let range = match range {
                    Ok(Some(v)) => multiplication::get_variant(v),
                    _ => {
                        println!("No variant selected, exiting program.");
                        return;
                    }
                };

                multiplication::run(range);
            }
            Mode::NumberTower => {
                let input = modes::get_number_input("Enter start number: ".to_string());
                match input {
                    Some(number) => number_tower::run(number),
                    None => {
                        println!("No number entered, exiting program.");
                        return;
                    }
                }
            }
            Mode::Update => {
                let _ = do_update();
            }
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

    println!("Update status: `{}`!", status.version());

    if status.updated() {
        println!("Please restart the program to use the new version.");
        std::process::exit(0);
    }

    Ok(())
}
