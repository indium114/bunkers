use crate::help;
use std::fs;

pub fn status() {
    let bunkers_dir = help::bunkers_dir();
    let lock = help::load_lockfile();

    let mut mounted = Vec::new();
    let mut unmounted = Vec::new();

    if let Ok(entries) = fs::read_dir(&bunkers_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("img") {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    if lock.contains_key(name) {
                        mounted.push(name.to_string());
                    } else {
                        unmounted.push(name.to_string());
                    }
                }
            }
        }
    }

    mounted.sort();
    unmounted.sort();

    if !mounted.is_empty() {
        println!("mounted:");
    };
    for name in &mounted {
        println!("| {name}");
    }

    if !mounted.is_empty() {
        println!();
    }
    if !unmounted.is_empty() {
        println!("unmounted:");
    }
    for name in &unmounted {
        println!("| {}", name)
    }
}
