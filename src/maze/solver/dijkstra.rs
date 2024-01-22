use crate::maze;
use crate::maze::solver::MazeSolverStrategy;

pub struct DijkstraSolverStrategy;

impl MazeSolverStrategy for DijkstraSolverStrategy {
    fn solve(&self, maze_struct: maze::MazeStruct) {
        println!(
            "Solve from dijkstra.rs, maze_struct area is: {}",
            maze_struct.get_area()
        );
    }
}
