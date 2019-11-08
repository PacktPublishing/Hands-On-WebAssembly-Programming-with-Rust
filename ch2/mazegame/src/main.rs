// Declare modules for our other files.
//
// Note: if you're looking at this code having just finished Chapter 2 of
// the book, you won't have learned about modules yet.
//
// Don't fret - they're a simple feature of the Rust language which allows us
// to break our code up so it's not all in one enormous file.
//
// We'll talk about more about modules as the first topic in Chapter 3!
mod direction;
mod input;
mod player;
mod point;

// Bring the Direction, Player, and Point types into this module
// so that we can refer to them without having to prefix them with
// their module.
// (E.g. we can now write "Direction" instead of "direction::Direction").
use direction::Direction;
use player::Player;
use point::Point;

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
        let user_input = input::get_input_line(prompt);

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
