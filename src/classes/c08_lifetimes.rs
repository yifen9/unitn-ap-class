///     https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

// struct User {
//     username: & str,
//     email: & str,
//     sign_in_count: u64,
//     active: bool,
// }
struct User2 {
    username: &'static str,
    email: &'static str,
    sign_in_count: u64,
    active: bool,
}
struct IntUser<'a> {
    username: &'a i32,
    email: &'a i32,
    sign_in_count: u64,
    active: bool,
}

//
pub fn lifetime_test() {
    let user1 = User2 {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    let user2 = User2 {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
//     // QUIZ: does this code respect lifetimes?
//     {
//         let r: i32 = 0;         // ---------+-- 'a
//                                 //          |
//         {                       //          |
//         let mut x : &i32 = &5;  // -+-- 'b  |
//         x = &r;                 //  |       |
//         println!("{}",x);
//         }                       // -+       |
//                                 //          |
//         println!("r: {}", r);   //          |
//     }                           // ---------+
//     // QUIZ: does this code respect lifetimes?
    // {
    //     let r;             // ---------+-- 'a
    //                             //          |
    //     {                       //          |
    //         let x = 5;      // -+-- 'b  |
    //         r = &x;             //  |       |
    //     }                       // -+       |
    //                             //          |
    //     println!("r: {}", r);   //          |
    // }                           // ---------+
}



// fn longest(x:&str, y:&str) -> &str {
//     if x.len() > y.len() { x } else { y}
// } // this function doesn't work

pub fn uselongest() {
    let x = String::from("hi");

    {
        let z;
        let y = String::from("there");
        // z = longest(&x,&y);
        z = correct_longest(&x,&y); //will be &y

        println!("z = {}",z);
    } //drop y, and thereby z
}

fn correct_longest<'a>(x:&'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn another_longest<'a,'b>(x:&'a str, y:&'b str) -> &'b str {
    // QUIZ: does this compile?
    // return if x.len() > y.len() { x } else { y };
    return y;
}
// FAQs 1, 2, 3


// Q: // why is this ok?
fn outliving_longest<'b, 'a: 'b>(x:&'a str, y:&'b str) -> &'b str {
    if x.len() > y.len() { x } else { y}
}

pub fn testintuser(){
    let i = 5;
    let j = 10;

    let user = IntUser {
        email: &i,
        username: &i,
        active: true,
        sign_in_count: 1,
    };
    {
        let k = 20;

        let user2 = IntUser {   //lifetime subsumption
            email: &i,
            username: &k,
            active: true,
            sign_in_count: 1,
        };
    }
}


//      http://blog.pnkfx.org/blog/2019/06/26/breaking-news-non-lexical-lifetimes-arrives-for-everyone
fn nll() {                                           // SCOPE TREE
    let mut names =                         // +- `names` scope start
        ["abe", "beth", "cory", "diane"];           // |
                                                    // |
    let alias = &mut names[0];            // | +- `alias` scope start
                                                    // | |
    *alias = "alex";  // <------------------------ write to `*alias`
                                                    // | |
    println!("{}", names[0]);  // <--------------- read of `names[0]`
                                                    // | |
                                                    // | +- `alias` scope end
                                                    // +- `name` scope end
}
pub fn nll_example() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    let r3 ;
    println!("{} and {}", r1, r2);
    r3 = &mut s;  // r1 and r2 are no longer used after this point
    // QUIZ: can i do this ?
    // println!("{}", r3);

}


// So how do we use references in struct definition?
// we need lifetime annotations in structs
struct Good_User<'a, 'b> {
    username: &'a str,
    email: &'b str,
    sign_in_count: u64,
    active: bool,
}
fn use_lifetimes() {
    let user1 = Good_User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

// this struct defines a lifetime parameter,
// we can only instantiate it with a str that is already valid
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // QUIZ: do i need the lifetime annotation here on &self?
    // fn level(&self) -> i32 {
    //     3
    // }
    // QUIZ: do i need the lifetime annotation here ?
    // fn announce_and_return_part(&self, announcement: &str) -> &str {
    //     println!("Attention please: {}", announcement);
    //     self.part
    // }

    fn announce_and_return_part_2(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


pub fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // Q: do the lifetimes check out?
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let mut second : &str = "asd";
    // {        // in this example, error[E0597]: `another` does not live long enough
    //     let another = String::from("Call me Ishmael. Some years ago...");
    //     second = another.split('a').next().expect("what");
    // }
    let ii = ImportantExcerpt{
        part : second
    };

    let x = i.announce_and_return_part_2("asd");
    println!("{}A", x);
}

