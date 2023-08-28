pub struct Config;

impl Config {
    pub fn get_modes() -> Vec<&'static str> {
        vec!["Multiply", "Number Tower", "Check for updates"]
    }

    pub fn get_mode(idx: usize) -> Mode {
        match idx {
            0 => Mode::Multiply,
            1 => Mode::NumberTower,
            2 => Mode::Update,
            _ => Mode::Multiply,
        }
    }
}

pub enum Mode {
    Multiply,
    NumberTower,
    Update,
}
