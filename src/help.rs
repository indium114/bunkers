use dirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::{Command, Stdio};
use which::which;

// MARK: directory helpers
fn home() -> String {
    let dir = dirs::home_dir();
    return dir
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
}

pub fn bunkers_dir() -> String {
    let path: String = home() + "/.bunkers";
    let _ = fs::create_dir(&path);
    return path;
}

pub fn bunker_path(name: String) -> String {
    return bunkers_dir() + "/" + &name + ".img";
}

// MARK: elevator helper
/// determines whether to use 'sudo' or 'doas' to get root permissions
pub fn determine_elevator() -> String {
    match which("doas") {
        Ok(_path) => "doas".to_string(),
        Err(_) => "sudo".to_string(),
    }
}

// MARK: cryptsetup helpers
pub fn cryptsetup_open(loopdev: &str, mapper: &str, password: Option<&str>) -> bool {
    let keyfile = "/tmp/bunkers.key";

    if password.is_some() {
        if fs::write(keyfile, password.unwrap()).is_err() {
            return false;
        }
    }

    let cryptsetup_open_command: Result<std::process::ExitStatus, std::io::Error>;
    if password.is_some() {
        cryptsetup_open_command = Command::new(determine_elevator())
            .arg("cryptsetup")
            .arg("open")
            .arg(loopdev)
            .arg(mapper)
            .arg("--keyfile")
            .arg(keyfile)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .status();
    } else {
        cryptsetup_open_command = Command::new(determine_elevator())
            .arg("cryptsetup")
            .arg("open")
            .arg(loopdev)
            .arg(mapper)
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .status();
    }

    let result = match cryptsetup_open_command {
        Ok(_) => true,
        Err(_) => false,
    };

    return result;
}

pub fn cryptsetup_close(mapper: &str) -> bool {
    let result = Command::new(determine_elevator())
        .arg("cryptsetup")
        .arg("close")
        .arg(mapper)
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status();

    return match result {
        Ok(_) => true,
        Err(_) => false,
    };
}

// MARK: losetup helpers
pub fn losetup_attach(path: &str) -> String {
    let loop_device = Command::new(determine_elevator())
        .arg("losetup")
        .arg("--find")
        .arg("--show")
        .arg(&path)
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run losetup");

    return String::from_utf8(loop_device.stdout)
        .unwrap()
        .trim()
        .to_string();
}

pub fn losetup_detach(path: &str) -> bool {
    let result = Command::new(determine_elevator())
        .arg("losetup")
        .arg("-d")
        .arg(&path)
        .status();

    return match result {
        Ok(_) => true,
        Err(_) => false,
    };
}

// MARK: lockfile stuff
const LOCKFILE: &str = "/tmp/bunkers_lock.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct LockEntry {
    loop_path: String,
    mount_path: String,
}

pub type Lockfile = HashMap<String, LockEntry>;

/// load and save lockfile
pub fn load_lockfile() -> Lockfile {
    fs::read_to_string(LOCKFILE)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_lockfile(lock: &Lockfile) -> bool {
    match serde_json::to_string_pretty(lock) {
        Ok(json) => fs::write(LOCKFILE, json).is_ok(),
        Err(_) => false,
    }
}

/// lock and unlock helpers
pub fn lock(name: &str, loopdev: &str, mount: &str) {
    let mut lock = load_lockfile();

    lock.insert(
        name.to_string(),
        LockEntry {
            loop_path: loopdev.to_string(),
            mount_path: mount.to_string(),
        },
    );

    save_lockfile(&lock);
}

pub fn unlock(name: &str) {
    let mut lock = load_lockfile();
    lock.remove(name);
    save_lockfile(&lock);
}
