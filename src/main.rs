//! Entry point for a CLI application that uses the `rust_math_lib` library.

use rust_math_lib::integrals::Integral;
use rust_math_lib::utils::colours::{*};

fn main() {
    println!("****************************** START ******************************\n");

    // ---- Example of usage of the `Integral` struct ---- //

    // ---- Square function ---- //

    let lower = 0.0;
    let upper = 3.0;
    let function = |x: f64| x * x;
    let intervals_list = [1e3 as u32, 1e4 as u32, 1e5 as u32, 1e6 as u32, 1e7 as u32, 1e8 as u32];
    println!("Integrating function {}x^2{} from {}{}{} to {}{}{}, using different numbers of intervals.", YELLOW, RESET, GREEN, lower, RESET, GREEN, upper, RESET);

    // Iterating over the list of intervals without references (`intervals_list` is an array of primitive values, which implement the `Copy` trait and are cheap to copy).
    for intervals in intervals_list {
        if intervals > 1e5 as u32 {
            println!("\nUsing {}{:e}{} intervals.", MAGENTA, intervals as f64, RESET);
        } else {
            println!("\nUsing {}{}{} intervals.", MAGENTA, intervals, RESET);
        }
        let result = Integral::new(function, lower, upper, intervals as u64).integrate();
        println!("The result is: {}{:.6}{}", GREEN, result, RESET);
    }

    // ---- Sine function ---- //

    let lower = 0.0;
    let upper = std::f64::consts::PI;
    let function = |x: f64| (x).sin();
    println!("\nIntegrating function {}sin(x){} from {}{}{} to {}pi{}, using different numbers of intervals.", YELLOW, RESET, GREEN, lower, RESET, GREEN, RESET);

    // Same iteration as above, but using references (`&intervals_list` is a slice of references). Not necessary in this case, but it's a good practice in general.
    for intervals in &intervals_list {
        if *intervals > 1e5 as u32 {
            println!("\nUsing {}{:e}{} intervals.", MAGENTA, *intervals as f64, RESET);
        } else {
            println!("\nUsing {}{}{} intervals.", MAGENTA, *intervals, RESET);
        }
        let result = Integral::new(function, lower, upper, *intervals as u64).integrate();
        println!("The result is: {}{:.6}{}", GREEN, result, RESET);
    }

    println!("\n******************************  END  ******************************");
}