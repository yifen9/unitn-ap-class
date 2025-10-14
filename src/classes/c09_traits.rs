fn the_large_one_i8(x: i8, y:i8) -> i8 {if x > y {x} else {y}}
fn the_large_one_i16(x: i16, y:i16) -> i16 {if x > y {x} else {y}}

// Q: why should this code not compile?
// fn the_large_one_gen<T>(x: T, y: T) -> T {if x > y {x} else {y}}

fn the_large_one_gen_correct<T:PartialOrd>(x: T, y: T) -> T {
    if x > y {x} else {y}
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn say_hello() where Self: Sized {
        println!("Hello")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// QUIZ: Will this compile:
// impl Summary for NewsArticle {
//
// }

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn say_hello() {
        println!("Yello!")
    }
}

pub fn traitexample(){
    let t = Tweet{
        username: "Marco".to_string(),
        content: "no way jose".to_string(),
        reply: false,
        retweet: false
    };
    let n = NewsArticle{
        headline: "Wasps!".to_string(),
        location: "Povo".to_string(),
        author: "Patrignani".to_string(),
        content: "there is a wasp in my attic".to_string()
    };
    t.summarize();
    n.summarize();

    // QUIZ: what do these print?
    NewsArticle::say_hello();
    Tweet::say_hello();
}


fn notify_fn(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_bound<T: Summary>(item: &T) {
    println!("More Breaking news! {}", item.summarize());
}

use std::cmp::Ordering;
// let's import 2 commonly used traits first
use std::fmt::{Debug, Display, Formatter};

fn notify_fn2(item1: &(impl Summary + Display), item2: &(impl Summary + Display)) {}

fn notify_bound2<T: Summary + Display>(item1: &T, item2: &T) {}

pub fn example_notify(){
    let t = Tweet{
        username: "Marco".to_string(),
        content: "no way jose".to_string(),
        reply: false,
        retweet: false
    };
    notify_fn(&t);
    notify_bound(&t);
    // QUIZ: why will this not compile?
    notify_fn2(&t, &t);
    notify_bound2(&t, &t);

    let n = NewsArticle{
        headline: "Wasps!".to_string(),
        location: "Povo".to_string(),
        author: "Patrignani".to_string(),
        content: "there is a wasp in my attic".to_string()
    };
    notify_fn2(&t, &n);
    // notify_bound2(&t, &n);
    //
    // DNC: error[E0277]: `Tweet` doesn't implement `std::fmt::Display`
}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("");
        Ok(())
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("");
        Ok(())
    }
}


fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}
fn some_function_where<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug{
    0
}

// fn ret_trait_wrong() -> dyn Summary {
//     let t = Tweet{
//         username: "Marco".to_string(),
//         content: "no way jose".to_string(),
//         reply: false,
//         retweet: false
//     };
//     let n = NewsArticle{
//         headline: "Wasps!".to_string(),
//         location: "Povo".to_string(),
//         author: "Patrignani".to_string(),
//         content: "there is a wasp in my attic".to_string()
//     };
//     if rand::random() < 0.5 {
//         t
//     } else {
//         n
//     }
// }


fn ret_trait() -> Box<dyn Summary> {
    let t = Tweet{
        username: "Marco".to_string(),
        content: "no way jose".to_string(),
        reply: false,
        retweet: false
    };
    let n = NewsArticle{
        headline: "Wasps!".to_string(),
        location: "Povo".to_string(),
        author: "Patrignani".to_string(),
        content: "there is a wasp in my attic".to_string()
    };
    if 0.1 < 0.5 {
        // and now we return stuff allocated in a Box
        Box::new(t)
    } else {
        Box::new(n)
    }
}


struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}


impl Animal for Sheep {
    // uncomment function to see error
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}
// QUIZ: What's the return type?
fn random_animal(random_number: f64) ->
//
                                     Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

pub fn animals_example() {
    let random_number = rand::random();
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

/* ==== Conditional Trait Implementation ======
====================== */
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    fn print(&self) {
        println!("Pair")
    }
}
impl<T: Display + PartialOrd> Pair<T> {

    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


/* ==== Deriving Traits ====
   ====================== */
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);
#[derive(Debug,PartialEq)]
struct Inches(i32);
impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

