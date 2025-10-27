/* ========== Box ==========
   ========================= */
pub fn example_box(){
    let b = Box::new(5);
    println!("b's value = {}", *b);
    println!("b's address = {}", b);
    println!("b's real address = {:p}", b);
    println!("b's another address = {:p}", &b);
}

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn example_box_long() {
    let point: Point = origin();    // Stack allocated
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    }); // Heap allocated

    let boxed_point: Box<Point> = Box::new(origin());

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes on the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack", mem::size_of_val(&rectangle));

    // QUIZ:
    // what's going to be printed next?
    // 8 | 16 | 32
    println!("Boxed point occupies {} bytes on the stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack", mem::size_of_val(&box_in_a_box));

    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack", mem::size_of_val(&unboxed_point));
}
// Recursive types
//QUIZ:
// which of the following is a recursive type?
// Array | List | Stream | Pair

// uncomment, comment
// enum WishList{
//     WCons(i32, WishList),
//     WNil,
// }
// DNC: error[E0072]: recursive type `List` has infinite size

enum List {
    Cons(i32, Box<List>),
    Nil,
}
use self::List::{Cons,Nil};
pub fn recursivetypes(){
    // let list : WishList = WCons(1, WCons(2, WCons(3, WNil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}


/* ========= Deref =========
   ========================= */
use std::ops::Deref;

struct MyBox<T>{
    el : T,
    idx : i32
}
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox{el:x,idx:0}
    }
}

pub fn example_smart1() {
    // first, we test on the canonical Box
    let x = 5;
    let y = Box::new(x);

    println!("I expect 5: {}", x);
    println!("I expect 5: {}", *y);
    // Behind the scenes what is actually run is this.
    println!("I expect 5: {}", *(y.deref()));  // *(y.deref())

    let x = 5;
    let y = MyBox::new(x);

    // QUIZ: what does the second println print?
    println!("I expect 5: {}", x);
    // comment the IMPL below
    println!("I expect! 5: {}", *y);
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    // type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.el
        // &self.idx
    }
}


/* ========== Drop =========
   ========================= */
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
pub fn example_drop() {
    let mut c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    {
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        c = d;
    }
    let p1 = &c;
    let p2 = &c;
    let mp1 = &mut c;

    println!("End of function");
}

/* ========== Rc ===========
   ========================= */
