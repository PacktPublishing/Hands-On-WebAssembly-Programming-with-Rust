#[allow(non_snake_case)]
// Use the rand Rng trait (needed for the .gen() function to be available)
use rand::Rng;

/// Faster version of estimate_pi.
///
/// It still uses Monte Carlo sampling but no longer supports reporting
/// progress on the way. It also generates numbers slightly differently
/// which uses a trait, which readers of Chapter 1 will not yet know about.
///
/// # Arguments
///
/// * n: a 64-bit positive integer, the number of samples to run
/// * display: a 64-bit positive integer, the frequency at which to report
///            the currently estimated value of pi
#[allow(non_snake_case)]
pub fn estimate_pi_fast(N: u64) -> f64 {
    // Prepare a random number generator we'll use for the whole loop
    let mut rng = rand::thread_rng();

    // Initialise the counter
    let mut M: u64 = 0;

    // Run the loop n times
    for _ in 0..N {
        // Get two random numbers from our generator
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        // The same check as before
        if (x*x + y*y) < 1.0 {
            M += 1;
        }
    }

    // The result - our estimate for pi!
    4.0 * (M as f64 / N as f64)
}
