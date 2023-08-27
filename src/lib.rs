pub mod config;
pub mod modes;

use config::Config;
use dialoguer::{theme::ColorfulTheme, Select};

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
        }

        println!();
    }
}
