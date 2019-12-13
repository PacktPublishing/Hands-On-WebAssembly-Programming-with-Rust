fn main() {
    // Create the starting player state
    let mut player = Player::new();

    // Also reserve these two names as mutable variables. We're
    // not initialising them yet (there's no = 0, for example).
    // - the Rust compiler will ensure that we initialise
    // them before we try to read them later on.
    let mut backtrack_direction: Direction;
    let mut last_distance: u64;

    // Generate the locations of the key and exit

    let key_location = Point {
        x: random_integer(-5, 5),
        y: random_integer(-5, 5),
    };

    let exit_location = Point {
        x: random_integer(-5, 5),
        y: random_integer(-5, 5),
    };

    // Create a map - a container linking a key to a value. The keys are
    // directions, and the values are bools. We know this from the
    // generic parameters we have specified: Direction and bool.
    let mut possible_directions: std::collections::HashMap<Direction, bool> =
        std::collections::HashMap::new();

    // For the player's first move they're allowed to go any direction.
    for direction in Direction::all() {
        possible_directions.insert(direction, true);
    }

    // Output some instructions to the player.
    // The r#" ... "# allows us to write a string that spans
    // over multiple lines.
    println!(
        r#"You wake up to find yourself in a mysterious maze.

Everywhere you step, the walls twist and shift. You can always
retrace your last step, but nothing else seems constant.

Nearby you can sense the presence of a key. Can you find it
and escape the maze?
"#
    );

    // We're now ready to start! Let's loop until we have to end.
    // (The end points are the two break statements).
    loop {
        // Figure out where the player is currently heading.
        let (target, target_location) = if player.has_key {
            ("exit", exit_location)
        } else {
            ("key", key_location)
        };

        // Store the current distance from the player to the key/exit,
        // before they move.
        last_distance = player.location.distance(target_location);

        // Let the player know which ways they can't go.
        for direction in Direction::all() {
            if !possible_directions[&direction] {
                println!("  There is a wall to the {}", direction.name())
            }
        }

        // Ask the player for a direction
        let direction = match ask_player_direction() {
            Some(direction) => direction,
            None => {
                // The player responded Q to quit.
                println!("Bye!");
                break;
            }
        };

        // The player gave a direction, check if they can go that way.
        let can_travel = possible_directions[&direction];
        if can_travel {
            // Yes - make the move
            backtrack_direction = direction.opposite();
            player.step(direction);
        } else {
            // No - give them an error, and re-start the game loop
            println!("  You can't go that way, there's a wall!");
            continue;
        }

        // The player has moved - figure out the new distance to the key/exit
        let distance = player.location.distance(target_location);

        if distance == 0 {
            // They got there!
            println!("You found the {}!", target);
            if player.has_key {
                // It's the exit, yay!
                println!("You have escaped, congratulations!");
                break;
            } else {
                // It's the key!
                println!("You can now sense the exit...");
                player.has_key = true;
            }
        } else if distance < last_distance {
            println!("  You sense you are getting closer to the {}...", target);
        } else if distance == last_distance {
            println!(
                "  You sense the {} is just as far as it was before...",
                target
            );
        } else {
            println!("  You sense you are getting further from the {}...", target);
        }

        // Shift the walls now that the player's made a step.
        for direction in Direction::all() {
            if direction == backtrack_direction {
                // The player can always undo their last step.
                possible_directions.insert(direction, true);
            } else {
                // For the other directions, let it be random if there's a wall.
                //
                // rand::random() is a generic function, and in this case Rust can
                // deduce that we want a bool output to go into our HashMap<Direction, bool>.
                //
                // rand::random() is set up so that in this case of a bool we get 50/50
                // chance of each of true and false.
                possible_directions.insert(direction, rand::random());
            }
        }
    }
}

/// A coordinate in the maze
/// Deriving the Copy and Clone traits simplifies ownership of
/// variables of the Point type. We'll explain why when we go
/// over these traits in chapter 3.
#[derive(Copy, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    /// Method to calculate distance (in steps) to a target point.
    pub fn distance(self, other: Point) -> u64 {
        let x_diff = (self.x - other.x).abs() as u64;
        let y_diff = (self.y - other.y).abs() as u64;
        x_diff + y_diff
    }
}


