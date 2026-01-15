// I did this example on my own
struct Position<T> {
    x: T,
    y: T,
}

// Example of implementing a generic struct for concrate type
impl Position<u8> {
    fn get_y(&self) -> &u8 {
        &self.y
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl Point<u32, usize> {}

fn main() {
    let point_int = Point { x: 5, y: 10 };
    let point_flt = Point { x: 5.0, y: 10.5 };
    let point_mix = Point { x: 5, y: 10.5 };
}
