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
