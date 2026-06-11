use crate::help::{self, cryptsetup_open, determine_elevator};
use nix::unistd::{Gid, Uid};
use std::process::Command;
use usefulog;
use which::which;

pub fn mount(name: &String) -> bool {
    // MARK: some handy variables that'll help us later
    let mapper_name = "bunkers-".to_string() + name;
    let mapper_path = "/dev/mapper/".to_string() + &mapper_name;

    // MARK: attach loop device
    let path: String = help::bunker_path(name.to_string());
    let loop_path: String = help::losetup_attach(&path);

    // MARK: attempt to get password from 'pass'
    let password: Option<String> = match which("pass") {
        Ok(_path) => {
            // get the password
            let pass_path: String = "bunkers/".to_string() + name;
            let output = Command::new("pass")
                .arg("show")
                .arg(&pass_path)
                .output()
                .expect("failed to run pass");

            let output_stdout = output.stdout;
            let pass = String::from_utf8_lossy(&output_stdout);

            if pass.trim().starts_with("Error:") || pass.trim() == "" {
                None
            } else {
                Some(pass.trim().to_string())
            }
        }
        Err(_) => None,
    };

    // MARK: cryptsetup open
    let _ = cryptsetup_open(&loop_path, &mapper_name, password.as_deref());

    // MARK: run fsck
    let fsck_passed = match Command::new(determine_elevator())
        .arg("fsck.ext4")
        .arg("-p")
        .arg(&mapper_path)
        .status()
    {
        Ok(_) => true,
        Err(_) => false,
    };

    if !fsck_passed {
        usefulog::err(format!("fsck failed on /dev/mapper/{}", &mapper_name));
        return false;
    }

    // MARK: actually mount the thing
    let mount_path: String = help::make_mount_path(name.to_string());
    let _ = Command::new(determine_elevator())
        .arg("mount")
        .arg(&mapper_path)
        .arg(&mount_path)
        .status();

    // MARK: chown the mount to the current user
    let uid = Uid::current().as_raw();
    let gid = Gid::current().as_raw();
    let _ = Command::new(determine_elevator())
        .arg("chown")
        .arg(format!("{uid}:{gid}"))
        .arg(&mount_path)
        .status();

    // MARK: lock the new mount
    help::lock(&name, &loop_path, &mount_path);

    usefulog::ok(format!("successfully mounted {} at {}", &name, &mount_path));

    return true;
}
