#[cfg(test)]
mod tests {
    use crate::Bresenham;
    use crate::Point;

    #[test]
    fn test_symmetry() {
        let points: Vec<Point> = Bresenham::new((1, 1), (3, 7)).collect();
        let points2: Vec<Point> = Bresenham::new((3, 7), (1, 1)).collect();

        assert_eq!(points.len(), points2.len());
        assert_eq!(points.len(), 5);

        assert!(points.contains(&(3, 6)));
        assert!(points2.contains(&(3, 6)));

        assert!(points.contains(&(2, 5)));
        assert!(points2.contains(&(2, 5)));

        assert!(points.contains(&(2, 4)));
        assert!(points2.contains(&(2, 4)));

        assert!(points.contains(&(2, 3)));
        assert!(points2.contains(&(2, 3)));

        assert!(points.contains(&(1, 2)));
        assert!(points2.contains(&(1, 2)));
    }
}