pub fn example_rc(){
    // uncomment these 3 lines
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(RcCons(5,
                           Rc::new(RcCons(10,
                                          Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // QUIZ: what is the sequence of printed numbers?
    // 1, 2, 3, 4 | 1, 2, 3, 2 | 1, 2, 3 ,3

    let t = Box::new(10);
    let tt = Rc::new(t);
    let ttt = (tt.clone());
    let tttt = (tt.clone());
    let y = Rc::new(tt);
}

use std::rc::Rc;
use self::RcList::{RcCons,RcNil};

enum RcList {
    RcCons(i32, Rc<RcList>),
    // Cons(i32, Box<List>)
    RcNil,
}


/* ==== Implicit Deref =====
   ========================= */


fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn implitictderef() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //     hello(&(*m)[..]);
}

/* ========== Arc ==========
   ========================= */
pub fn arc() {
    use std::sync::Arc;
    use std::thread;
    use std::thread::sleep;

    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        // QUIZ: can i replace Arc with Rc?

        let tjh = thread::spawn(move || {
            println!("{:?}", apple);
            println!("count after creating apple in a thread: {}", Arc::strong_count(&apple));
        });
        // tjh.join();
    }

}

struct NaiveRc<T> {
    reference_count: usize,
    inner_value: T,
}

impl<T: Copy> Clone for NaiveRc<T> {
    fn clone(&self) -> Self {
        // QUIZ: why does this code not compile?
        // mutability | lifetime | ownership |
        // self.reference_count += 1;


        return NaiveRc{
            reference_count : self.reference_count,
            inner_value : self.inner_value.clone()
        }
    }
}
impl<T: Copy> NaiveRc<T> {
    //We could implement a special, differently-named cloning function that takes &mut self,
    // but that is awful for usability
    // (because it defies the convention of simply checking if a type implements Clone),
    // and forces the user of our API to always declare mutable instances of that type
    fn clone_mut(&mut self) -> Self{
        self.reference_count += 1;
        return NaiveRc{
            reference_count : self.reference_count,
            inner_value : self.inner_value.clone(),
        }
    }
}


/* ======== RefCell ========
   ========================= */
//Interior mutability is a design pattern in Rust
// that allows you to mutate data even when there
// are immutable references to that data;
// QUIZ: what rust principles dictate this?




//
//
// normally, this action is disallowed by the borrowing rules.
//
// To mutate data, the pattern uses **unsafe** code inside a data structure
// to bend Rust’s usual rules that govern mutation and borrowing.
//      NOTE: while we're going to use these library types that
//          rely on unsafe Rust, we are not ever going to write unsafe
//          Rust ourselves. For a good reason.
//
// We will explore this concept using the `RefCell<T>` type,
// which follows the interior mutability pattern.
// Unlike `Rc<T>`, the `RefCell<T>` type represents
// single ownership over the data it holds.
// What makes `RefCell<T>` different from a type like `Box<T>`?
//
// recall the borrowing rules we learned in previous lectures:
//
//  - At any given time, you can have either
// one mutable reference or any number of immutable references.
//  - References must always be valid.
//
// When using `Box<T>`, the borrowing rules’ invariants
// are enforced at compile time.
// With `RefCell<T>`, these invariants are enforced at
//      runtime
// With Box references, if you break these rules, you’ll get a
//      compiler error
// With `RefCell<T>`, if you break these rules, your program will
//      panic and exit.
//
// Checking borrowing rules at compile time has the advantage of
// catching errors sooner in the development process,
// and there is no impact on runtime performance because all the analysis is completed beforehand.
// For those reasons, checking the borrowing rules at compile time
// is the best choice in the majority of cases.
//
// However, there are other scenarios where one might want
// to take advantage of the additional flexibility afforded by checking
// the borrowing rules at runtime. Because some analyses are impossible,
// if the Rust compiler can’t be sure the code complies with the ownership rules,
// it might reject a correct program; in this way, it is conservative.
// The `RefCell<T>` type is useful when you’re sure your code follows
// the borrowing rules but the compiler is unable to understand and guarantee that.

//When to use each type?
//  - `Rc<T>` enables multiple owners of the same data; `
//          Box<T>` and `RefCell<T>` have single owners.
//  - `Box<T>` allows immutable or mutable borrows checked at compile time;
//          `Rc<T>` allows only immutable borrows checked at compile time;
//          `RefCell<T>` allows immutable or mutable borrows checked at runtime.
//  - Because `RefCell<T>` allows mutable borrows checked at runtime,
//          you can mutate the value inside the
//          `RefCell<T>` even when the `RefCell<T>` is immutable.

use std::cell::RefCell;

// the refcell API provides borrow_mut and borrow methods
// to get something of type RefMut or Ref out
// (these implement Deref and Drop)
pub fn refcell_usage() {
    let refcell = RefCell::new(10);
    println!("refcell {:?}", refcell, );
    let mut mptr = refcell.borrow_mut();
    println!("refcell {:?} + content {}", refcell, *mptr);
    *mptr = 11;
    println!("refcell {:?} + content {}", refcell, *mptr);
    // uncomment for panic.
    mem::drop(mptr);
    let ptr1 = refcell.borrow();
    let ptr2 = refcell.borrow();
    let ptr3 = refcell.borrow();
    println!("refcell {:?} + content {}",refcell,*ptr1 );

    // QUIZ: how can i call borrow without panicking?


    //
    // wrap the borrow_mut in a scope to force the drop and this can be uncommented
    // or use
    // mem::drop(mptr);
}

// start here

pub fn refcell_usage_2(){
    let refcell = RefCell::new(10);
    println!("refcell {:?}",refcell,);
    let mut ptr1 = refcell.borrow();
    let mut ptr2 = refcell.borrow();
    let mut ptr3 = refcell.borrow();
    println!("refcell {:?} + content {}",refcell,*ptr1 );
    println!("refcell {:?} + content {}",refcell,*ptr2 );
    // can we use these read-only pointers though for writing?
    // uncomment to find out
    // *ptr3 = 11;
    // println!("refcell {:?} + content {}",refcell,*ptr3 );
}

/* == Interior Mutability ==
   ========================= */
// Recall: Interior mutability is a design pattern in Rust that allows you
// to mutate data even when there are immutable references to that data

//In this example, we’ll create a library that tracks a value against
// a maximum value and sends messages based on how close to the maximum value the current value is.
// This library could be used to keep track of a user’s quota
// for the number of API calls they’re allowed to make, for example.
// Our library will only provide the functionality of tracking
// how close to the maximum a value is and what the messages should be at what times.
// Applications that use our library will be expected to provide
// the mechanism for sending the messages:
// the application could put a message in the application,
// send an email, send a text message, or something else.
// The library doesn’t need to know that detail.
// It only uses a trait we'll provide called Messenger.
//

// The Messenger trait is the interface our mock object needs to implement
// so that the mock can be used in the same way a real object is.
pub trait Messenger {
    fn send(&self, msg: &str);
}
// we need to specify both the lifetime and the type of the messenger field
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>  where T: Messenger, {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
// The other important part is that we want to test the behavior
// of the set_value method on the LimitTracker.
// We can change what we pass in for the value parameter,
// but set_value doesn’t return anything for us to make assertions on.
// We want to be able to say that if we create a LimitTracker
// with something that implements the Messenger trait and a particular value for max,
// when we pass different numbers for value,
// the messenger is told to send the appropriate messages.
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub mod tests {
    // let's use all that is defined outside this module tests
    use super::*;

    // We need a mock object that, instead of sending an email or text message when we call send,
    // will only keep track of the messages it’s told to send.
    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        // adds the message to the mock message vec
        fn send(&self, message: &str) {
            // QUIZ
            // What's the problem with this code?
            // Why won't the borrow checker allow this.
            // self.sent_messages
            //     .push(String::from(message));

            //
            // DNC: error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference

            // now this may seem counterintuitive: just change the Trait signature!
            // alas, sometimes we cannot change trait signatures as we want,
            // and sometime we do not want to
            // here, we are just testing some functionality, we do not want to change the API
        }
    }

    // added but not used
    pub fn it_sends_an_over_75_percent_warning_message() {
        // We can create a new instance of the mock object,
        let mock_messenger = MockMessenger::new();
        // create a LimitTracker that uses the mock object,
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        // call the set_value method on LimitTracker,
        limit_tracker.set_value(80);
        // and then check that the mock object has the messages we expect.
        // uncomment, and it'll crash
        assert_eq!(mock_messenger.sent_messages.len(), 1);

        // This example shows an attempt to do this, but the borrow checker won't allow it.
    }
}
// We also can’t take the suggestion from the error text to use &mut self instead,
// because then the signature of send wouldn’t match the signature in the Messenger trait definition.
//
// However, we can fix this example by using interior mutability.
// We will store the sent_messages within a `RefCell<T>`,
// and then the send method will be able to modify hte sent_messages
//  to store the messages we've seen. Here is an example implementation.

pub mod workingtests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // compare to the previous definition!
        // The sent_messages field is now of type `RefCell<Vec<String>>`
        // instead of `Vec<String>`.
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        //In the new function, we create a new `RefCell<Vec<String>>` instance around the empty vector.
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // compare to the previous def!
        // For the implementation of the send method,
        // the first parameter is still an immutable borrow of self,
        // which matches the trait definition.
        // We call borrow_mut on the `RefCell<Vec<String>>` in self.sent_messages
        // to get a mutable reference to the value inside the `RefCell<Vec<String>>`,
        // which is the vector.
        // Then we can call push on the mutable reference to the vector
        // to keep track of the messages sent during the test.
        fn send(&self, message: &str) {
            self.sent_messages
                .borrow_mut()
                .push(String::from(message));
        }
    }

    pub fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        //The last change we have to make is in the assertion:
        // to see how many items are in the inner vector,
        // we call borrow on the `RefCell<Vec<String>>` to get an immutable reference to the vector.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    // When creating immutable and mutable references,
    // we use the & and &mut syntax, respectively.
    // With `RefCell<T>`, we use the borrow and borrow_mut methods,
    // which are part of the safe API that belongs to `RefCell<T>`.
    // The borrow method returns the smart pointer type `Ref<T>`,
    // and borrow_mut returns the smart pointer type `RefMut<T>`.
    // Both types implement Deref, so we can treat them like regular references.
    //
    // The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers
    // are currently active.
    // Every time we call borrow, the `RefCell<T>` increases its count
    // of how many immutable borrows are active.
    // When a `Ref<T>` value goes out of scope,
    // the count of immutable borrows goes down by one.
    // Just like the compile-time borrowing rules,
    // `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.
    //
    // Now if we violate the borrowing rules,
    // we'll get an error at compile time rather than at runtime. Here is an example.

    // QUIZ: what trait implementations are needed to express this behaviour?

    // uncomment this and comment the other impl Messenger for MockMessenger above
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         let mut one_borrow = self.sent_messages.borrow_mut();
    //         let mut two_borrow = self.sent_messages.borrow_mut();
    //
    //         one_borrow.push(String::from(message));
    //         two_borrow.push(String::from(message));
    //     }
    // }

    // We can see that with refcell this example will compile, but when we run it the program will panic.
    //
    // Notice that the code panicked with the message already borrowed:
    // BorrowMutError. This is how `RefCell<T>` handles violations of the borrowing rules at runtime.
}

