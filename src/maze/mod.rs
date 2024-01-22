//MODULE maze
pub mod builder;
pub mod solver;

//Structs
//Holds the maze data
pub struct MazeStruct{
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl MazeStruct {
    pub fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

//Holds the data for each cell
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub is_wall: bool,
}

//Creates an empty maze with the given width and height
pub fn create_empty_maze(width: u32, height: u32) -> MazeStruct {
    let mut cells: Vec<Cell> = Vec::new();
    for x in 0..width {
        for y in 0..height {
            let cell = Cell {
                x,
                y,
                is_wall: false,
            };
            cells.push(cell);
        }
    }

    MazeStruct {
        width,
        height,
        cells,
    }
}




