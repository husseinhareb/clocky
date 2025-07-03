// src/main.rs
use chrono::Local;
use std::{thread, time::Duration, io::{self, Write}};
mod numbers;
use numbers::{ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, COLON};

/// Clears the terminal screen and moves the cursor to the top-left corner
fn clear_screen() {
    // ANSI escape code: clear screen and move cursor to (1,1)
    print!("\x1B[2J\x1B[H");
}

fn main() {
    loop {
        clear_screen();
        let now = Local::now();
        let time_str = now.format("%H:%M:%S").to_string();

        // Split each character's ASCII art into lines
        let arts: Vec<Vec<&str>> = time_str.chars().map(|ch| {
            let art = match ch {
                ':' => COLON,
                '0' => ZERO,
                '1' => ONE,
                '2' => TWO,
                '3' => THREE,
                '4' => FOUR,
                '5' => FIVE,
                '6' => SIX,
                '7' => SEVEN,
                '8' => EIGHT,
                '9' => NINE,
                _ => COLON,
            };
            art.lines().skip(1).collect()
        }).collect();

        // There are 5 rows per character
        for row in 0..5 {
            for art in &arts {
                print!("{}  ", art[row]);
            }
            println!();
        }

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