/* ====== Rc + RefCell =====
   ========================= */
// It is common to use `RefCell<T>` together with `Rc<T>`.
// Recall that `Rc<T>` lets you have multiple owners of some data,
//      but it only gives immutable access to that data.
// If you have an `Rc<T>` that holds a `RefCell<T>`,
// you can get a value that can have multiple owners and that you can mutate!
// Because `Rc<T>` holds only immutable values,
// we can’t change any of the values in the list once we’ve created them.
// Now we will add in `RefCell<T>` to gain the ability to change the values.

pub mod rc_plus_refcell {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use std::cell::RefCell;
    use std::rc::Rc;
    use self::List::{Cons,Nil};

    pub fn examplepcrefcell() {
        // Here we create a value that is an instance of `Rc<RefCell<i32>>`
        // and store it in a variable named value so we can access it directly later.
        let value = Rc::new(RefCell::new(5));
        //Then we create a List in a with a Cons variant that holds value.
        // We need to clone value so both a and value have ownership of the inner 5 value
        // rather than transferring ownership from value to a or having a borrow from value.
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        // We wrap the list a in an `Rc<T>` so when we create lists b and c,
        // they can both refer to a.
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        // Graph:
        /*
            { Nil } <----------------|
                                     |
            {  5  } <------------|   |
                                 |   |
  |---------------> |-> a ---> [ | , | ]
  |                 |
  |   b ----> [ | , |]
  |             |----------------------------> { 3 }
  |
  |-----------------|
                |---+----------> { 4 }
      c ----> [ | , |]

         */
        // After we’ve created the lists in a, b, and c, we add 10 to the value in value.
        // We do this by calling borrow_mut on value,
        // QUIZ: what Rust feature is used here under the hood?
        let x = *value.borrow_mut() += 10;
        //
        //

        //
        // which uses implicit dereferencing
        // to dereference the `Rc<T>` to the inner `RefCell<T>` value.
        // The borrow_mut method returns a `RefMut<T>` smart pointer,
        // and we use the dereference operator on it to change the inner value.
        // When we print a, b, and c, we can see that they all have the modified value of 15 rather than 5.
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);

    }
    // Here we have an outwardly immutable List value.
    // But we can use the methods on `RefCell<T>` that provide access
    // to its interior mutability so we can modify our data when we need to.
    // The runtime checks of the borrowing rules protect us from data races,
    // and it’s sometimes worth trading a bit of speed for this flexibility in our data structures.
}

