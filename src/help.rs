use dirs;
use std::fs;

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
