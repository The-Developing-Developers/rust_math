//! Entry point for a CLI application that uses the `rust_math_lib` library.
//! This application is a command-line interface (CLI) for performing mathematical
//! calculations provided by the `rust_math_lib` library.
//!
//! Crates used:
//! - figlet-rs: for generating ASCII art text
//! - inquire: for user input prompts
//! - meval: for parsing and evaluating mathematical expressions and numbers scientifically written
//! - tabled: for displaying results in a table format

use std::time::Instant;

use figlet_rs::FIGfont;
use inquire::error::InquireError;
use inquire::list_option::ListOption;
use inquire::validator::{MinLengthValidator, StringValidator, Validation};
use inquire::{MultiSelect, Select, Text};
use meval;
use tabled;
use tabled::{Table, Tabled};

use rust_math_lib::derivatives::Derivative;
use rust_math_lib::integrals::Integral;

/// Struct to hold the statistics of the calculations performed.
/// Used to display the results in a table format.
///
/// # Traits:
/// - `Tabled`: This trait is used to format the struct for display in a table.
///
/// # Fields:
/// - `algorithm`: The name of the algorithm used for the calculation.
/// - `process_time`: The time taken to perform the calculation.
/// - `result`: The result of the calculation.
#[derive(Tabled)]
struct CalculationStats {
    #[tabled(rename = "Algorithm")]
    pub algorithm: String,
    #[tabled(rename = "Process Time")]
    pub process_time: String,
    #[tabled(rename = "Result")]
    pub result: f64,
}

