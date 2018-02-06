extern crate dropbox_dir;

use dropbox_dir::{business_dir, personal_dir};

pub fn main() {
    match personal_dir() {
        Ok(path) => println!("personal: {}", path),
        Err(_) => println!("personal: <not available>"),
    }
    match business_dir() {
        Ok(path) => println!("business: {}", path),
        Err(_) => println!("business: <not available>"),
    }
}
