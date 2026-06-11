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
    let loop_path: String = help::losetup_attach(&path);

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
    if !result {
        println!("failed to run cryptsetup open");
        return false;
    }

    // MARK: format as ext4
    let _ = Command::new(&elevator)
        .arg("mkfs.ext4")
        .arg("/dev/mapper/bunkers-mapper")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status();

    // TODO: cryptsetup close
    let _ = help::cryptsetup_close("bunkers-mapper");

    // TODO: losetup -d

    return true;
}
