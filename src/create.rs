use crate::help;
use std::process::{Command, Stdio};
use which::which;

pub fn create(name: &String, size: &u32) {
    // MARK: create the image
    let path: String = help::bunker_path(name.to_string());

    let _ = Command::new("truncate")
        .arg("-s")
        .arg(size.to_string() + "M")
        .arg(&path)
        .output()
        .expect("failed to run truncate");

    // MARK: mount loop device
    let elevator: String = match which("doas") {
        Ok(_path) => "doas".to_string(),
        Err(_) => "sudo".to_string(),
    };
    let device_path = Command::new(&elevator)
        .arg("losetup")
        .arg("--find")
        .arg("--show")
        .arg(&path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run losetup");
}
