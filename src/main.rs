use crate::maze::builder::MazeBuilder;
use crate::maze::solver::MazeSolver;
use crate::maze::builder::recursive_backtracker::RecursiveBacktrackerStrategy;
use crate::maze::solver::dijkstra::DijkstraSolverStrategy;

mod maze;
fn main() {
    let maze1: maze::MazeStruct = maze::create_empty_maze(100, 100);
    let maze2: maze::MazeStruct = maze::create_empty_maze(100, 100);

    println!(
        "Width: {}, Height: {}, Nr of cells: {}",
        maze1.width,
        maze1.height,
        maze1.cells.len()
    );

    let recursive_backtracker: MazeBuilder<RecursiveBacktrackerStrategy> = MazeBuilder::new(RecursiveBacktrackerStrategy);
    let dijkstra_solver: MazeSolver<DijkstraSolverStrategy> = MazeSolver::new(DijkstraSolverStrategy);
    recursive_backtracker.build(maze1);
    dijkstra_solver.solve(maze2);
}