/// Main function that serves as the entry point for the CLI application.
///
/// It displays a title and a menu for the user to select between calculations.
/// Based on the user's selection, it calls the appropriate function to perform the calculation.
fn main() {
    print_title(); // Print the title using FIGlet
    println!("Welcome to the Rust Math CLI!\n");

    // Display the main menu and get the user's selection
    loop {
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
        Select::new("Select type of calculation:", options).prompt();

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

/// Prompts the user to ask if they want to perform another calculation.
///
fn ask_for_another_calculation() -> bool {
    // Ask the user if they want to perform another calculation
    let another = Text::new("Do you want to perform another calculation?")
        .with_default("yes")
        .with_placeholder("yes/no")
        .prompt()
        .unwrap();
    match another.to_lowercase().as_str() {
        "y" | "yes" => return true,
        _ => return false,
    }
}

/// Converts a vector of CalculationStats into a table format for display.
fn get_stats_table(stats: &Vec<CalculationStats>) -> Table {
    let mut table = Table::new(stats);
    table.with(tabled::settings::Style::rounded());
    table.modify(
        tabled::settings::object::Columns::last(),
        tabled::settings::Alignment::right(),
    );
    table
}

/// Requests the user to input a function, lower and upper bounds, and the number of intervals for integration.
/// It then performs numerical integration and prints the result.
fn call_integrals() {
    // Expression validator for the function input
    let expr_validator = |input: &str| match input.parse::<meval::Expr>() {
        Ok(expr) => match expr.bind("x") {
            Ok(_) => Ok(Validation::Valid),
            Err(e) => return Ok(Validation::Invalid(e.into())),
        },
        Err(e) => Ok(Validation::Invalid(e.into())),
    };

    // Define the default values for the user inputs
    let mut default_func = "sin(x)".to_string();
    let mut default_lower_bound = "0".to_string();
    let mut default_upper_bound = "pi".to_string();
    let mut default_num_intervals = "1e7".to_string();

    loop {
        // Request user input for function
        let func = Text::new("Insert the function")
            .with_default(&default_func)
            .with_validator(expr_validator)
            .prompt()
            .unwrap();
        default_func = func.clone();
        // Parse the function string into a meval expression
        let expr: meval::Expr = func.parse().unwrap();
        // Bind the variable 'x' to the expression
        let func = expr.clone().bind("x").unwrap();

        // Request user input for lower bound
        let lower_bound = Text::new("Insert the lower bound")
            .with_default(&default_lower_bound)
            .prompt()
            .unwrap();
        default_lower_bound = lower_bound.clone();
        // Parse the lower bound string into a floating-point number
        let lower_bound = meval::eval_str(lower_bound).unwrap();

        // Request user input for upper bound
        let upper_bound = Text::new("Insert the upper bound")
            .with_default(&default_upper_bound)
            .prompt()
            .unwrap();
        default_upper_bound = upper_bound.clone();
        // Parse the upper bound string into a floating-point number
        let upper_bound = meval::eval_str(upper_bound).unwrap();

        // Request user input for number of intervals
        let num_intervals = Text::new("Insert the number of intervals")
            .with_default(&default_num_intervals)
            .prompt()
            .unwrap();
        default_num_intervals = num_intervals.clone();
        // Parse the number of intervals string into an unsigned 64-bit integer
        let num_intervals = meval::eval_str(num_intervals).unwrap() as u64;

        // Print the user inputs
        println!("Test function: {}", default_func);
        println!("Lower bound: {}", default_lower_bound);
        println!("Upper bound: {}", default_upper_bound);
        println!("Intervals: {}", default_num_intervals);

        // Perform numerical integration using the Integral struct
        let res =
            Integral::new(Box::new(func), lower_bound, upper_bound, num_intervals).integrate();

        // Print the result of the integration
        println!("The result of the integral is: {}", res);

        // Ask the user if they want to perform another calculation
        if !ask_for_another_calculation() {
            break;
        }
    }
}

/// Requests the user to input a function, X coordinate, and increment for derivative calculation.
/// It then performs numerical differentiation and prints the result.
fn call_derivatives() {
    // Define the options for the algorithms
    let algorithms_options = vec![
        ListOption::new(0, "Forward Difference"),
        ListOption::new(1, "Central Difference"),
        ListOption::new(2, "Backward Difference"),
    ];

    // Expression validator for the function input
    let expr_validator = |input: &str| match input.parse::<meval::Expr>() {
        Ok(expr) => match expr.bind("x") {
            Ok(_) => Ok(Validation::Valid),
            Err(e) => return Ok(Validation::Invalid(e.into())),
        },
        Err(e) => Ok(Validation::Invalid(e.into())),
    };

    // Define the default values for the user inputs
    let mut default_algorithms: Vec<usize> = vec![0, 1, 2];
    let mut default_func = "sin(x)".to_string();
    let mut default_x_coord = "0".to_string();
    let mut default_increment = "1e-7".to_string();

    loop {
        // Request user input for algorithm
        let algorithms = MultiSelect::new(
            "Select which algorithms to run:",
            algorithms_options.clone(),
        )
        .with_default(&default_algorithms)
        .with_validator(MinLengthValidator::new(1))
        .with_help_message("Please, select at least one algorithm!")
        .prompt()
        .unwrap();
        default_algorithms = algorithms.iter().map(|x| x.index).collect();

        // Request user input for function
        let func = Text::new("Insert the function")
            .with_default(&default_func)
            .with_validator(expr_validator)
            .prompt()
            .unwrap();
        default_func = func.clone();
        // Parse the function string into a meval expression
        let expr: meval::Expr = func.parse().unwrap();
        // Bind the variable 'x' to the expression
        let func = expr.clone().bind("x").unwrap();

        // Request user input for X coordinate
        let x_coord = Text::new("Insert the X coordinate")
            .with_default(&default_x_coord)
            .prompt()
            .unwrap();
        default_x_coord = x_coord.clone();
        // Parse the X coordinate string into a floating-point number
        let x_coord = meval::eval_str(x_coord).unwrap();

        // Request user input for increment
        let increment = Text::new("Insert the increment")
            .with_default(&default_increment)
            .prompt()
            .unwrap();
        default_increment = increment.clone();
        // Parse the increment string into a floating-point number
        let increment = meval::eval_str(increment).unwrap();

        // Print the user inputs
        println!("Test function: {}", default_func);
        println!("X coordinate: {}", default_x_coord);
        println!("Increment: {}", default_increment);

        // Perform numerical differentiation using the Derivative struct
        let mut derivative = Derivative::new(Box::new(func), x_coord, increment);
        let mut stats: Vec<CalculationStats> = vec![];
        algorithms.iter().for_each(|algorithm| {
            println!("Using algorithm: {}", algorithm.value);
            let res;
            let process_time = Instant::now();
            match algorithm.value {
                "Forward Difference" => {
                    res = derivative.forward_difference();
                }
                "Central Difference" => {
                    res = derivative.central_difference();
                }
                "Backward Difference" => {
                    res = derivative.backward_difference();
                }
                _ => {
                    println!("Invalid algorithm selected. Using Forward Difference as default.");
                    res = derivative.forward_difference();
                }
            }
            let process_time = process_time.elapsed();
            stats.push(CalculationStats {
                algorithm: algorithm.value.to_string(),
                process_time: format!("{:?}", process_time),
                result: res,
            });
            // Print the result of the differentiation
            // println!("The result of the derivate is: {}", res);
        });

        // Print the results of the differentiation
        println!("\nResults of the differentiation:");
        println!("{}", get_stats_table(&stats));

        // Ask the user if they want to perform another calculation
        if !ask_for_another_calculation() {
            break;
        }
    }
}
