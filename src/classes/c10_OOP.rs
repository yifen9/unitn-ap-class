/* ======= Objects =========
   ====================== */
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}

/* ===== Encapsulation =====
   ====================== */
// QUIZ: public / private


pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn get_average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


/* ===== Inheritance =======
   ====================== */

//QUIZ: what Rust feature enables dynamic dispatch?

fn wrong_Vecs() {
    let v1 = vec![1u32, 1u32];
    let v2 = vec![1u64, 1u64];
    // let v3 = vec![1u32, 1u64];       // DNC: error[E0308]: mismatched types
}

trait Show {
    fn show(&self) -> String;
}
impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}
impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}
impl Show for String{
    fn show(&self) -> String{
        self.clone()
    }
}
pub fn example_oop1() {
    let answer = 42;
    let maybe_pi = 3.14;
    let s = "what".to_string();

    // QUIZ: does this compile?
    let v: Vec<&dyn Show> = vec![&answer, &maybe_pi, &s];
    for d in v.iter() {
        println!("show {}", d.show());
    }
}


trait Quack {
    fn quack(&self);
}
// Duck is a "class"
struct Duck ();
impl Quack for Duck {   // this looks like Duck implements Quack , in Java
    fn quack(&self) {
        println!("quack!");
    }
}
// second class
struct RandomBird {
    is_a_parrot: bool
}
// this looks like RandomBird implements Quack , in Java
impl Quack for RandomBird {
    fn quack(&self) {
        if ! self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}

pub fn example_animals_oop() {
    let duck1 = Duck();
    let duck2 = RandomBird { is_a_parrot: false };
    let parrot = RandomBird { is_a_parrot: true };

    let ducks: Vec<&dyn Quack> = vec![&duck1, &duck2, &parrot];

    // QUIZ: what does this print ?
    // qqs | qss | qqq
    for d in &ducks {
        d.quack();
    }

    println!("And now with subtleties:");
    // these 2 pairs of functions
    // print the same things, but operate differently
    quack_ref(&duck1);
    quack_ref(&parrot);
    quack_trait(&duck1);
    quack_trait(&parrot);
}
// QUIZ: what is the difference between these two?
fn quack_ref (q: &dyn Quack) {
    q.quack();
}
fn quack_trait<Q> (q: &Q)
    where Q: Quack {
    q.quack();
}
















// recall the trait Show from above, this is a new trait
trait Location {
    fn location(&self) -> String;
}
trait ShowTell: Show + Location {}

#[derive(Debug)]
struct Foo {
    name: String,
    location: String
}
// some methods for allocating this struct
impl Foo {
    fn new(name: &str, location: &str) -> Foo {
        Foo{
            name: name.to_string(),
            location: location.to_string()
        }
    }
}
// let's make Foo implement Show
impl Show for Foo {
    fn show(&self) -> String {
        self.name.clone()
    }
}
// and also Location
impl Location for Foo {
    fn location(&self) -> String {
        self.location.clone()
    }
}
impl ShowTell for Foo {}

// let's create 3 simple functions to tell the types
fn transferShow(x: &dyn Show) -> &dyn Show {
    x
}
fn transferLocation(x: &dyn Location) -> &dyn Location {
    x
}
fn transferShowTell(x: &dyn ShowTell) -> &dyn ShowTell {
    x
}

pub fn example_multiple_traits(){
    let f = Foo::new("n", "a");
    // let's look at the types of these three variables
    let ff1 = transferShow(&f);
    let ff2 = transferLocation(&f);
    let ff3 = transferShowTell(&f);
    println!(" Foo's: {}, Show's: {}, Location's {}, ShowTell's: {}", f.name, ff1.show(), ff2.location(), ff3.show() );
}