# rust_bresenham

![bresenham line](bresenham.png "Bresenham line")

## Usage with iterator

    use rust_bresenham::Bresenham;

    fn main() {
        let points_iter = Bresenham::new((1, 1), (3, 7));

        for point in points_iter {
            println!("{:?}", point);
            // (1, 2)
            // (2, 3)
            // (2, 4)
            // (2, 5)
            // (3, 6)
        }
    }


## Get all points

    use rust_bresenham::Bresenham;

    fn main() {
        let points: Vec<_> = Bresenham::new((3, 7), (1, 1)).collect();

        println!("points = {:?}", points);
        // points = [(1, 2), (2, 3), (2, 4), (2, 5), (3, 6)]
    }
