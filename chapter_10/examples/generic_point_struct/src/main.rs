// I did this example on my own

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let point_int = Point { x: 5, y: 10 };
    let point_flt = Point { x: 5.0, y: 10.5 };
    let point_mix = Point { x: 5, y: 10.5 };
}
