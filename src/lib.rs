pub mod main_menu;
pub mod modes;

use dialoguer::{theme::ColorfulTheme, Select};
use main_menu::{get_mode, get_modes, Mode};
use modes::{multiplication, number_tower, update};

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
        Mode::Update => update::do_update(),
        Mode::Exit => std::process::exit(0),
    }

    println!();
}
