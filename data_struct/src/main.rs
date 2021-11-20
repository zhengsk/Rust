struct Rectangle {
    width: isize,
    height: isize,
}

impl Rectangle {
    fn area(&self) -> isize{
        return self.width * self.height;
    }
}

fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle {
        width: 32,
        height: 32,
    };

    println!("Rect width is {}, height is {}, area is {}", rect1.width, rect1.height, rect1.area());
}
