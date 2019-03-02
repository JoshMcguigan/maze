use std::fmt;
use std::fmt::Write as _;

/// +---+---+---+
/// | 02| 12| 22|
/// +---+---+---+
/// | 01| 11| 21|
/// +---+---+---+
/// | 00| 10| 20|
/// +---+---+---+
///
/// Cell positions labeled above as xy
/// Walls marked below increasing from 0..N
///
/// Vertical Walls
/// 8 11
/// 7 10
/// 6 9
///
/// Horizontal Walls
/// 3 4 5
/// 0 1 2
///

#[derive(Debug, Copy, Clone)]
enum Wall {
    Open,
    Closed,
}

struct Maze {
    height: u32,
    width: u32,
    walls: Vec<Wall>,
}

struct WallIndexesForCell {
    north: Option<usize>,
    east: Option<usize>,
    south: Option<usize>,
    west: Option<usize>,
}

struct MazeTraveler {
    current_x: u32,
    current_y: u32,
    maze: Maze,
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

    /// Gets the index into the wall array which stores the wall to the north of the
    /// cell at (x, y). Returns None for cells in the top row.
    fn north_wall_index_for_cell(&self, x: u32, y: u32) -> Option<usize> {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        // return None for top row of cells
        if y >= self.height - 1 { return None; }

        let index = (x + y * self.width) as usize;
        debug_assert!(index < self.walls.len());
        Some(index)
    }

    /// Gets the index into the wall array which stores the wall to the east of the
    /// cell at (x, y). Returns None for cells in the right-most row.
    fn east_wall_index_for_cell(&self, x: u32, y: u32) -> Option<usize> {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        // return None for right-most row of cells
        if x >= self.width - 1 { return None; }

        // vertical segments are stored after all of the horizontal segments
        let num_horizontal_segments = (self.height - 1) * self.width;
        let index = (num_horizontal_segments + (y + x * self.height)) as usize;
        debug_assert!(index < self.walls.len());
        Some(index)
    }

    fn wall_indexes_for_cell(&self, x: u32, y: u32) -> WallIndexesForCell {
        let south = match (x, y) {
            // cells at bottom of maze do not have south wall
            (x, 0) => None,
            // get south wall index by going one cell down and getting north wall index
            (x, y) => self.north_wall_index_for_cell(x, y - 1),
        };

        let west = match (x, y) {
            // walls at left of maze have no west wall
            (0, y) => None,
            // get west wall index by going one cell left and getting east wall index
            (x, y) => self.east_wall_index_for_cell(x - 1, y),
        };

        WallIndexesForCell {
            north: self.north_wall_index_for_cell(x, y),
            east: self.east_wall_index_for_cell(x, y),
            south,
            west,
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

impl MazeTraveler {
    fn new(maze: Maze) -> Self {
        MazeTraveler {
            current_x: 0,
            current_y: 0,
            maze,
        }
    }

    fn release(self) -> Maze {
        self.maze
    }

//    fn open_north(&mut self) {
//        self.maze[0] = Wall::Open;
//    }
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

    #[test]
    fn wall_indexes_for_cell_00() {
        let maze = Maze::new(3, 3);
        let wall_indexes = maze.wall_indexes_for_cell(0, 0);

        assert_eq!(Some(0), wall_indexes.north);
        assert_eq!(Some(6), wall_indexes.east);
        assert_eq!(None, wall_indexes.south);
        assert_eq!(None, wall_indexes.west);
    }

    #[test]
    fn wall_indexes_for_cell_11() {
        let maze = Maze::new(3, 3);
        let wall_indexes = maze.wall_indexes_for_cell(1, 1);

        assert_eq!(Some(4), wall_indexes.north);
        assert_eq!(Some(10), wall_indexes.east);
        assert_eq!(Some(1), wall_indexes.south);
        assert_eq!(Some(7), wall_indexes.west);
    }

    #[test]
    fn wall_indexes_for_cell_22() {
        let maze = Maze::new(3, 3);
        let wall_indexes = maze.wall_indexes_for_cell(2, 2);

        assert_eq!(None, wall_indexes.north);
        assert_eq!(None, wall_indexes.east);
        assert_eq!(Some(5), wall_indexes.south);
        assert_eq!(Some(11), wall_indexes.west);
    }

//    #[test]
//    fn open_north() {
//        let maze = Maze::new(3, 3);
//        let mut traveler = MazeTraveler::new(maze);
//
//        traveler.open_north();
//
//        assert_snapshot_matches!("open_north", traveler.release().as_string());
//    }
}
