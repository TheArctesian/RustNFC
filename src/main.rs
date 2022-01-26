extern crate nfc;

use nfc::context;
use nfc::misc;

fn main() {
    let mut context = context::new();

    if context.is_null() {
        println!("Unable to initialize new NFC context!");
    }

    // Initialize libnfc
    nfc::init(&mut context);
    
    // Print libnfc version
    println!("libnfc version: {}", misc::version());
}
