// I did this example on my own

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixed_up<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

    fn get_x(&self) -> &T {
        &self.x
    }
}

// Implementacja tylko dla Point<u32, usize>
impl Point<u32, usize> {
    fn distance(&self) -> f32 {
        let x = self.x as f32;
        let y = self.y as f32;

        (x * x + y * y).sqrt()
    }
}

fn main() {
    let point_int = Point { x: 5, y: 10 };
    let point_flt = Point { x: 5.0, y: 10.5 };
    let point_mix = Point { x: 5, y: 10.5 };
}
