use crate::help;
use std::process::{Command, Stdio};
use usefulog;

pub fn umount(name: &String) -> bool {
    let lock = help::load_lockfile();
    let Some(lock_entry) = lock.get(name) else {
        usefulog::err(format!("{} is not mounted", &name));
        return false;
    };
    let mapper_path = "/dev/mapper/bunkers-".to_string() + &name;

    // MARK: unmount
    let _ = Command::new(help::determine_elevator())
        .arg("umount")
        .arg(help::mount_path(name.to_string()))
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status();

    // MARK: cryptsetup close
    let close_success = help::cryptsetup_close(&mapper_path);
    if !close_success {
        usefulog::err("failed to run cryptsetup close");
        return false;
    }

    // MARK: detach loop device
    let _ = Command::new(help::determine_elevator())
        .arg("losetup")
        .arg("-d")
        .arg(&lock_entry.loop_path)
        .status();

    // MARK: unlock in lockfile
    help::unlock(&name);

    usefulog::ok(format!("unmounted {}", &name));

    return true;
}
