//! Entry point for a CLI application that uses the `rust_math_lib` library.

use figlet_rs::FIGfont;
use inquire::{Select, Text, error::InquireError};
use meval;

use rust_math_lib::integrals::Integral;

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

    let func = Text::new("Insert the function: ").prompt().unwrap();
    let expr: meval::Expr = func.parse().unwrap();
    let func = expr.clone().bind("x").unwrap();

    let lower_bound = Text::new("Insert the lower bound: ").prompt().unwrap();
    let lower_bound = meval::eval_str(lower_bound).unwrap();

    let upper_bound = Text::new("Insert the upper bound: ").prompt().unwrap();
    let upper_bound = meval::eval_str(upper_bound).unwrap();

    let num_intervals = Text::new("Insert the number of intervals: ")
        .prompt()
        .unwrap();
    let num_intervals = meval::eval_str(num_intervals).unwrap() as u64;

    println!("Test function: {:?}", expr);
    println!("Lower bound: {lower_bound}");
    println!("Upper bound: {upper_bound}");
    println!("Intervals: {num_intervals}");

    let res = Integral::new(Box::new(func), lower_bound, upper_bound, num_intervals).integrate();
    println!("The result of the integral is: {}", res);
}
