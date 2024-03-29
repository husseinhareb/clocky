use std::thread;
use std::time::Duration;
use chrono::{Local, Timelike};
use std::io::Write;
use std::process::Command;

mod numbers;

struct Number {
    value: u8,
    ascii: &'static str,
}

impl Number {
    fn new(value: u8, ascii: &'static str) -> Self {
        Number { value, ascii }
    }
}

fn main() {
    let numbers = [
        Number::new(0, numbers::ZERO),
        Number::new(1, numbers::ONE),
        Number::new(2, numbers::TWO),
        Number::new(3, numbers::THREE),
        Number::new(4, numbers::FOUR),
        Number::new(5, numbers::FIVE),
        Number::new(6, numbers::SIX),
        Number::new(7, numbers::SEVEN),
        Number::new(8, numbers::EIGHT),
        Number::new(9, numbers::NINE),
    ];

    println!("{}",numbers[0].ascii)

}



