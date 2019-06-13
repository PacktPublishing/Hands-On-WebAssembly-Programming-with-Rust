#[allow(non_snake_case)]

/// Estimate pi using Monte Carlo sampling
///
/// # Arguments
///
/// * num_samples: a 64-bit positive integer, the number of samples to run
/// * display: a 64-bit positive integer, the frequency at which to report
///            the currently estimated value of pi
pub fn estimate_pi(num_samples: u64, display: u64) -> f64 {
    // Initialise two variables we will use for our working
    let mut M: u64 = 0;
    let mut N: u64 = 0;

    // Initialise the output variable pi
    let mut pi: f64 = 0.0;

    // Loop num_samples times
    for _ in 0..num_samples {
        // Each time increase N
        N += 1;

        // Prepare two random numbers between 0 and 1
        let x: f64 = rand::random();
        let y: f64 = rand::random();

        // Check if the point (x, y) is inside the quarter-circle
        // of radius 1
        if (x*x + y*y) < 1.0 {
            // If it is, then increase our count M
            M += 1;
        }

        // Update our estimate for pi
        pi = 4.0 * (M as f64 / N as f64);

        // If this is time to display our progress, print some output
        if (N % display) == 0 {
            println!("{}: {}", N, pi);
        }
    }

    // Return the value of pi from the function
    pi
}

// These aren't in the print reproduction. `estimate_pi_fast` is an
// equivalent function to estimate_pi with a couple of tweaks designed
// to aid performance, but at the cost of needing a little more
// Rust knowledge (traits) than we want to introduce in Chapter 1.
mod estimate_pi_fast;
pub use estimate_pi_fast::estimate_pi_fast;
