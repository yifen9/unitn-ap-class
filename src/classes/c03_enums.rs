pub enum IpAddrKind {
    V4,
    V6,
}
enum IpAddr {
    V4(i32,i32,i32,i32),
    V6(String),
    V0(),
}


///     https://doc.rust-lang.org/book/ch06-00-enums.html
pub fn enum_usage(){
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let loopback = IpAddr::V6(String::from("::1"));
    let home = IpAddr::V4(127, 0, 0, 1);
}


// https://doc.rust-lang.org/std/option/enum.Option.html
/* enum Option<T> {
    None,
    Some(T),
} */

///     https://doc.rust-lang.org/std/option/enum.Option.html
pub fn option(){
    let x: i8 = 5;
    let y: Option<i8> = None; //Some(5);
    // QUIZ: can i do:
    // let sum = x + y;
    let nopt : Option<i32> = None;
    let opt = Some(10);

    // QUIZ: what will these expressions do?
    // let xxv = nopt.unwrap();
    // let v = opt.unwrap();
    // println!("Some of {}",None);

}

///     https://doc.rust-lang.org/book/ch18-00-patterns.html?highlight=pattern%20ma#patterns-and-matching
pub fn patternmatching(){
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    // QUIZ: is this ok?
    // match home {
    //     IpAddr::V4(a, b, c, d) => println!("Is V4"),
    //     IpAddr::V6(a) => println!("Is V6")
    // };
    match home {
        IpAddr::V4(127, b, c, d) => println!("Is V4 loopback"),
        IpAddr::V4(a, b, c, d) => println!("Is V4"),
        IpAddr::V6(a) => println!("Is V6"),
        _ => println!(" errror")
    };
    let _variable = match loopback {
        IpAddr::V4(127, b, c, d) => Some(loopback),
        _ => None
    };
    // Q : what is the type of `_variable` ?
    let firstfield = match IpAddr::V4(10,20,30,40){
        IpAddr::V4(a,_,_,_) => a,
        _ => 0,
    };
    println!("The first field is: {}", firstfield);

    let nopt : Option<i32> = None;
    let opt : Option<i32> = Some(3);
    let test_eq = match (opt, nopt) {
        (Some(o),Some(n)) => {o == n},
        (Some(_),None) => {false},
        (None,Some(_)) => {false},
        (None, None) => {false},
    };
    println!("Are they the same? {}", test_eq);

    let issome = nopt.is_some();
    let isnone = opt.is_none();
    let content = opt.unwrap();
    let exp = opt.expect("insert error message here");
}


///     https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
pub fn errors() {
    /*    enum Result<T, E> {
        Ok(T),
        Err(E),
    } */
    use std::fs::File;

    // erase file and comment line to see panic
    File::create("hello.txt");

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    use std::io::ErrorKind;
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}



fn qm() -> Option<i32> {
    let x : Result<(),()> = Ok(());
    let r = retop()?;
    let r = match retop(){
        Some(z) => z,
        None => return None,
    };
    return Some(4);
}
pub fn testqm() {
    let r = qm();
    println!("Received {:?}",r);
    let r = retop();
    println!("Received {:?}",r);
}
fn retop() -> Option<String>{
    return Some(String::from("asd"));
}
fn retn() -> Option<i32> {
    return None;
}


use std::fs::File;
use std::io::{Read, Write};
use std::io::prelude::*;

pub fn readfilecontent () -> Result<(),String>{

    // create a new file X -> deal with the Result
    let file = File::open("foo.txt");
    let mut f = match file {
        Ok(x) => x,
        Err(e) => return Err(String::from("could not open")),
    };
    let r = f.write_all(b"adv prog sssss");
    // do not unwrap immediately -> issues when unwrapping later
    // write to it using write_all and a 'b' buffer -> deal with the Result
    // read its content
    let mut s = String::new();
    f.read_to_string(&mut s);
    let scount = calculateS(&s);
    println!("String : {}, count: {}", s, scount);
    // feed to calculateS, with String parameter (not &String) -> Ownership!
    // printout the content and the s's
    return Ok(());
}
// write out calculateS
// use chars iterator
// use eq_ignore_ascii_case
fn calculateS(string : &String) -> i32{
    let mut count =0;
    for x in string.chars(){
        if x.eq_ignore_ascii_case(&'s'){
            count +=1;
        }
    }
    return count;
}
