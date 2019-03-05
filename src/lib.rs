use std::fmt;
use rand::Rng as _;

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

const LINE_ENDING: &str = "\n";

#[derive(Debug, Copy, Clone)]
enum Wall {
    Open,
    Closed,
}

pub struct Maze {
    height: u32,
    width: u32,
    walls: Vec<Wall>,
}

struct MazeIterator {
    current_x: u32,
    current_y: u32,
    max_x: u32,
    max_y: u32,
}

/// Calculates and stores the distance from start point to every other cell on the maze
struct MazePath {
    start: MazeCell,
    distances: Vec<Vec<u32>>,
}

/// Stores the available movement options from a given starting point
/// None represents either a wall or maze edge in that direction
#[derive(Debug, PartialEq)]
struct MovementOptions {
    north: Option<MazeCell>,
    east: Option<MazeCell>,
    south: Option<MazeCell>,
    west: Option<MazeCell>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct MazeCell {
    x: u32,
    y: u32,
}

impl Maze {
    /// Create a new maze of all closed walls
    /// Panics if height or width are < 1
    fn new(width: u32, height: u32) -> Self {
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

    pub fn binary_tree(width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();
        Self::binary_tree_with_rand_fn(height, width, || rng.gen_bool(0.5))
    }

    fn binary_tree_with_rand_fn<F>(width: u32, height: u32, mut rand_bool: F) -> Self
        where F: FnMut() -> bool
    {
        let mut maze = Self::new(height, width);
        let maze_iter = MazeIterator::new(&maze);
        for cell in maze_iter {
            if rand_bool() {
                let result = maze.open_north_wall(cell);
                if result.is_err() {
                    // if you can't open the north wall, fall back to opening the east wall
                    // the northeast cell will fail opening both north and east walls, so we ignore this error result
                    let _ = maze.open_east_wall(cell);
                }
            } else {
                let result = maze.open_east_wall(cell);
                if result.is_err() {
                    // if you can't open the east wall, fall back to opening the north wall
                    // the northeast cell will fail opening both north and east walls, so we ignore this error result
                    let _ = maze.open_north_wall(cell);
                }
            }
        }

        maze
    }

    pub fn sidewinder(width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();
        let mut rng2 = rand::thread_rng();
        Self::sidewinder_with_rand_fn(
            height,
            width,
            || rng.gen_bool(0.5),
            || rng2.gen(),
        )
    }

    fn sidewinder_with_rand_fn<F1, F2>(
        width: u32,
        height: u32,
        mut rand_bool: F1,
        mut rand_usize: F2,
    ) -> Self
        where
            F1: FnMut() -> bool,
            F2: FnMut() -> usize
    {
        let mut maze = Self::new(height, width);
        let maze_iter = MazeIterator::new(&maze);
        let mut cells_in_run = vec![];
        for cell in maze_iter {
            cells_in_run.push(cell);
            if rand_bool() {
                // randomly open a passage north from one of the cells in run
                let selected_cell_index = rand_usize() % cells_in_run.len();
                let selected_cell = cells_in_run[selected_cell_index];
                cells_in_run.drain(..); // the run ends once a passage is opened north
                let result = maze.open_north_wall(selected_cell);
                if result.is_err() {
                    // if you can't open the north wall, fall back to opening the east wall
                    let _ = maze.open_east_wall(selected_cell);
                }
            } else {
                let result = maze.open_east_wall(cell);
                if result.is_err() {
                    // if you can't open the east wall, fall back to opening the north wall
                    // the northeast cell will fail opening both north and east walls, so we ignore this error result
                    cells_in_run.drain(..); // the run ends once a passage is opened north
                    let _ = maze.open_north_wall(cell);
                }
            }
        }

        maze
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
            (_x, 0) => None,
            // get south wall index by going one cell down and getting north wall index
            (x, y) => self.north_wall_index_for_cell(x, y - 1),
        }
    }

    #[allow(dead_code)] // keep this method for symmetry although it is currently unused
    fn west_wall_index_for_cell(&self, x: u32, y: u32) -> Option<usize> {
        match (x, y) {
            // walls at left of maze have no west wall
            (0, _y) => None,
            // get west wall index by going one cell left and getting east wall index
            (x, y) => self.east_wall_index_for_cell(x - 1, y),
        }
    }

