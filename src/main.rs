use enigo::*;
use std::env;
use std::process::Command;
use std::{thread, time};

const WAIT_FOR_WINDOW_ACTIVATING_MS: u64 = 50_u64;

fn main() {
    let key = env::args().nth(1).unwrap();
    let dur = time::Duration::from_millis(WAIT_FOR_WINDOW_ACTIVATING_MS);
    let mut enigo = Enigo::new();
    let ror_args = ["run-or-raise", "Zathura", "echo", "no-zathura"];
    Command::new("/home/guru/bin/rp-enhanced-window-switcher")
        .args(&ror_args)
        .spawn()
        .expect("Failed to execute ratpoison to switch to Zathura");
    thread::sleep(dur);
    match key.as_ref() {
        "next" => enigo.key_click(Key::Layout('J')),
        "prior" => enigo.key_click(Key::Layout('K')),
        _ => eprintln!("Unknown key {}", key),
    };
    Command::new("ratpoison")
        .arg("-c")
        .arg("focuslast")
        .spawn()
        .expect("Failed to execute ratpoison");
}