// Smart pointers are Data Pointers
// think about the C pointer and RC<RefCell< .. >>
//  and their capabilities
// certain language features are there to limit the usage of pointers in Rust
//  and do not limit this in C



/* === Reference cycles ====
   ========================= */
//Rust’s memory safety guarantees make it difficult, but not impossible,
// to accidentally create memory that is never cleaned up (known as a memory leak).
// Notice that memory leakage is not a memory safety vulnerability!
//
// Preventing memory leaks entirely is not one of Rust’s guarantees
// in the same way that disallowing data races at compile time is.
// We can see that Rust allows memory leaks by using `Rc<T>` and `RefCell<T>`:
// it’s possible to create references where items refer to each other in a cycle.
// This creates memory leaks because the reference count of each item
// in the cycle will never reach 0, and the values will never be dropped.
pub mod overflow {
    use std::cell::RefCell;
    use std::rc::Rc;
    use self::List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
        fn head(&self) -> Option<&i32> {
            match self {
                Nil => None,
                Cons( e, _) => Some(e)
            }
        }
    }

    pub fn exampleoverflow() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        // Uncomment the next line to see that we have a cycle;
        // QUIZ: What will happen?
        println!("a next item = {:?}", a.head());

        //
        // and here?
        println!("a next item = {:?}", a.tail());
    }
}
// We will overflow the stack.
// The reference count of the `Rc<List>` instances in both a and b are 2
// after we change the list in a to point to b.
// At the end of main, Rust drops the variable b,
// which decreases the reference count of the `Rc<List>` instance from 2 to 1.
// The memory that `Rc<List>` has on the heap won’t be dropped at this point,
// because its reference count is 1, not 0.
// Then Rust drops a, which decreases the reference count of the a `Rc<List>` instance from 2 to 1 as well.
// This can’t be dropped either, because the other `Rc<List>` instance still refers to it.


