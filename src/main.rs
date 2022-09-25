#[derive(Debug, PartialEq)]
struct Point(isize, isize);

fn get_coordinates(point1: &Point, point2: &Point) -> Vec<Point> {
    let mut coordinates: Vec<Point> = vec![];
    let dx = isize::abs(point2.0 - point1.0);
    let dy = isize::abs(point2.1 - point1.1);
    let sx = {
        if point1.0 < point2.0 {
            1
        } else {
            -1
        }
    };
    let sy = {
        if point1.1 < point2.1 {
            1
        } else {
            -1
        }
    };

    let mut error = (if dx > dy { dx } else { -dy }) / 2;
    let mut current_x = point1.0;
    let mut current_y = point1.1;

    loop {
        coordinates.push(Point(current_x, current_y));

        if current_x == point2.0 && current_y == point2.1 {
            break;
        }

        let error2 = error;

        if error2 > -dx {
            error -= dy;
            current_x += sx;
        }
        if error2 < dy {
            error += dx;
            current_y += sy;
        }
    }

    coordinates
}

fn main() {
    let mut points: Vec<Point> = Vec::new();
    let mut points2 = Vec::new();
    points.append(&mut get_coordinates(&Point(1, 1), &Point(3, 7)));
    points2.append(&mut get_coordinates(&Point(3, 7), &Point(1, 1)));
    println!("points = {:?}", points);
    println!("points2 = {:?}", points2);
}

#[test]
fn test_symmetry() {
    let mut points: Vec<Point> = Vec::new();
    let mut points2 = Vec::new();
    points.append(&mut get_coordinates(&Point(1, 1), &Point(3, 7)));
    points2.append(&mut get_coordinates(&Point(3, 7), &Point(1, 1)));

    assert_eq!(points.len(), points2.len());

    assert_eq!(points.contains(&Point(3, 7)), true);
    assert_eq!(points2.contains(&Point(3, 7)), true);

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

    assert_eq!(points.contains(&Point(1, 1)), true);
    assert_eq!(points2.contains(&Point(1, 1)), true);
}
