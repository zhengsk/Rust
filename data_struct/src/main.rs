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


    struct User {
        name: String,
        age: i32,
        email: String,
        active: bool,
    }

    impl User {
        fn hi(&self) -> String {
            let mut hi = String::from("Hi, ");
            let name: &str = &self.name[..];
            hi.push_str(name);
            return hi;
        }
    }

    
    let lily = User {
        name: String::from("Lily"),
        age: 12,
        active: true,
        email: String::from("lily@qq.com"),
    };

    let daer = User {
        name: String::from("daer"),
        age: 18,
        ..lily
    };

    println!("{}", daer.hi());
}
