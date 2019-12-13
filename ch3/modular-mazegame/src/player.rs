use crate::{Direction, Point};

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
