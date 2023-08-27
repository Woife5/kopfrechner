pub struct Config;

impl Config {
    pub fn get_modes() -> Vec<&'static str> {
        vec!["Multiply", "Number Tower"]
    }

    pub fn get_mode(idx: usize) -> Mode {
        match idx {
            0 => Mode::Multiply,
            1 => Mode::NumberTower,
            _ => Mode::Multiply,
        }
    }
}

pub enum Mode {
    Multiply,
    NumberTower,
}
