# Usage
    use rust_bresenham::Bresenham;

    let points = Bresenham::new(Point(1, 1), Point(3, 7));

    for point in points {
      // point.0
      // point.1
    }

    // OR

    let points = Bresenham::new(Point(3, 7), Point(1, 1)).collect();
    println!("points = {:?}", points);
