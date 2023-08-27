use kopfrechner::run;

fn main() {
    run();

    // wait for a keypress before exiting
    println!("Press any key to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}
