pub struct Config;

impl Config {
    pub fn get_modes() -> Vec<&'static str> {
        vec![
            "Train multiplication",
            "Calculate a Number Tower",
            "Check for updates",
            "Exit",
        ]
    }

    pub fn get_mode(idx: usize) -> Mode {
        match idx {
            0 => Mode::Multiply,
            1 => Mode::NumberTower,
            2 => Mode::Update,
            _ => Mode::Exit,
        }
    }
}

pub enum Mode {
    Multiply,
    NumberTower,
    Update,
    Exit,
}
