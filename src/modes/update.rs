use colored::Colorize;
use self_update::cargo_crate_version;

pub fn do_update() {
    println!("Checking for updates...");

    let updater = self_update::backends::github::Update::configure()
        .repo_owner("woife5")
        .repo_name("kopfrechner")
        .bin_name("kopfrechner")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build();

    if let Err(e) = updater {
        println!("Failed to check for updates: {}", e);
        return;
    }

    let status = updater.unwrap().update();

    if let Err(e) = status {
        println!("Failed to check for updates: {}", e);
        return;
    }

    let status = status.unwrap();

    if status.updated() {
        println!("Updated to version {}", status.version().blue().bold());
        println!("The program will exit now, please restart it to use the new version.");

        let _ = std::io::stdin().read_line(&mut String::new());
        std::process::exit(0);
    }
}
