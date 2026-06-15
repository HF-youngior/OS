#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("\x1b[31mThis is red text\x1b[0m");
    println!("\x1b[32mThis is green text\x1b[0m");
    println!("\x1b[34mThis is blue text\x1b[0m");
    println!("\x1b[33mColor output test OK!\x1b[0m");
    0
}
