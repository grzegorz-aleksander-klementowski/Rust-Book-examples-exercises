#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle { 

        fn area(&self) -> usize {
    &self.width * self.height
        }

        fn width(&self) -> bool { // Methods like this are called getters
            self.width > 0
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn square(size: usize) -> Self { // associated functions Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called new, but new isn’t a special name and isn’t built into the language. 
            Self {
                width: size,
                height: size,
            }
        }

    }


fn main() {

    let scale: usize = 1;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);
    
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));



    dbg!(&rect1);
}


