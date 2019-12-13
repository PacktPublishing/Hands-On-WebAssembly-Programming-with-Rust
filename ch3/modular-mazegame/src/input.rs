/// Read a line of user entry from the command line, prompting
/// the user with the provided string.
///
/// The function takes one argument, "prompt", which is the text
/// to display to the command line when asking the user for input.
/// We don't need to save the prompt for later, so its type can be
/// borrowed text data (&str).
///
/// The return value of the function is the text which the user typed in.
/// Because this is a new piece of data which we've created while running
/// this function, it is owned text data (String).
pub fn get_input_line(prompt: &str) -> String {
    use std::io::Write;

    // Create a buffer to store the contents we will read
    let mut buf = String::new();

    // Note: the write!() macro, flush() function and read_line() function
    // return values of type Result - an enum built into the Rust language
    // with two variants:
    //   Ok - to report success, and
    //   Err - to report failure.
    //
    // We'll learn more about this enum in Chapter 3. For the moment we
    // consume it immediately to get the successful value by using the
    // Result::expect() method. Expect always returns the success case.
    //
    // How? By crashing the program with the specified messsage if the Result
    // is an error!
    //
    // This is a very sloppy way to do error handling in Rust. It's fine for now,
    // but we'll touch on a better way in Chapter 3.

    // Get a handle to the program stdout, and then write the
    // prompt. Flushing ensures that the prompt appears immediately,
    // as we're then going to wait for the player to input something.
    let mut stdout = std::io::stdout();
    write!(stdout, "{} ", prompt).expect("Failed to write stdout");
    stdout.flush().expect("Failed to flush stdout");

    // Wait for the player to enter a line of characters
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    // Make a copy of the input characters without any whitespace
    // at the start or end. This is our function output.
    //
    // Note: the output of buf.trim() is a &str - borrowed text data.
    // Where is it borrowed from? buf!
    // We could learn this from the signature of .trim() :
    //
    //     pub fn trim(&self) -> &str
    //
    // Because there's only only borrowed input (&self), the borrowed data
    // coming out of the function must come from that source.
    //
    // And because .trim() is a method (we can see that because it's got
    // the special self input), we know that self is the object on which the
    // method is being called - in this case that's buf.
    //
    // In summary, buf.trim() is borrowed text data which refers to just the
    // "middle" bit of buf between any whitespace at the start and end.
    //
    // We need to send the result of buf.trim() to String::from() in order to
    // create an owned copy of this text data, as our borrow of buf can't
    // continue beyond this point: buf is about to become inaccessible,
    // meaning its lifetime is over and it will shortly be deleted.
    String::from(buf.trim())
}