fn example_derivable() {
    let _one_second = Seconds(1);
    // QUIZ: why do this not compile?
    // println!("One second looks like: {:?}", _one_second);
    // QUIZ: why do this not compile?
    // let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
// impl PartialOrd for Point<T,U>{
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         todo!()
//     }
// }

/* === Self (capital S) ====
   ====================== */

trait T {
    type Item;
    const C: i32;
    fn new() -> Self;

    fn f(&self) -> Self::Item;
}
struct ST;
impl T for ST {
    type Item = i32;
    const C: i32 = 9;
    fn new() -> Self {           // `Self` is the type `ST`.
        ST
    }
    fn f(&self) -> Self::Item {  // `Self::Item` is the type `i32`.
        Self::C                  // `Self::C` is the constant value `9`.
    }
}
pub fn test () {
    let st = ST::new();
    let n = st.f();
}

// what is the difference between a generic struct and one
// that defines its type alias?

//
// the control of who instantiates the type: caller/callee


// pub fn testtt(a : Box<dyn T<Item = i32>>) -> ST{
//     return 3;
// }

/* ===== Super Traits ======
   ====================== */
// Rust doesn't have "inheritance", but you can define a
// trait as being a superset of another trait.
//
//  what is the difference between trait inheritance and class inheritance

//
// traits only extend the behaviour
// classes also extend the shape
// For example:
trait Person {
    fn name(&self) -> &String;
}
/*// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
// so a Student 'is a' Person
*/
trait Student: Person {
    fn university(&self) -> String;
}
trait Programmer {
    fn fav_language(&self) -> String;
    fn git_username(&self) -> String;
}
/*
// CompSciStudent (computer science student) is a subtrait
// of both Programmer and Student. Implementing CompSciStudent
// requires you to impl both supertraits
// and their supertraits
*/
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
// why can we have this "multiple inheritance" ?

// This function uses all the methods available as
// something that implements CompSciStudent
fn comp_sci_student_greeting(student: &impl CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        Programmer::git_username(student)
        // student.git_username()
    )
}

struct Entity{
    nm:String,
}
// QUIZ: how many IMPLs do we need now for
// Entity if we say it implements CompSciStudent?
// comment this impl, retype and see the issues
impl CompSciStudent for Entity{
    fn git_username(&self) -> String {
        String::from("squera")
    }
}
impl Programmer for Entity {
    fn fav_language(&self) -> String {
        String::from("Rust!")
    }
    fn git_username(&self) -> String {
        String::from("aswd")
    }
}
impl Student for Entity {
    fn university(&self) -> String {
        String::from("unitn")
    }
}
impl Person for Entity {
    fn name(&self) -> &String {
        &self.nm
    }
}

// let's now test the previous function on some Entity
pub fn example_supertraits(){
    let e = Entity{
        nm : String::from("marco")
    };
    let f = Entity{
        nm : String::from("pigna")
    };
    println!("{}",comp_sci_student_greeting(&e));
    println!("{}",comp_sci_student_greeting(&f));
}




/* ===== Polymorphism ======
   ========================= */
// To many people, polymorphism is synonymous with inheritance.
// They're wrong. ish.
// But it’s actually a more general concept that refers
// to code that can work with data of multiple types.
//
// Polymorphism comes in 3 ways in programming languages
// (https://en.wikipedia.org/wiki/Polymorphism_(computer_science))
//      parametric polymorphism
//          more similar to what we have here
//          Rust has Bounded parametric polymorphism,
//          because each type parameter can also be given Trait bounds
//      ad hoc polymorphism
//         overloading
//      subtyping polymorphism
//          inheritance

// Rust instead uses generics to abstract over different possible types
// and trait bounds to impose constraints on what those types must provide.
// This is sometimes called *bounded parametric polymorphism*.


trait Descrivibile {
    fn descrivi(&self) -> String;
}

fn stampa1<T: Descrivibile>(item: Box<T>) {
    println!("{}", item.descrivi());
}

fn stampa2(item: Box<impl Descrivibile>) {
    println!("{}", item.descrivi());
}

fn stampa3(item: Box<dyn Descrivibile>) {
    println!("{}", item.descrivi());
}