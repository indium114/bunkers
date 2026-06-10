use crate::help;
use std::process::{Command, Stdio};
use which::which;

pub fn create(name: &String, size: &u32) -> bool {
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
    let loop_device = Command::new(&elevator)
        .arg("losetup")
        .arg("--find")
        .arg("--show")
        .arg(&path)
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run losetup");

    let loop_path: String = String::from_utf8(loop_device.stdout)
        .unwrap()
        .trim()
        .to_string();

    // MARK: make sure that device_path starts with /dev/loop
    if !loop_path.starts_with("/dev/loop") {
        println!("loop_path does not start with /dev/loop");
        return false;
    }
    println!("loop_path starts with /dev/loop");

    return true;
}
