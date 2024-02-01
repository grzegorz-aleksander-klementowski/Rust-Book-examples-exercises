struct Color(usize, usize, usize);
struct Point(usize, usize, usize);
struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}
