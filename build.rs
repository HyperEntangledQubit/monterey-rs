extern crate version_check as rustc;

use std::process::exit;

fn main() {
    // Verify installed version of rustc isn't too low
    if rustc::is_min_version("1.64.0") != Some(true) {
        eprintln!("Error occurred during version check in build.rs");
        exit(1);
    }
}
