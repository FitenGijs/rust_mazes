use crate::maze;
pub mod dijkstra;

pub trait MazeSolverStrategy {
    fn solve(&self, maze_struct: maze::MazeStruct);
}

pub struct MazeSolver<T: MazeSolverStrategy> {
    pub solve_strategy: T,
}

impl<T: MazeSolverStrategy> MazeSolver<T> {
    pub fn new(solve_strategy: T) -> Self {
        Self { solve_strategy }
    }

    pub fn solve(&self, maze_struct: maze::MazeStruct) {
        self.solve_strategy.solve(maze_struct);
    }
}