    /// Returns Ok if it was able to open the wall
    /// Returns Err if north wall for this cell was the edge of the maze
    fn open_north_wall(&mut self, cell: MazeCell) -> Result<(), ()> {
        let index = self.north_wall_index_for_cell(cell.x, cell.y);

        match index {
            Some(index) => {
                self.walls[index] = Wall::Open;
                Ok(())
            },
            None => {
                Err(())
            },
        }
    }

    /// Returns Ok if it was able to open the wall
    /// Returns Err if east wall for this cell was the edge of the maze
    fn open_east_wall(&mut self, cell: MazeCell) -> Result<(), ()> {
        let index = self.east_wall_index_for_cell(cell.x, cell.y);

        match index {
            Some(index) => {
                self.walls[index] = Wall::Open;
                Ok(())
            },
            None => {
                Err(())
            },
        }
    }

    fn get_movement_options_for(&self, cell: MazeCell) -> MovementOptions {
        MovementOptions::new(
            Some(MazeCell::new(0, 1)),
            None,
            None,
            None,
        )
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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

        write!(f, "{}", total)
    }
}

impl MazeIterator {
    fn new(maze: &Maze) -> Self {
        MazeIterator {
            current_x: 0,
            current_y: 0,
            max_x: maze.width - 1,
            max_y: maze.height - 1,
        }
    }
}

impl Iterator for MazeIterator {
    type Item = MazeCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y > self.max_y { return None; }

        if self.current_x < self.max_x {
            // have not hit right-most wall, increment x
            let x = self.current_x;
            let y = self.current_y;

            self.current_x += 1;

            Some(MazeCell::new(x, y))
        } else {
            // on right-most wall
            // wrap x and increment y
            let x = self.current_x;
            let y = self.current_y;

            self.current_x = 0;
            self.current_y += 1;

            Some(MazeCell::new(x, y))
        }
    }
}

impl MovementOptions {
    fn new(
        north: Option<MazeCell>,
        east: Option<MazeCell>,
        south: Option<MazeCell>,
        west: Option<MazeCell>,
    ) -> Self {
        MovementOptions { north, east, south, west }
    }
}

