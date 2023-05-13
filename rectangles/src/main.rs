#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area_method(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "the area of the rectangle is {} square pixels.",
        area_function(&rect1)
    );

    println!("rect1 is {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        ..rect1
    };

    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area_method()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect1.width);
    } else {
        println!("The rectangle has a zero width.");
    }

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));

    let rect5 = Rectangle::square(20);
    println!(
        "The rectangle 4 has width {} and height {}.",
        rect5.width, rect5.height
    );
}

fn area_function(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
