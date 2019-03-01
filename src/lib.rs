use std::fmt;
use std::fmt::Write as _;

/// +---+---+---+
/// |   |   |   |
/// +---+---+---+
/// |   |   |   |
/// +---+---+---+
/// |   |   |   |
/// +---+---+---+

/// Vertical Walls
/// 1 1
/// 1 1
/// 1 1
///
/// Horizontal Walls
/// 1 1 1
/// 1 1 1
///

#[derive(Debug, Copy, Clone)]
enum Wall {
    Open,
    Closed,
}

struct Maze {
    height: u32,
    width: u32,
    walls: Vec<Wall>
}

impl Maze {
    /// Create a new maze of all closed walls
    /// Panics if height or width are < 1
    fn new(height: u32, width: u32) -> Self {
        let num_vertical_segments = (width - 1) * height;
        let num_horizontal_segments = (height - 1) * width;
        let total_walls = num_vertical_segments + num_horizontal_segments;
        let walls = vec![Wall::Closed; total_walls as usize];

        Maze {
            height,
            width,
            walls,
        }
    }

    fn as_string(&self) -> String {
        let horizontal_wall_segment = "+---";
        let vertical_wall_segment = "|   ";
        let mut horizontal = String::new();
        let mut vertical = String::new();

        for _ in 0..self.width {
            horizontal += horizontal_wall_segment;
            vertical += vertical_wall_segment;
        }

        horizontal += "+";
        vertical += "|";

        let mut total = String::new();

        for _ in 0..self.height {
            let _ = writeln!(total, "{}", horizontal);
            let _ = writeln!(total, "{}", vertical);
        }
        let _ = write!(total, "{}", horizontal); // last row should not have newline

        total
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot_matches;

    #[test]
    fn new_3x3() {
        let maze = Maze::new(3, 3);

        // todo can insta use test name as name
        // todo can insta handle printing if type impls Display
        assert_snapshot_matches!("new_3x3", maze.as_string());
    }
}
