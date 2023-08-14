use euclid::Point2D;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Cell {
    pub block_x: usize,
    pub block_y: usize,
}

impl Cell {
    pub fn new(block_x: usize, block_y: usize, pixel_size: usize) -> Self {
        Self {
            block_x: block_x / pixel_size,
            block_y: block_y / pixel_size,
        }
    }
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
    pub fn neighbors_old(
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

impl Cell {
    pub fn neighbors(
        &self,
        width: usize,
        height: usize,
        buffer: &Vec<Vec<u32>>,
        pixel_size: usize,
        min_distance: f32,
    ) -> Vec<Cell> {
        let mut result = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let x = self.block_x as i32 + i;
                let y = self.block_y as i32 + j;

                if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                    continue;
                }

                let mut too_close = false;
                for k in 0..pixel_size {
                    for l in 0..pixel_size {
                        let px = x as usize * pixel_size + k;
                        let py = y as usize * pixel_size + l;
                        if py >= buffer.len() || px >= buffer[py].len() || buffer[py][px] != 0 {
                            let dx = (px as f32 - self.to_point(pixel_size).x) / pixel_size as f32;
                            let dy = (py as f32 - self.to_point(pixel_size).y) / pixel_size as f32;
                            let distance = (dx * dx + dy * dy).sqrt();
                            if distance < min_distance {
                                too_close = true;
                                break;
                            }
                        }
                    }
                    if too_close {
                        break;
                    }
                }

                if !too_close {
                    result.push(Cell::new(x as usize, y as usize, pixel_size));
                }
            }
        }

        result
    }
}

impl Cell {
    pub fn distance(&self, other: &Self) -> f32 {
        let dx = (self.block_x as f32 - other.block_x as f32).abs();
        let dy = (self.block_y as f32 - other.block_y as f32).abs();
        (dx * dx + dy * dy).sqrt()
    }

    pub fn is_obstacle(&self, buffer: &[Vec<u32>]) -> bool {
        buffer[self.block_y][self.block_x] == 0
    }
}
