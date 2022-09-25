// https://rosettacode.org/wiki/Bitmap/Bresenham%27s_line_algorithm#Rust
mod tests;

pub type Point = (isize, isize);

pub struct Bresenham {
    end: Point,
    current_x: isize,
    current_y: isize,
    dx: isize,
    dy: isize,
    error: isize,
    sx: isize,
    sy: isize,
}

impl Bresenham {
    pub fn new(start: Point, end: Point) -> Self {
        let dx = isize::abs(end.0 - start.0);
        let dy = isize::abs(end.1 - start.1);
        let error = (if dx > dy { dx } else { -dy }) / 2;
        let sx = {
            if start.0 < end.0 {
                1
            } else {
                -1
            }
        };
        let sy = {
            if start.1 < end.1 {
                1
            } else {
                -1
            }
        };

        Self {
            end: end.clone(),
            current_x: start.0,
            current_y: start.1,
            dx,
            dy,
            error,
            sx,
            sy,
        }
    }
}

impl Iterator for Bresenham {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x == self.end.0 && self.current_y == self.end.1 {
            return None;
        }

        let error2 = self.error;

        if error2 > -self.dx {
            self.error -= self.dy;
            self.current_x += self.sx;
        }
        if error2 < self.dy {
            self.error += self.dx;
            self.current_y += self.sy;
        }

        Some((self.current_x, self.current_y))
    }
}
