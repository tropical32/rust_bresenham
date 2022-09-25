// https://rosettacode.org/wiki/Bitmap/Bresenham%27s_line_algorithm#Rust
#[derive(Debug, PartialEq, Clone)]
struct Point(isize, isize);

struct Bresenham {
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

        Some(Point(self.current_x, self.current_y))
    }
}

#[test]
fn test_symmetry() {
    let points: Vec<Point> = Bresenham::new(Point(1, 1), Point(3, 7)).collect();
    let points2: Vec<Point> = Bresenham::new(Point(3, 7), Point(1, 1)).collect();

    assert_eq!(points.len(), points2.len());

    assert_eq!(points.contains(&Point(3, 7)), true);

    assert_eq!(points.contains(&Point(3, 6)), true);
    assert_eq!(points2.contains(&Point(3, 6)), true);

    assert_eq!(points.contains(&Point(2, 5)), true);
    assert_eq!(points2.contains(&Point(2, 5)), true);

    assert_eq!(points.contains(&Point(2, 4)), true);
    assert_eq!(points2.contains(&Point(2, 4)), true);

    assert_eq!(points.contains(&Point(2, 3)), true);
    assert_eq!(points2.contains(&Point(2, 3)), true);

    assert_eq!(points.contains(&Point(1, 2)), true);
    assert_eq!(points2.contains(&Point(1, 2)), true);

    assert_eq!(points2.contains(&Point(1, 1)), true);
}
