/*
method syntax
*/

#[derive(Debug)]
struct Rect {
    width:  u32,
    height: u32,
}

impl Rect {
    // associated fn square - creates a square as a Rect type.
    fn square(size: u32) -> Rect {
        Rect {
            width:  size,
            height: size,
        }
    }
    // area - calculates the area of a rect.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // width - returns whether the width is nonzero or nah.
    fn width(&self) -> bool {
        self.width > 0
    }

    // can_hold - checks of two types of rects can hold eacothers area.
    pub fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let sq = Rect::square(24);

    println!("\n this is a square: {sq:#?}");

    let rect_one = Rect {
        width:  40,
        height: 80,
    };

    let rect_two = Rect {
        width:  20,
        height: 30,
    };

    println!("the area of our rectangle is {}. ", rect_one.area());
    if rect_one.width() {
        println!("rectangles width is {}", rect_one.width);
    }

    println!("can rect1 hold rect2? {}", rect_one.can_hold(&rect_two));
}
