use crate::maze;
use crate::maze::builder::MazeBuildStrategy;

pub struct RecursiveBacktrackerStrategy;

impl MazeBuildStrategy for RecursiveBacktrackerStrategy {
    fn build(&self, maze_struct: maze::MazeStruct) {
        println!(
            "build from recursive_backtracker.rs, maze_struct area is: {}",
            maze_struct.get_area()
        );
    }
}
