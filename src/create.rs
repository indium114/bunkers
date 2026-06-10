use crate::help;
use std::process::Command;

pub fn create(name: &String, size: &u32) {
    // MARK: create the image
    let path: String = help::bunker_path(name.to_string());

    let _ = Command::new("truncate")
        .arg("-s")
        .arg(size.to_string() + "M")
        .arg(path)
        .output()
        .expect("failed to run truncate");
}
