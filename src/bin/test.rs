
extern crate objcruntime;

use objcruntime::objc;

fn main() {
    match objc::get_class("blah") {
        Some(_) => println!("blah exist !"),
        None => println!("blah doesn't exist !"),
    };

    match objc::get_class("NSString") {
        Some(_) => println!("NSString exist !"),
        None => println!("NSString doesn't exist !"),
    };

}