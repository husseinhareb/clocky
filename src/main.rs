// src/main.rs
use chrono::Local;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

mod numbers;
use numbers::{COLON, EIGHT, FIVE, FOUR, NINE, ONE, SEVEN, SIX, THREE, TWO, ZERO};

/// Draws the current time in big ASCII art at the current cursor position.
fn draw_clock() {
    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();

    // Build a vector of 5-line slices for each character
    let arts: Vec<Vec<&str>> = time_str
        .chars()
        .map(|ch| {
            let art = match ch {
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
                ':' => COLON,
                _ => COLON,
            };
            art.lines().skip(1).collect()
        })
        .collect();

    // Print row by row
    for row in 0..5 {
        for art in &arts {
            print!("{}  ", art[row]);
        }
        println!();
    }
}

fn main() {
    // Draw the clock once at startup
    draw_clock();
    io::stdout().flush().unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));

        // Move cursor up 5 lines (so we overwrite exactly where we drew)
        print!("\x1B[5A");
        draw_clock();
        io::stdout().flush().unwrap();
    }
}
