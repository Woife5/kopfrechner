use colored::Colorize;
use self_update::cargo_crate_version;

fn is_release_archive(asset_name: &str, target: &str) -> bool {
    asset_name == format!("kopfrechner-{target}.tar.xz")
        || asset_name == format!("kopfrechner-{target}.zip")
}

fn bin_path_in_archive(target: &str) -> &'static str {
    if target.contains("windows") {
        "{{ bin }}"
    } else {
        "kopfrechner-{{ target }}/{{ bin }}"
    }
}

pub fn do_update() {
    println!("Checking for updates...");

    let target = self_update::get_target().to_owned();
    let bin_path = bin_path_in_archive(&target);
    let updater = self_update::backends::github::Update::configure()
        .repo_owner("woife5")
        .repo_name("kopfrechner")
        .bin_name("kopfrechner")
        .bin_path_in_archive(bin_path)
        .asset_matcher(move |assets| {
            assets
                .iter()
                .find(|asset| is_release_archive(asset.name(), &target))
                .cloned()
        })
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

    if status.is_updated() {
        println!("Updated to version {}", status.version().blue().bold());
        println!("The program will exit now, please restart it to use the new version.");

        let _ = std::io::stdin().read_line(&mut String::new());
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::{bin_path_in_archive, is_release_archive};

    #[test]
    fn selects_cargo_dist_archives_but_not_checksum_sidecars() {
        let targets_and_archives = [
            (
                "aarch64-apple-darwin",
                "kopfrechner-aarch64-apple-darwin.tar.xz",
            ),
            (
                "x86_64-apple-darwin",
                "kopfrechner-x86_64-apple-darwin.tar.xz",
            ),
            (
                "x86_64-unknown-linux-gnu",
                "kopfrechner-x86_64-unknown-linux-gnu.tar.xz",
            ),
            (
                "x86_64-pc-windows-msvc",
                "kopfrechner-x86_64-pc-windows-msvc.zip",
            ),
        ];

        for (target, archive) in targets_and_archives {
            assert!(is_release_archive(archive, target));
            assert!(!is_release_archive(&format!("{archive}.sha256"), target));
            assert!(!is_release_archive("source.tar.xz", target));
            assert!(!is_release_archive(archive, "different-target"));
        }
    }

    #[test]
    fn uses_the_cargo_dist_binary_path_for_each_archive_format() {
        assert_eq!(
            bin_path_in_archive("aarch64-apple-darwin"),
            "kopfrechner-{{ target }}/{{ bin }}"
        );
        assert_eq!(
            bin_path_in_archive("x86_64-apple-darwin"),
            "kopfrechner-{{ target }}/{{ bin }}"
        );
        assert_eq!(
            bin_path_in_archive("x86_64-unknown-linux-gnu"),
            "kopfrechner-{{ target }}/{{ bin }}"
        );
        assert_eq!(bin_path_in_archive("x86_64-pc-windows-msvc"), "{{ bin }}");
    }
}