/* ======== Graphs =========
   ========================= */
// For example, when connecting nodes in a graph.
// You may think to wrap the nodes in Rc or Arc and call it a day.
// That a perfectly reasonable line of though, and it would work
//      if you never, ever needed to mutate nodes.
// Once you try building the graph by incrementally adding and connecting nodes,
// the compiler will give you grief.
// You could call get_mut to receive an Option<&mut T>,
// but that would work only once: get_mut only returns a mutable reference
// as if there is only one “strong” reference to the value... Foiled again!
//
// Fortunately, you can use interior mutability here:
//  use Rc<Cell<T>> or Rc<RefCell<T>>.
// That way you can clone the reference-counted wrapper as much as you want
// and still modify the innermost value wrapped by Cell or RefCell.


// A graph can be represented in several ways. For the sake of illustrating how
// interior mutability works in practice, let's go with the simplest
// representation: a list of nodes.
struct Graph<T> {
    nodes: Vec<Node<T>>,
}
// Each node has an inner value and a list of adjacent nodes it is connected to
// (through a directed edge).
struct Node<T>(NodeRef<T>);
// The private representation of a node.
struct _Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}
// That list of adjacent nodes cannot be the exclusive owner of those nodes, or
// else each node would have at most one edge to another node and the graph
// couldn't also own these nodes.
// We need to wrap Node with a reference-counted box, such as Rc or Arc. We'll
// go with Rc, because this is a toy example.
type NodeRef<T> = Rc<RefCell<_Node<T>>>;
// However, Rc<T> and Arc<T> enforce memory safety by only giving out shared
// (i.e., immutable) references to the wrapped object, and we need mutability to
// be able to connect nodes together.
// The solution for this problem is wrapping Node in either Cell or RefCell, to
// restore mutability. We're going to use RefCell because Node<T> doesn't
// implement Copy (we don't want to have independent copies of nodes!).

impl<T> Node<T> {
    // Creates a new node with no edges.
    fn new(inner: T) -> Node<T> {
        let node = _Node {
            inner_value: inner,
            adjacent: vec![]
        };
        Node(Rc::new(RefCell::new(node)))
    }

    // Adds a directed edge from this node to other node.
    fn add_adjacent(&self, other: &Node<T>) {
        (self.0.borrow_mut()).adjacent.push(other.0.clone());
    }
}

impl<T> Graph<T> {
    fn with_nodes(nodes: Vec<Node<T>>) -> Self {
        Graph { nodes: nodes }
    }
}

pub fn graphexample() {
    // Create some nodes
    let node_1 = Node::new(1);
    let node_2 = Node::new(2);
    let node_3 = Node::new(3);

    // Connect some of the nodes (with directed edges)
    node_1.add_adjacent(&node_2);
    node_1.add_adjacent(&node_3);
    node_2.add_adjacent(&node_1);
    node_3.add_adjacent(&node_1);

    // Add nodes to graph
    let graph = Graph::with_nodes(vec![node_1, node_2, node_3]);

    // Show every node in the graph and list their neighbors
    for node in graph.nodes.iter().map(|n| n.0.borrow()) {
        let value = node.inner_value;
        let neighbours = node.adjacent.iter()
            .map(|n| n.borrow().inner_value)
            .collect::<Vec<_>>();
        println!("node ({}) is connected to: {:?}", value, neighbours);
    }
}
// If you ignore the loop that prints out the graph’s information,
// now the user doesn’t know how a Node is implemented.
// This version’s usability can still be improved by implementing
// the std::fmt::Debug trait for Node and Graph, for instance.

