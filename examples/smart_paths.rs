extern crate dropbox_dir;

use dropbox_dir::SmartPath;

pub fn main() {
    match SmartPath::new_personal("some/dir") {
        Ok(path) => println!("personal (some/dir): {}", path.local().display()),
        Err(_) => println!("personal: <not available>"),
    }
    match SmartPath::new_business("") {
        Ok(path) => println!("business: {}", path.local().display()),
        Err(_) => println!("business: <not available>"),
    }
}