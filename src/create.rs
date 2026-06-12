use crate::help;
use std::process::{Command, Stdio};
use usefulog;

pub fn create(name: &String, size: &u32) -> bool {
    // MARK: create the image
    let path: String = help::bunker_path(name.to_string());

    let truncate_status = Command::new("truncate")
        .arg("-s")
        .arg(size.to_string() + "M")
        .arg(&path)
        .status();
    match truncate_status {
        Ok(_) => (),
        Err(_) => {
            usefulog::err("failed to allocate the image with truncate");
            return false;
        }
    }

    // MARK: mount loop device
    let elevator: String = help::determine_elevator();
    let loop_path: String = help::losetup_attach(&path);

    // MARK: make sure that device_path starts with /dev/loop
    if !loop_path.starts_with("/dev/loop") {
        usefulog::err("loop_path does not start with /dev/loop");
        return false;
    }

    // MARK: encrypt with LUKS
    let luks_status = Command::new(&elevator)
        .arg("cryptsetup")
        .arg("luksFormat")
        .arg(&loop_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    match luks_status {
        Ok(_) => (),
        Err(_) => {
            usefulog::err("failed to run cryptsetup luksFormat");
            return false;
        }
    }

    // MARK: cryptsetup open
    let result = help::cryptsetup_open(&loop_path, "bunkers-mapper", None);
    if !result {
        usefulog::err("failed to run cryptsetup open");
        return false;
    }

    // MARK: format as ext4
    let mkfs_status = Command::new(&elevator)
        .arg("mkfs.ext4")
        .arg("/dev/mapper/bunkers-mapper")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status();
    match mkfs_status {
        Ok(_) => (),
        Err(_) => {
            usefulog::err("failed to format image using mkfs.ext4");
            return false;
        }
    }

    // MARK: cryptsetup close
    let close_status = help::cryptsetup_close("bunkers-mapper");
    if !close_status {
        usefulog::err("failed to close mapper");
        return false;
    }

    // MARK: detach loop device
    let detach_status = help::losetup_detach(&loop_path);
    if !detach_status {
        usefulog::err("failed to detach loop device");
        return false;
    }

    usefulog::ok(format!("created bunker {}", &name));

    return true;
}
