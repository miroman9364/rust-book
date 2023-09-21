#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        dbg!(self.width * self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        dbg!(self.get_long_side() > other.get_long_side() && self.get_short_side() > other.get_short_side())
    }

    fn get_long_side(&self) -> u32 {
        dbg!(self.width.max(self.height))
    }

    fn get_short_side(&self) -> u32 {
        dbg!(self.width.min(self.height))
    }

    fn new(width: u32, height: u32) -> Self {
        dbg!(Self { width, height })
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels, and can hold {}.",
        rect1,
        rect1.area(),
        rect1.can_hold(& Rectangle::new(rect1.height - 1, rect1.width - 1))
    );
}
