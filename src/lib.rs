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

const LINE_ENDING: &'static str = "\n";

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

    fn south_wall_index_for_cell(&self, x: u32, y: u32) -> Option<usize> {
        match (x, y) {
            // cells at bottom of maze do not have south wall
            (x, 0) => None,
            // get south wall index by going one cell down and getting north wall index
            (x, y) => self.north_wall_index_for_cell(x, y - 1),
        }
    }

    // todo perhaps this method can be removed?
    fn wall_indexes_for_cell(&self, x: u32, y: u32) -> WallIndexesForCell {
        let west = match (x, y) {
            // walls at left of maze have no west wall
            (0, y) => None,
            // get west wall index by going one cell left and getting east wall index
            (x, y) => self.east_wall_index_for_cell(x - 1, y),
        };

        WallIndexesForCell {
            north: self.north_wall_index_for_cell(x, y),
            east: self.east_wall_index_for_cell(x, y),
            south: self.south_wall_index_for_cell(x, y),
            west,
        }
    }

    fn as_string(&self) -> String {

        let horizontal_wall_segment = "+---";
        let vertical_wall_segment = "|   ";
        let mut horizontal_maze_edge = String::new();
        let mut vertical = String::new();

        for _ in 0..self.width {
            horizontal_maze_edge += horizontal_wall_segment;
            vertical += vertical_wall_segment;
        }

        horizontal_maze_edge += "+";
        vertical += "|";


        let mut total = String::new();

        // add top maze edge
        total += &horizontal_maze_edge;

        for y in (0..self.height).rev() {

            total += LINE_ENDING;

            // add left maze edge
            total += "|   ";

            for x in 0..self.width {
                // for each cell add east wall
                let wall_index = self.east_wall_index_for_cell(x, y);

                if let Some(index) = wall_index {
                    let segment = match self.walls[index] {
                        Wall::Open => "    ",
                        Wall::Closed => "|   ",
                    };

                    total += segment;
                } else {
                    // you've reached the edge of the maze
                    total += "|";
                }
            }

            // insert newline between vertical walls and horizontal walls
            total += LINE_ENDING;

            for x in 0..self.width {
                // for each cell add south wall

                // todo consider cleaning this up by adding helper method to get wall
                //     state directly rather than index
                let wall_index = self.south_wall_index_for_cell(x, y);

                if let Some(index) = wall_index {
                    let segment = match self.walls[index] {
                        Wall::Open => "+   ",
                        Wall::Closed => "+---",
                    };

                    total += segment;
                } else {
                    // you've reached the edge of the maze
                    total += "+---";
                }
            }

            total += "+";
        }

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

    fn open_north_wall(&mut self) {
        let index = self.maze.north_wall_index_for_cell(self.current_x, self.current_y);

        match index {
            Some(index) => self.maze.walls[index] = Wall::Open,
            None => {}, // trying to open through the edge of the map is currently a no-op
        };
    }

    fn open_east_wall(&mut self) {
        let index = self.maze.east_wall_index_for_cell(self.current_x, self.current_y);

        match index {
            Some(index) => self.maze.walls[index] = Wall::Open,
            None => {}, // trying to open through the edge of the map is currently a no-op
        };
    }

    fn move_to_next_cell(self) -> Option<Self> {
        let MazeTraveler {
            current_x: x,
            current_y: y,
            maze
        } = self;

        if x < maze.width - 1 {
            // have not hit right-most wall, increment x
            Some(MazeTraveler {
                current_x: x + 1,
                current_y: y,
                maze: maze
            })
        } else if x == maze.width - 1 && y < maze.height - 1 {
            // on right-most wall, but not in top row
            // wrap x and increment y
            Some(MazeTraveler {
                current_x: 0,
                current_y: y + 1,
                maze: maze
            })
        } else {
            None
        }
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

    #[test]
    fn open_north_wall() {
        let maze = Maze::new(3, 3);
        let mut traveler = MazeTraveler::new(maze);

        traveler.open_north_wall();

        assert_snapshot_matches!("open_north_wall", traveler.release().as_string());
    }

    #[test]
    fn open_east_wall() {
        let maze = Maze::new(3, 3);
        let mut traveler = MazeTraveler::new(maze);

        traveler.open_east_wall();

        assert_snapshot_matches!("open_east_wall", traveler.release().as_string());
    }

    #[test]
    fn next_cell_open_north_wall() {
        let maze = Maze::new(3, 3);
        let traveler = MazeTraveler::new(maze);
        let mut traveler = traveler.move_to_next_cell().unwrap();

        traveler.open_north_wall();

        assert_snapshot_matches!("next_cell_open_north_wall", traveler.release().as_string());
    }

    #[test]
    fn next_cell_open_east_wall() {
        let maze = Maze::new(3, 3);
        let traveler = MazeTraveler::new(maze);
        let mut traveler = traveler.move_to_next_cell().unwrap();

        traveler.open_east_wall();

        assert_snapshot_matches!("next_cell_open_east_wall", traveler.release().as_string());
    }

    #[test]
    fn move_to_next_cell_wraps() {
        let maze = Maze::new(3, 3);
        let traveler = MazeTraveler::new(maze);
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 1, 0
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 2, 0
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 0, 1

        assert_eq!(0, traveler.current_x);
        assert_eq!(1, traveler.current_y);
    }

    #[test]
    fn move_to_next_cell_returns_none() {
        let maze = Maze::new(3, 3);
        let traveler = MazeTraveler::new(maze);
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 1, 0
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 2, 0
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 0, 1
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 1, 1
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 2, 1
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 0, 2
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 1, 2
        let mut traveler = traveler.move_to_next_cell().unwrap(); // Position: 2, 2
        let traveler = traveler.move_to_next_cell();

        assert!(traveler.is_none());
    }
}