// You can play with this example in the Rust Playground:
//          https://play.rust-lang.org/?gist=9ccf40fae2347519fcae7dd42ddf5ed6
// Try changing some things yourself! I find breaking things helps me consolidate new knowledge:
//      Replacing RefCell with Cell
//      Removing RefCell and using Rc<Node<T>>
//      Removing Rc and using RefCell<Node<T>>

/* ========= Cell ==========
   ========================= */
// Finally, let's mention Cell too, which can be also used in place of RefCell for interior mutability
// The most obvious difference between Cell and RefCell is that
//      RefCell makes run-time borrow checks, while Cell does not.
// Cell is quite simple to use:
//      you can read and write a Cell’s inner value by calling get or set on it.
// Since there are no compile-time or run-time checks,
// you do have to be careful to avoid some bugs the borrow checker would stop you from writing,
// such as accidentally overwriting the wrapped value:
use std::cell::Cell;
use std::time::Duration;

// like RefCell, Cell implements interior mutability:
// mutation of values in an immutable context.
fn foo(cell: &Cell<u32>) {
    let value = cell.get();
    cell.set(value * 2);
}
pub fn cellexamplee() {
    let cell = Cell::new(1);
    let new_value = cell.get() + 10;
    println!("cell value : {}", cell.get());
    // foo is going to do stuff with the cell
    foo(&cell);
    println!("cell value : {}", cell.get());
    cell.set(new_value);
    // but we can still change its contents afterwards
    // QUIZ: what will this print?
    println!("cell value : {}", cell.get());

    //
    // notice the `cell` is not mutable!
}
// so the Cell is always mutable
// In contrast, a RefCell requires you to call borrow or borrow_mut
// (immutable and mutable borrows) before using it, yielding a pointer to the value.
// Its borrow semantics are identical to externally mutable variables:
// you can have either a mutable borrow on the inner value or several immutable borrows,
// so the kind of bug I mentioned earlier is detected in run-time.

// A significant difference between Cell and RefCell is that
// Cell<T> requires that the inner value T implements Copy,
// while RefCell<T> has no such restriction.
// Often, you won’t want copy semantics on your wrapped types, so you’ll have to use RefCell.
//
// Put succinctly,
//      Cell has Copy semantics and provides values
//      RefCell has move semantics and provides references.

// we define our Rc with Cell
struct NaiveRcWithCell<T> {
    inner_value: T,
    references: Cell<usize>,
}

// implement its creation
impl<T> NaiveRcWithCell<T> {
    fn new(inner: T) -> Self {
        NaiveRcWithCell {
            inner_value: inner,
            references: Cell::new(1),
        }
    }
    fn references(&self) -> usize {
        self.references.get()
    }
}
// and implement its cloning, in a way that allows us to increment the references
impl<T: Clone> Clone for NaiveRcWithCell<T> {
    fn clone(&self) -> Self {
        self.references.set(self.references.get() + 1);
        NaiveRcWithCell {
            inner_value: self.inner_value.clone(),
            references: self.references.clone(),
        }
    }
}
// now we use our Cell Rc, and the counts are all correct!
pub fn rcwithcellexample() {
    let wrapped = NaiveRcWithCell::new("Hello!");
    println!("references before cloning: {:?}", wrapped.references());
    let wrapped_clone = wrapped.clone();
    println!("references after cloning: {:?}", wrapped.references());
    println!("clone references: {:?}", wrapped_clone.references());
    let wrapped_clone2 = wrapped.clone();
    println!("references after cloning2: {:?}", wrapped.references());
    println!("references after cloning2: {:?}", wrapped_clone.references());
    println!("references after cloning2: {:?}", wrapped_clone2.references());
}


// Calling borrow or borrow_mut on a mutably borrowed RefCell will cause a panic,
// as will calling borrow_mut on a immutably borrowed value.
// This aspect makes RefCell unsuitable to be used in a parallel scenario;
// you should use a thread-safe type (like a Mutex or a RwLock, for example) instead.
//

pub mod par{
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};
    use std::thread;

    pub fn arcmutex() {
        // let counter = Arc::new(RefCell::new(0));
        // [DNC] error[E0277]: `RefCell<i32>` cannot be shared between threads safely
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                println!("I am thread number {}",num);
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }
}

// A RefCell will stay “locked” until the pointer you received falls out of scope,
// so you might want to declare a new block scope (ie., { ... }) while working with the borrowed value,
// or even explicitly drop the borrowed value when you’re done with it, to avoid unpleasant surprises.
