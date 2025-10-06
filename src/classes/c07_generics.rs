// this is a struct with generic fields
struct Point<T, U> {
    x: T,
    y: U,
}
//Q? what is T?
// No type inspection! -> no instanceof!
// impl Point<T,U>{
//     // pub fn add(e : T) -> T {
//     //     if T == i32 {
//     //         ...
//     //     }
//     //     if T == String{
//     //         ...
//     //     }
//     // }
// }
impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }

}


// this instead is an impl that only exists for points i32, i32.
impl Point<i32,i32>{
    fn xx(&self) -> i32 {
        0
    }
}
impl Point<f32,f32>{
    fn xx(&self) -> i32 {
        1
    }
}
impl<T> Point<i32,T>{
    fn xxx(&self) -> i32 {
        0
    }
}

pub fn struct_generic(){
    let both_integer : Point<i32,i32> = Point { x: 5, y: 10 };
    let both_float : Point<f32,f32> = Point { x: 1.0, y: 4.0 };
    let integer_and_float : Point<i32,f32> = Point { x: 5, y: 4.0 };
    let val1:i32 = *both_integer.x();
    let val2:i32 = integer_and_float.xxx();

    // QUIZ: can i call this:
    // both_integer.xxx();
    // both_float.xx();

    println!("val1 and val2: {} and {}", val1, val2);
}

pub fn explicit_type(){
    let x = Some(true);
    // let x = None;
    println!("{:?}",x);
    let b = String::new();
    let trimmed = b.trim();
    // turbofish!
    match trimmed.parse::<u32>() {
        _ => ()
    }
}

/*
// This function
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}

// Gets compiled by the rust compiler into
enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}
fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
 */
