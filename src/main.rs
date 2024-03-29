use std::thread;
use std::time::Duration;
use chrono::{Local, Timelike};
use std::io::Write;

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

    // Number of rows in each ASCII art (assuming all ASCII arts have the same height)
    let rows = numbers[0].ascii.lines().count();

    // Iterate over each row of the ASCII arts
    for i in 0..rows {
        // Iterate over each number
        for num in &numbers {
            // Print the ith row of the ASCII art for the current number
            print!("{:8}", num.ascii.lines().nth(i).unwrap()); // Adjust the spacing as needed
        }
        // Move to the next line after printing all numbers for the current row
        println!();
    }
    loop {
        let local_time = Local::now();

        let hour = local_time.hour();
        let minute = local_time.minute();
        let second = local_time.second();

        let hour_tens = hour/ 10;
        let hour_units = hour % 10;

        let minute_tens = minute/ 10;
        let minute_units = minute % 10;

        let second_tens = second / 10;
        let second_units = second % 10;

        print!("{}{}:{}{}:{}{}\r", hour_tens,hour_units, minute_tens,minute_units, second_tens,second_units);
        std::io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
    }
}




