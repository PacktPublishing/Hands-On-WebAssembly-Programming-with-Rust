/// This file will become our runnable program.
/// As it's the only program in our library's crate, we can run it simply
/// with `cargo run`. If there were multiple programs we would need to add
/// the --bin argument to cargo run, e.g. `cargo run --bin estimate_pi`.

fn main() {
    // const is similar to let. It creates a value that is hard-coded into the
    // program at compile time.
    #[allow(non_snake_case)]
    const N: u64 = 10000000;

    // Run our estimate_pi function and print the result.
    println!("PI is: {}", first_library::estimate_pi(N, 5000));

    // If you're feeling curious, uncomment this line and run the
    // program with `cargo run` to see the "fast" version of
    // estimate_pi. You'll see it completes in a noticeably less time than
    // the line above.
    println!("PI (fast) is: {}", first_library::estimate_pi_fast(N));
}
