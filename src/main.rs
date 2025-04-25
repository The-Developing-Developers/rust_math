//! Entry point for a CLI application that uses the `rust_math_lib` library.
//! This application is a command-line interface (CLI) for performing mathematical
//! calculations provided by the `rust_math_lib` library.
//!
//! Crates used:
//! - figlet-rs: for generating ASCII art text
//! - inquire: for user input prompts
//! - meval: for parsing and evaluating mathematical expressions and numbers scientifically written

use figlet_rs::FIGfont;
use inquire::{Select, Text, error::InquireError};
use meval;

use rust_math_lib::derivatives::Derivative;
use rust_math_lib::integrals::Integral;

/// Main function that serves as the entry point for the CLI application.
///
/// It displays a title and a menu for the user to select between calculations.
/// Based on the user's selection, it calls the appropriate function to perform the calculation.
fn main() {
    print_title(); // Print the title using FIGlet
    println!("Welcome to the Rust Math CLI!\n");

    // Display the main menu and get the user's selection
    match main_menu().as_str() {
        "Integrals" => {
            println!("You selected Integrals.");
            // Call the integrals module or function here
            call_integrals();
        }
        "Derivatives" => {
            println!("You selected Derivatives.");
            // Call the derivatives module or function here
            call_derivatives();
        }
        "Exit" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            println!("Invalid selection. Exiting...");
            std::process::exit(1);
        }
    }
}

/// Prints the title of the application using FIGlet font.
fn print_title() {
    let title = "Rust Math";
    let font = FIGfont::standard().unwrap();
    let figure = font.convert(title).unwrap();
    println!("{}", figure);
}

/// Displays the main menu and prompts the user to select a calculation type.
///
/// # Returns
/// A string representing the user's selection.
fn main_menu() -> String {
    // Define the options for the menu
    let options = vec!["Integrals", "Derivatives", "Exit"];

    // Ask the user to select an option from the menu
    let selected: Result<&str, InquireError> =
        Select::new("Select type of calculation?", options).prompt();

    // Check the result of the selection
    match selected {
        Ok(choice) => {
            return String::from(choice);
        }
        Err(_) => {
            println!("Something went wrong! Exiting...");
            std::process::exit(1);
        }
    }
}

/// Requests the user to input a function, lower and upper bounds, and the number of intervals for integration.
/// It then performs numerical integration and prints the result.
fn call_integrals() {
    // Request user input for function
    let func = Text::new("Insert the function: ").prompt().unwrap();
    // Parse the function string into a meval expression
    let expr: meval::Expr = func.parse().unwrap();
    // Bind the variable 'x' to the expression
    let func = expr.clone().bind("x").unwrap();

    // Request user input for lower bound
    let lower_bound = Text::new("Insert the lower bound: ").prompt().unwrap();
    // Parse the lower bound string into a floating-point number
    let lower_bound = meval::eval_str(lower_bound).unwrap();

    // Request user input for upper bound
    let upper_bound = Text::new("Insert the upper bound: ").prompt().unwrap();
    // Parse the upper bound string into a floating-point number
    let upper_bound = meval::eval_str(upper_bound).unwrap();

    // Request user input for number of intervals
    let num_intervals = Text::new("Insert the number of intervals: ")
        .prompt()
        .unwrap();
    // Parse the number of intervals string into an unsigned 64-bit integer
    let num_intervals = meval::eval_str(num_intervals).unwrap() as u64;

    // Print the user inputs
    println!("Test function: {:?}", expr);
    println!("Lower bound: {}", lower_bound);
    println!("Upper bound: {}", upper_bound);
    println!("Intervals: {}", num_intervals);

    // Perform numerical integration using the Integral struct
    let res = Integral::new(Box::new(func), lower_bound, upper_bound, num_intervals).integrate();

    // Print the result of the integration
    println!("The result of the integral is: {}", res);
}

/// Requests the user to input a function, X coordinate, and increment for derivative calculation.
/// It then performs numerical differentiation and prints the result.
fn call_derivatives() {
    // Request user input for function
    let func = Text::new("Insert the function: ").prompt().unwrap();
    // Parse the function string into a meval expression
    let expr: meval::Expr = func.parse().unwrap();
    // Bind the variable 'x' to the expression
    let func = expr.clone().bind("x").unwrap();

    // Request user input for X coordinate
    let x_coord = Text::new("Insert X coordinate: ").prompt().unwrap();
    // Parse the X coordinate string into a floating-point number
    let x_coord = meval::eval_str(x_coord).unwrap();

    // Request user input for increment
    let increment = Text::new("Insert the increment: ").prompt().unwrap();
    // Parse the increment string into a floating-point number
    let increment = meval::eval_str(increment).unwrap();

    // Print the user inputs
    println!("Test function: {:?}", expr);
    println!("X coordinate: {}", x_coord);
    println!("Increment: {}", increment);

    // Perform numerical differentiation using the Derivative struct
    let res = Derivative::new(Box::new(func), x_coord, increment).forward_difference();

    // Print the result of the differentiation
    println!("The result of the derivate is: {}", res);
}