/// The player state
///
/// In the book we also talked about the player collecting weapons
/// and fighting off monsters. While cool, that would add a lot of
/// bulk to the code without really showing new teaching concepts.
/// For this reason, we don't have weapons in this little maze game.
pub struct Player {
    pub location: Point,
    pub has_key: bool,
}

impl Player {
    /// Associated function to create a new player
    pub fn new() -> Player {
        Player {
            location: Point { x: 0, y: 0 },
            has_key: false,
        }
    }

    /// Method to update the player's location according
    /// to taking a single step in the given direction.
    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                self.location.y += 1;
            }
            Direction::South => {
                self.location.y -= 1;
            }
            Direction::East => {
                self.location.x += 1;
            }
            Direction::West => {
                self.location.x -= 1;
            }
        }
    }
}


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


/// A function to generate a random integer ranging from lower to upper.
/// E.g. random_integer(-2, 5) will pick any of -2, -1, 0, 1, 2, 3, 4, or 5.
fn random_integer(lower: i64, upper: i64) -> i64 {
    // Panic (crash the program) if lower is higher than upper.
    // By checking this we can be sure that upper >= lower for the
    // rest of the function.
    //
    // A less destructive option to crashing the program would be
    // for random_integer to output a `Result` type, e.g. Result<i64, String>.
    // Then we could return the Err variant here rather than crashing
    // the program (and let code which uses the function decide whether to
    // recover from the error or crash).
    assert!(upper >= lower);

    // Take a random number from 0 up to (but not including) 1.
    let fraction: f64 = rand::random();

    // The number of integers in the range from lower to upper.
    let n = (upper - lower) + 1;

    // Rescale fraction to be from 0 to (but not including) n.
    // Flooring the result means we get an integer from 0 to n-1.
    // Adding lower to that will result in a number from lower to upper,
    // as upper = (n - 1) + lower, which we can see by rearranging
    // the calculation for n above.
    (n as f64 * fraction).floor() as i64 + lower
}

/// A function to ask the user to pick the direction to travel.
/// The user can also enter Q to quit, in which case None will be
/// returned.
fn ask_player_direction() -> Option<Direction> {
    // We're going to keep looping until the user gives us valid input.
    loop {
        // The question we will ask
        let prompt = "Which direction will you go (N/S/E/W, or Q to quit)?";

        // Request input from the program.
        let user_input = get_input_line(prompt);

        // If the user didn't give exactly one character,
        // let them know we reject the input and restart the loop.
        if user_input.len() != 1 {
            println!("Error: must give a direction");
            continue;
        }

        // If the user input matches, we will stop the function by
        // returning a value.
        // Otherwise, we'll print an error, and then the loop will
        // restart.
        match user_input.to_lowercase().chars().nth(0) {
            Some('n') => return Some(Direction::North),
            Some('s') => return Some(Direction::South),
            Some('e') => return Some(Direction::East),
            Some('w') => return Some(Direction::West),
            Some('q') => return None,
            _ => println!("Error: direction must be N,S,E, or W"),
        }
    }
}

/// A simple enum to represent to compass directions
///
/// We need to derive() a few traits to allow the direction
/// to be used in the HashMap, and to simplify ownership of
/// variables of the Direction type. We'll explain all this
/// in Chapter 3 - we just thought it wise to mention this
/// derive() line in case you wonder what it means.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Associated function to get a list of all possible directions
    pub fn all() -> Vec<Direction> {
        let mut directions = Vec::new();
        directions.push(Direction::North);
        directions.push(Direction::South);
        directions.push(Direction::East);
        directions.push(Direction::West);
        directions

        // Note: using the vec! macro, we could have done:
        //
        // vec![
        //     Direction::North,
        //     Direction::South,
        //     Direction::East,
        //     Direction::West
        // ]
        //
        // as a slightly shorter way to build our Vec.
        //
        // It would be totally equivalent to the actual code we've written.
    }

    /// Method to convert a direction to a user-friendly form for display
    ///
    /// The output type is &'static str, which we know to be borrowed text
    /// data which exists for the 'static lifetime. As a reminder, the
    /// 'static lifetime is the lifetime of data which exists for the
    /// whole duration of the program (i.e. hard-coded data).
    pub fn name(self) -> &'static str {
        match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        }
    }

    /// Method to flip a direction: North <-> South, East <-> West
    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}
