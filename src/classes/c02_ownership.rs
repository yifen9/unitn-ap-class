///     https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
pub fn ownership(){
    // allocates s1
    let s1 = String::from("hello");
    {   //  a new scope block
        let _s2 = s1;// moves s1 into s2
        // decomment the print below
        // println!("{}", s1); // error, `s1` doesn't own the value anymore.
    }
    // Q: what happens when I decomment this line?
    // println!("{}", s1);

    {   // If we really want to keep `s1` and `s2` at the same time,
        // we can `clone` it explicitly:
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }
    // ownership for Copy types
    let x: i32 = 5;
    {
        let y = x;
        println!("x = {}, y = {}", x, y); //works
    }
}

// Q: when is the memory for the heap-allocated `s` freed ?
pub fn ownership_for_functions() {
    let s = String::from("hello");
    takes_ownership( s );
    // Q: is s live here?
    let x = 5;
    // 'x' is Copy, so it is copied, not moved
    makes_copy(x);
    //
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

///     https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
pub fn refs_and_borrowing(){
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);    // note the '&'
    println!("The length of '{}' is {}.", s1, len);
    // QUIZ: can i uncomment:
    // let len = (&s1).push('a');

    let mut s = String::from("hello");
    change(&mut s);

    let r1 = &mut s;
    // QUIZ: does this code compile? can i uncomment this?
    // let r2 = &mut s;
    let r2 = "asd";
    println!("r1 and r2: {} and {}", r1, r2);

    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("r1 and r2 and r3: {} and {} and {}", r1, r2, r3);
    // QUIZ: does this code compile? can we uncomment this line?
    let r3 = &mut s;
    // let x = s;
    println!("r3: {} ", r3);
    // println!("{}",x);
    //
    // println!("r1 and r2 and r3: {} and {} and {}", r1, r2, r3);

    // see dangle

    let string = no_dangle();
    println!("String {}",string);
}

/// Example function used for borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Example function used for borrowing
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &'static String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s   // return s; // equivalent
}

///     https://doc.rust-lang.org/book/ch04-03-slices.html
pub fn slices(){
    let s : String = String::from("hello world");
    let _hello : &str = &s[0..5];
    let _world : &str = &s[6..11];

    let _slice = &s[0..2];  // these are equal
    let _slice = &s[..2];

    let len = s.len();  // these are equal too
    let _slice = &s[3..len];
    let _slice = &s[3..];

    let a = [1, 2, 3, 4, 5];
    // let slice1 = &a[0..7];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let v = vec![1, 2, 3, 4, 5];
    let v = &v[1..4];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}


pub fn ownership_and_compound(){
    let mut  v = vec![String::from("something");10];
    // QUIZ: can i uncomment both lines?
    // let first_nonmut = v[0];
    // let sec_nonmut = v[1];

    let first_nonnmut = &v[0];
    // note that this now is a &String

    let mut first_mut = v.get_mut(0).unwrap();
    first_mut.push_str(" else");
    println!("First Element: {}",first_mut);

    let second_nonnmut = v.get(1).unwrap();
    // QUIZ: what happens if we write
    // println!("First Element: {}",first_mut);
    // nothing: it works // compiler error

    // let cheat = &mut first_mut;  //DNC
    let third_nonmut = v.get(2).unwrap();
    println!("Nonmut: second and third {}, {}", second_nonnmut, third_nonmut);

    let (v05, v6plus) = v.split_at_mut(6);
    let mut one_mut = v05.get_mut(5).unwrap();
    one_mut.push('a');
    let mut two_mut = v6plus.get_mut(0).unwrap();
    println!("5: {}, 6: {}",one_mut, two_mut);

    let mut vv = [v05,v6plus].concat();
    println!("{:?} == {:?}",v,vv);
}
