/// A coordinate in the maze
/// Deriving the Copy and Clone traits simplifies ownership of
/// variables of the Point type - we'll explain why when we
/// get onto the topic of ownership.
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
