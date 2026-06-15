#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Hello from student app!");
    println!("Name: Yang Yahan");
    println!("Student ID: 23301083");
    println!("Test student_info OK!");
    0
}

