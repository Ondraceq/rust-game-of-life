#[derive(Debug)]
pub struct WindowSize {
    pub cells_in_width: u32,
    pub cells_in_height: u32,
    pub cell_width: u32,
    pub cell_height: u32,
}

impl WindowSize {
    pub fn get_width(&self) -> u32 {
        self.cell_width * self.cells_in_width
    }
    pub fn get_height(&self) -> u32 {
        self.cell_height * self.cells_in_height
    }
}
