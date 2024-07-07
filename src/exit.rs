use std::io::{self, Read};

pub fn wait_for_key_press() {
    println!("Press any key to exit...");
    let _ = io::stdin().read(&mut [0u8]).expect("Failed to read input");
}
