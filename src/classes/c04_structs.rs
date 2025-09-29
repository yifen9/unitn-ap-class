//      https://doc.rust-lang.org/book/ch05-00-structs.html
struct User {
    pub(crate) username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn struct_usage(){
    let _user0 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }; // Q: can i uncomment the line below ?
    // _user0.email = String::new();
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::new();
    user1.username = String::new();

    // QUIZ: if i go into a different file `c04_structshelper`,
    // can i write
    // let x = user1.email;

    let email = String::from("someone@example.com");
    let username = String::from("someusername123");

    let _user2 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };

    let user3 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let _user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user3
    };
    let _user5 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
}

pub fn retu() -> User{
    let _user5 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: false,
        sign_in_count: 10,
    };
    return _user5;
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}   // Usage:  println!("{:?}",r);

use std::fmt;

///     https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.side,self.side)
    }
}

pub fn struct_printing() {
    let rect = Rectangle{ width: 10, height: 20 };
    let squa = Square{side: 10};
    // println!("Printing a rectangle {}", rect);
    println!("Printing a rectangle {:?}", rect); // use :?
    println!("Printing a square {}", squa);
}


pub struct Square {
    side: u32
}
pub struct Rhombus {
    pub side: u32,
    acute_angle: i32,
}
pub fn new_rhombus() -> Rhombus{
    return Rhombus{ side: 0, acute_angle: 0 };
}
pub fn _new_square() -> Square{
    return Square{ side: 0 };
}

// GOTO structshelper file



impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn perimeter(& self) -> u32 {
        return self.height * 2 + self.width * 2;
    }
    fn double(&mut self) {
        self.width = self.width * 2;
        self.height = self.height * 2;
    }
    fn take_ownership(self) {
    }
    // QUIZ: are these methods or functions:
    // 1 pub fn test1(&self) ...
    // 2 fn test2(&self, arg: int) ...
    // 3 fn test3(arg : int) ...
}


impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    pub fn new() -> Rectangle {
        Rectangle{ width: 10, height: 20 }
    }
    // pub fn new( width : u32, height : u32) -> Rectangle {
    //     Rectangle{ width, height }
    // }
    pub fn new_with_params( width : u32, height : u32) -> Rectangle {
        Rectangle{ width, height }
    }
}

pub fn struct_impl(){
    let mut r = Rectangle::new();
    let a = r.area();
    r.double();
    let p = Rectangle::perimeter(&r);
    println!("The Rectangle is {:?}",r);
    println!("Area: {} and Perimeter: {}", a, p);
}


//

struct Test {
    pub f: i32,
    pub s: Vec<i32>,
}
pub fn ownstructs() {
    let mut example = Test {
        f: 32,
        s: vec![1],
    };
    let new_f = example.f;
    let new_s = example.s;
    println!("First {}", new_f);
    println!("Second {:?}",new_s); // who owns the vector of 1s?
    //Q: can i uncomment below?
    // println!("vec {:?}",example.s);
    // println!("Second {:?}",new_s);
}
