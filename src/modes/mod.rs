use std::io::{self, Write};

pub mod multiplication;
pub mod number_tower;
pub mod update;

fn get_number_input(prompt: String) -> Option<usize> {
    loop {
        print!("{}", prompt);
        let _ = io::stdout().flush(); // flush stdout to make sure the prompt is displayed

        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read from stdin");
        let input_text = input_text.trim();

        match input_text {
            "q" | "exit" | "quit" => return None,
            _ => match input_text.parse::<usize>() {
                Ok(v) => return Some(v),
                Err(_) => {
                    println!("\rNot a valid number, try again. (Enter 'q' to quit)");
                    continue;
                }
            },
        }
    }
}
