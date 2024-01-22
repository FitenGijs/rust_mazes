mod maze;
fn main() {
    let maze: maze::MazeStruct = maze::create_empty_maze(100, 100);

    println!(
        "Width: {}, Height: {}, Nr of cells: {}",
        maze.width,
        maze.height,
        maze.cells.len()
    );

    maze::builder::random::build_maze();
    maze::solver::dijkstra::solve_maze();
}
