//! Entry point for a CLI application that uses the `rust_math_lib` library.

use rust_math_lib::integrals::Integrator;

fn main() {
    println!("****************************** START ******************************\n");

    // Example of usage of the `Integrator` struct

    // Square function

    let lower = 0.0;
    let upper = 3.0;
    let function = |x: f64| x * x;
    let intervals_list = [1e3 as u32, 1e4 as u32, 1e5 as u32, 1e6 as u32, 1e7 as u32, 1e8 as u32];
    println!("Integrating function x^2 from {} to {}, using different numbers of intervals.", lower, upper);

    for intervals in &intervals_list {
        if *intervals > 1e5 as u32 {
            println!("\nUsing {:e} intervals.", *intervals as f64);
        } else {
            println!("\nUsing {} intervals.", *intervals);
        }
        let result = Integrator::integrate(function, lower, upper, *intervals);
        println!("The result is: {:.6}", result);
    }

    // Sine function

    let lower = 0.0;
    let upper = std::f64::consts::PI;
    let function = |x: f64| (x).sin();
    println!("\nIntegrating function sin(x) from {} to pi, using different numbers of intervals.", lower);

    for intervals in &intervals_list {
        if *intervals > 1e5 as u32 {
            println!("\nUsing {:e} intervals.", *intervals as f64);
        } else {
            println!("\nUsing {} intervals.", *intervals);
        }
        let result = Integrator::integrate(function, lower, upper, *intervals);
        println!("The result is: {:.6}", result);
    }

    println!("\n******************************  END  ******************************");
}