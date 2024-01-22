use crate::maze;

pub mod recursive_backtracker;

pub trait MazeBuildStrategy {
    fn build(&self, maze_struct: maze::MazeStruct);
}

pub struct MazeBuilder<T: MazeBuildStrategy> {
    pub build_strategy: T,
}

impl<T: MazeBuildStrategy> MazeBuilder<T> {
    pub fn new(build_strategy: T) -> Self {
        Self { build_strategy }
    }

    pub fn build(&self, maze_struct: maze::MazeStruct) {
        self.build_strategy.build(maze_struct);
    }
}
