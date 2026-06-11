use crate::help;
use std::process::{Command, Stdio};

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
    let elevator: String = help::determine_elevator();
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

    // MARK: encrypt with LUKS
    let _ = Command::new(&elevator)
        .arg("cryptsetup")
        .arg("luksFormat")
        .arg(&loop_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run cryptsetup");

    // MARK: cryptsetup open
    let result = help::cryptsetup_open(&loop_path, "bunkers-mapper", None);

    // TODO: mkfs.ext4
    // TODO: cryptsetup close
    // TODO: losetup -d

    return true;
}
