use euclid::Point2D;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub block_x: usize,
    pub block_y: usize,
}
impl Cell {
    pub fn from_point(point: Point2D<f32, ()>, block_size: usize) -> Self {
        let block_x = (point.x / block_size as f32).floor() as usize;
        let block_y = (point.y / block_size as f32).floor() as usize;
        Self { block_x, block_y }
    }
}

impl Cell {
    pub fn to_point(&self, block_size: usize) -> Point2D<f32, ()> {
        Point2D::new(
            self.block_x as f32 * block_size as f32,
            self.block_y as f32 * block_size as f32,
        )
    }
}

impl Cell {
    pub fn neighbors(
        &self,
        width: usize,
        height: usize,
        buffer: &Vec<Vec<u32>>,
        pixel_size: usize,
    ) -> Vec<Cell> {
        let mut result = Vec::new();
        // let x = self.block_x * pixel_size;
        // let y = self.block_y * pixel_size;
        if self.block_x > 0 && buffer[self.block_y][self.block_x - 1] == 0 {
            result.push(Cell {
                block_x: self.block_x - 1,
                block_y: self.block_y,
            });
        }
        if self.block_x < width / pixel_size - 1 && buffer[self.block_y][self.block_x + 1] == 0 {
            result.push(Cell {
                block_x: self.block_x + 1,
                block_y: self.block_y,
            });
        }
        if self.block_y > 0 && buffer[self.block_y - 1][self.block_x] == 0 {
            result.push(Cell {
                block_x: self.block_x,
                block_y: self.block_y - 1,
            });
        }
        if self.block_y < height / pixel_size - 1 && buffer[self.block_y + 1][self.block_x] == 0 {
            result.push(Cell {
                block_x: self.block_x,
                block_y: self.block_y + 1,
            });
        }
        result
    }

    pub fn heuristic(&self, other: &Cell, pixel_size: usize) -> f64 {
        let dx = (self.block_x as isize - other.block_x as isize).abs() as f64;
        let dy = (self.block_y as isize - other.block_y as isize).abs() as f64;
        let cx1 = (self.block_x * pixel_size + pixel_size / 2) as f64;
        let cy1 = (self.block_y * pixel_size + pixel_size / 2) as f64;
        let cx2 = (other.block_x * pixel_size + pixel_size / 2) as f64;
        let cy2 = (other.block_y * pixel_size + pixel_size / 2) as f64;
        ((dx * dx + dy * dy).sqrt() + (cx1 - cx2).abs() + (cy1 - cy2).abs()) / 2.0
    }
}
