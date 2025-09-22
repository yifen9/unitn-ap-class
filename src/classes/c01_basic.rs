/// Material for this module:
///     https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
///     https://doc.rust-lang.org/book/ch03-02-data-types.html
///     https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
///     https://doc.rust-lang.org/book/ch03-05-control-flow.html

use std::io;

/// This function shows Rust variables, assignment and mutability
pub fn var_ass_mut(){
    let y : i32 = 0;
    let x = y +1 ;
    println!("Values of x and y: {} and {}", x, y);

    let x = 'c';
    println!("Value of x: {}", x);
    // println!("Value of x: {}", x+1);
    // DNC: error[E0369]: cannot add `{integer}` to `&str`

    let y = 0;
    // y += 1;
    // DNC: error[E0384]: cannot assign twice to immutable variable `y`
    let z = 1;
    println!("Value of z: {}",z);
    println!("Value of z: {}",z+1);
    // Q: why is the line above ok?

    const _TRUE : i32 = 1;  // _ for warning suppression
}

const _FALSE : i32 = 0;
// QUIZ: can I use const FALSE from `src/main.rs` ?

pub fn vals_types(){
    // integers, usize and floats
    let x : i32 = 10;
    let y : i64 = 20;
    println!("Value of +: {}", x+(y as i32)); // explicit casting only
    let z : f32 = 1.2;
    let u : f64 = 3.45;
    println!("Value of +: {}", (z as f64)+u);

    // bools
    let t : bool = true;
    let f : bool = false;
    if t == f {
        println!("True is false");
    } else {
        println!("True is not false");
    }

    // chars
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Some chars: {}, {}, and {}", c, z, heart_eyed_cat);

    // tuples
    let t = (1,'c',false);
    let tt = t;
    // let last = t.;        // uncomment and see RR's suggestions
    let (a,b,c) = t;
    println!("First element: {} = {} = {}", a,b,c);

    // arrays
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // Q: read this type to me
    // this initialises the array with length 5 to all values being 3
    let _a = [3; 5];
    let a = [3, 3, 3, 3, 3];
    let a1 = a[0];
    let a2 = a[1];
    println!("Array Elements: {} and {}", a1, a2);

    // let a6 = a[7];   // tentative buffer overflow
    // DNC: error: this operation will panic at runtime
    print!("Input element index to lookup:");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(mut i) => {
            println!("Integer input: {}", i);
            // if i>4 { i = 4;  } // comment and input 6
            let _element = a[(i as usize)];
            println!("This will not print without the if");
        }
        ,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    let mut aa = [(1,2),(1,4)];
    println!("Array1 {:?}",aa);
    aa = [aa[0],(4,5)];
    println!("Array2 {:?}",aa);
    aa[1].0 = 3;
    println!("Array3 {:?}",aa);
    // QUIZ: cosa viene stampato qui?
    // [(1, 2), (1, 4)] || [(1, 2), (4, 5)] || [(1, 2), (3, 5)] || [(3, 2), (4, 5)] || [(1, 3), (4, 5)]
}

pub fn expressions(){
    // Rust has if-then and if-then-else conditionals
    // Rust has different forms of iteration
    //  - loops
    let mut c = 0;
    loop {
        println!("This will never stop");
        c += 1;
        if c == 4 {
            break; // breaking is the only way to escape an unguarded loop
        }
    }
    //  - while loops
    while c != 0 {
        println!("Cycle with while {}!", c);
        c -= 1;
    }

    for n in 1..51 {   // or 1..=50
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // array iteration
    let mut a = [10, 20, 30, 40, 50];
    for element in a.iter() {   // iter, iter_mut, into_iter
        println!("Iteration loop: the value is: {}", element);
    }
}

#[cfg(test)]
pub mod testing {
    use super::testfuns::{crapadd, okadd};

    // all functions marked as #[test] can be run with project testing
    #[test]
    fn test_crapadd() {
        println!("Testing Crap Add");
        assert_eq!(crapadd(1,3),2);

    }
    #[test]
    fn test_okadd(){
        println!("Testing OK Add");
        assert_eq!(okadd(1, 5), 6);
    }
}
pub mod testfuns{
    // statements VS expressions
    pub fn crapadd(x: i32,_y: i32) -> i32 {
        return x+x;
    }
    pub fn okadd(x: i32, y:i32) -> i32 {
        x+y
    }
}

