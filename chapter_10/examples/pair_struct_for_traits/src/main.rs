use rand::RngExt;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T>
where
    rand::distr::StandardUniform: rand::distr::Distribution<T>,
    T: std::fmt::Display,
{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn new_random() -> Self {
        let mut rng = rand::rng();
        let x: T = rng.random();
        let y: T = rng.random();
        Self::new(x, y)
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// I extended the program to generate randomly two points that create the line.
// The program aim to inform a user, which point is more far away from point zero (0x or 0y).
fn main() {
    let point_a: Pair<u32> = Pair::new_random();
    let point_b: Pair<u32> = Pair::new_random();

    println!("Closet to point to the zero point of Point A: ");
    point_a.cmp_display();
    println!("Closet to point to the zero point of Point B: ");
    point_b.cmp_display();
}
