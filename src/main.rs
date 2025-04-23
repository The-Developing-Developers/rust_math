//! Entry point for a CLI application that uses the `rust_math_lib` library.

use figlet_rs::FIGfont;
use inquire::{Select, error::InquireError};
use meval;

fn main() {
    print_title();
    println!("Welcome to the Rust Math CLI!");

    let next_menu = main_menu();
    match next_menu.as_str() {
        "Integrals" => {
            println!("You selected Integrals.");
            // Call the integrals module or function here
            call_integrals();
        }
        "Derivatives" => {
            println!("You selected Derivatives.");
            // Call the derivatives module or function here
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

fn print_title() {
    let title = "Rust Math";
    let font = FIGfont::standard().unwrap();
    let figure = font.convert(title).unwrap();
    println!("{}", figure);
}

fn main_menu() -> String {
    let options = vec!["Integrals", "Derivatives", "Exit"];

    let selected: Result<&str, InquireError> =
        Select::new("Select type of calculation?", options).prompt();

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

fn call_integrals() {
    let mut ctx = meval::Context::new();
    ctx.var("x", 2.0);

    let expr = "x^2";
    let res = meval::eval_str_with_context(expr, ctx).unwrap();

    // Print the result
    println!("The result of the integral is: {}", res);
}
