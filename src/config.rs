use std::ops::RangeInclusive;

pub struct Config {
    pub mode: Mode,
    pub range: RangeInclusive<u16>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            mode: Mode::Multiply,
            range: (1..=10),
        }
    }

    pub fn get_modes() -> Vec<&'static str> {
        vec!["Multiply", "Divide"]
    }

    pub fn get_mode(idx: usize) -> Mode {
        match idx {
            0 => Mode::Multiply,
            1 => Mode::Divide,
            _ => Mode::Multiply,
        }
    }

    pub fn get_variants() -> Vec<&'static str> {
        vec!["Small (1-10)", "Big (11-100)", "All (1-100)"]
    }

    pub fn get_variant(idx: usize) -> RangeInclusive<u16> {
        match idx {
            0 => 1..=10,
            1 => 11..=100,
            2 => 1..=100,
            _ => 1..=10,
        }
    }
}

pub enum Mode {
    Multiply,
    Divide,
}
