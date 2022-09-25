#[cfg(test)]
mod tests {
    use crate::Bresenham;
    use crate::Point;

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
}
