//! Entry point for a CLI application that uses the `rust_math_lib` library.

// use rust_math_lib::integrals::Integral;
// use rust_math_lib::utils::colours::{*};

use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Write, stdin},
    thread,
    time::Duration,
}; // `thread` and `time::Duration` are used for simulating the loading process.

fn main() {
    let mut stdout = stdout();

    // Clear the screen and display the greeting message
    execute!(
        stdout,
        Clear(ClearType::All), // Clear the entire screen
        cursor::MoveTo(0, 0), // Move the cursor to the top-left corner
        Print("Welcome to the Rust Math library Command Line Interface!\n"),
        Print("Loading, please wait...\n\n")
    )
    .unwrap(); // Ensure the output is displayed immediately. Panic if it fails.

    // Simulate a loading bar
    let total_steps = 20; // Number of steps in the loading bar
    let delay = Duration::from_millis(100); // Delay between steps (100ms for 3 seconds total)

    for step in 0 ..= total_steps {
        let percentage = (step as f32 / total_steps as f32) * 100.0; // Calculate the percentage of the loading bar

        // Draw the loading bar
        execute!(
            stdout,
            cursor::MoveTo(0, 3), // Move to the line where the loading bar is displayed
            Print("["),
            SetBackgroundColor(Color::Green),
            Print(" ".repeat(step)), // Filled part of the bar
            ResetColor,
            Print(" ".repeat(total_steps - step)), // Empty part of the bar
            Print("] "),
            Print(format!("{:.0}%", percentage)) // Display percentage
        )
        .unwrap();

        stdout.flush().unwrap(); // Ensure the output is displayed immediately
        thread::sleep(delay); // Wait before the next step
    }

    // Final message after loading is complete
    execute!(
        stdout,
        cursor::MoveTo(0, 5),
        Print("Loading complete.\n\n")
    )
    .unwrap();

    // Prompt the user for their choice
    execute!(
        stdout,
        Print("Please choose an option:\n"),
        Print("  1. Calculate a derivative\n"),
        Print("  2. Calculate an integral\n"),
        Print("Enter your choice (1 or 2): ")
    )
    .unwrap();

    // Read user input
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // Handle the user's choice
    match input {
        "1" => {
            execute!(stdout, Print("\nConnection to `rust_math_lib` still to be established.\n")).unwrap();
            // Placeholder for derivative calculation logic
        }
        "2" => {
            execute!(stdout, Print("\nConnection to `rust_math_lib` still to be established.\n")).unwrap();
            // Placeholder for integral calculation logic
        }
        _ => {
            execute!(stdout, Print("\nInvalid choice. Please restart the program.\n")).unwrap();
        }
    }
}