impl MazeCell {
    fn new(x: u32, y: u32) -> Self {
        MazeCell { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_display_snapshot_matches;

    #[test]
    fn new_3x3() {
        let maze = Maze::new(3, 3);

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn wall_indexes_for_cell_00() {
        let maze = Maze::new(3, 3);
        let x = 0;
        let y = 0;

        assert_eq!(Some(0), maze.north_wall_index_for_cell(x, y));
        assert_eq!(Some(6), maze.east_wall_index_for_cell(x, y));
        assert_eq!(None, maze.south_wall_index_for_cell(x, y));
        assert_eq!(None, maze.west_wall_index_for_cell(x, y));
    }

    #[test]
    fn wall_indexes_for_cell_11() {
        let maze = Maze::new(3, 3);
        let x = 1;
        let y = 1;

        assert_eq!(Some(4), maze.north_wall_index_for_cell(x, y));
        assert_eq!(Some(10), maze.east_wall_index_for_cell(x, y));
        assert_eq!(Some(1), maze.south_wall_index_for_cell(x, y));
        assert_eq!(Some(7), maze.west_wall_index_for_cell(x, y));
    }

    #[test]
    fn wall_indexes_for_cell_22() {
        let maze = Maze::new(3, 3);
        let x = 2;
        let y = 2;

        assert_eq!(None, maze.north_wall_index_for_cell(x, y));
        assert_eq!(None, maze.east_wall_index_for_cell(x, y));
        assert_eq!(Some(5), maze.south_wall_index_for_cell(x, y));
        assert_eq!(Some(11), maze.west_wall_index_for_cell(x, y));
    }

    #[test]
    fn open_north_wall() {
        let mut maze = Maze::new(3, 3);
        let mut maze_iter = MazeIterator::new(&maze);
        let cell = maze_iter.next().unwrap();

        maze.open_north_wall(cell).unwrap();

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn open_east_wall() {
        let mut maze = Maze::new(3, 3);
        let mut maze_iter = MazeIterator::new(&maze);
        let cell = maze_iter.next().unwrap();

        maze.open_east_wall(cell).unwrap();

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn next_cell_open_north_wall() {
        let mut maze = Maze::new(3, 3);
        let mut maze_iter = MazeIterator::new(&maze);
        maze_iter.next().unwrap(); // skip the first cell
        let cell = maze_iter.next().unwrap();

        maze.open_north_wall(cell).unwrap();

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn next_cell_open_east_wall() {
        let mut maze = Maze::new(3, 3);
        let mut maze_iter = MazeIterator::new(&maze);
        maze_iter.next().unwrap(); // skip the first cell
        let cell = maze_iter.next().unwrap();

        maze.open_east_wall(cell).unwrap();

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn move_to_next_cell_wraps() {
        let maze = Maze::new(3, 3);
        let mut maze_iter = MazeIterator::new(&maze);
        maze_iter.next().unwrap(); // Position: 0, 0
        maze_iter.next().unwrap(); // Position: 1, 0
        maze_iter.next().unwrap(); // Position: 2, 0
        let cell = maze_iter.next().unwrap(); // Position: 0, 1

        assert_eq!(0, cell.x);
        assert_eq!(1, cell.y);
    }

    #[test]
    fn move_to_next_cell_returns_none() {
        let maze = Maze::new(3, 3);
        let mut maze_iter = MazeIterator::new(&maze);
        maze_iter.next().unwrap(); // Position: 0, 0
        maze_iter.next().unwrap(); // Position: 1, 0
        maze_iter.next().unwrap(); // Position: 2, 0
        maze_iter.next().unwrap(); // Position: 0, 1
        maze_iter.next().unwrap(); // Position: 1, 1
        maze_iter.next().unwrap(); // Position: 2, 1
        maze_iter.next().unwrap(); // Position: 0, 2
        maze_iter.next().unwrap(); // Position: 1, 2
        maze_iter.next().unwrap(); // Position: 2, 2
        let cell = maze_iter.next();

        assert!(cell.is_none());
    }

    #[test]
    fn binary_tree_all_true() {
        let mock_rand_bool = || true;
        let maze = Maze::binary_tree_with_rand_fn(3, 3, mock_rand_bool);

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn binary_tree_all_false() {
        let mock_rand_bool = || false;
        let maze = Maze::binary_tree_with_rand_fn(3, 3, mock_rand_bool);

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn binary_tree_alternating_bool() {
        let mut val = false;
        let mock_rand_bool = || {
            val = !val;
            val
        };
        let maze = Maze::binary_tree_with_rand_fn(3, 3, mock_rand_bool);

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn sidewinder_all_true() {
        let mock_rand_bool = || true;
        let mock_rand_u32 = || 0_usize;
        let maze = Maze::sidewinder_with_rand_fn(3, 3, mock_rand_bool, mock_rand_u32);

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn sidewinder_all_false() {
        let mock_rand_bool = || false;
        let mock_rand_u32 = || 0_usize;
        let maze = Maze::sidewinder_with_rand_fn(3, 3, mock_rand_bool, mock_rand_u32);

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn sidewinder_alternating_bool_0usize() {
        let mut val = false;
        let mock_rand_bool = || {
            val = !val;
            val
        };
        let mock_rand_u32 = || 0_usize;
        let maze = Maze::sidewinder_with_rand_fn(3, 3, mock_rand_bool, mock_rand_u32);

        assert_display_snapshot_matches!(maze);
    }

    fn build_sidewinder_alternating_bool_1usize() -> Maze {
        let mut val = false;
        let mock_rand_bool = || {
            val = !val;
            val
        };
        let mock_rand_u32 = || 1_usize;
        Maze::sidewinder_with_rand_fn(3, 3, mock_rand_bool, mock_rand_u32)
    }

    #[test]
    fn sidewinder_alternating_bool_1usize() {
        let maze = build_sidewinder_alternating_bool_1usize();

        assert_display_snapshot_matches!(maze);
    }

    #[test]
    fn get_movement_options_for_sidewinder_00() {
        let maze = build_sidewinder_alternating_bool_1usize();

        let cell = MazeCell::new(0, 0);
        let expected_movement_options = MovementOptions::new(
            Some(MazeCell::new(0, 1)),
            None,
            None,
            None,
        );
        assert_eq!(expected_movement_options, maze.get_movement_options_for(cell));
    }
}
