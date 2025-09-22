///  https://doc.rust-lang.org/std/string/struct.String.html
pub fn strings(){
    let str1 : &str = "wat";
    let mut str2 = "wht";
    let s:String = String::from("asd");
    str2 = "mehs";
    println!("Strings {} and {}", str1 , str2);
    // what can one do with a &str and with a String?
    // str_string.
    // s.

    let str_string = String::from(str1);  // owned string
    // Q: what tells us that `strptr` is a reference?
    let strptr_string : &String = &str_string;  // ref to string (borrow)
    /* Strings ,visually:
            strptr_string                 str_string                 heap stuff
          | name  | value |         | name     | value |        | index | value |
          | ptr   | ----------------> ptr      | ---------------> 0     | h     |
                                    | length   | 5     |        | 1     | e     |
                                    | capacity | 5     |        | 2     | l     |
                                                                | 3     | l     |
                                                                | 4     | o     |
    */
    let str_slice : &str = &str_string[..2];
    println!("This is a slice {}", str_slice);
    println!("This is (not) a pointer: {}", strptr_string);
    println!("This is a pointer: {:p}", strptr_string);

    let s = "hell";
    // Q: what is the type of _s? (erase _ to reveal type inference)
    let s0 = & "hell".to_string();
    // s0.push('a');  // decomment and inspect the signature of 'push'
    // Q: why does this does not typecheck ?

    let mut s = "hell".to_string();
    let t = String::from("o world");
    s.push_str(&t);
    // inspect the signature of 'push_str'

    // Another way of manipulating strings is with `format!`
    let r = format!("{} is real t{}lings", s0, t);
    println!("{}",r);

    // let h = s0[0];   // Strings are non-indexable arrays
    // DNC: the type `String` cannot be indexed by `{integer}`
    for c in s0.chars() {
        println!("Char: {:?}", c)
    }
}

///     https://doc.rust-lang.org/std/vec/struct.Vec.html
pub fn vec(){
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(5);
    v.push(6);
    v.push(5);
    v.pop();
    let o = v.get(1);   // check this type after 'get'
    let n = v.get(0).unwrap();  // then this type after 'unwrap'
    let mut nn = v.get(2).unwrap();

    // QUIZ: can i do this:
    let nn = (nn + n);
    // nn = nn3;


    // let nn = *nn;       //shadowing!
    println!("Adding stuff {}", nn + n);

    let mut vv : Vec<String> = Vec::new();
    vv.push("asd".to_string());
    let mut x = vv.get_mut(0).unwrap(); //notice 'get_mut' instead of 'get'
    let xx = x.push('a');
    println!("{}--",x);

    // immutable iteration
    for i in &v {
        println!("{}", i);
    }
    // mutable iteration
    for i in &mut v {
        *i += 50;
    }
    println!("Vector v {:?}", v);   // print Debug gable stuff
}

pub fn mutability(){
    // Q: what can you do on something of type pointer?

    let x : &mut String = &mut String::from("asd");
    // Q: what is the type of 'x'?
    // x = x;
    // Q: can i decomment this?

    let mut z : &String = & String::from("ad");
    z = z;
    // z.push('a');
    // Q: can i decomment this?

    let mut y : &mut String = &mut String::from("asdasd");
    y.push('a');        // fine: y points to a mutable String
    y = x;                 // also fine: y itself is mutable

}

///     https://doc.rust-lang.org/std/collections/struct.HashMap.html
pub fn hashmap(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("Hashmap content: {}: {}", key, value);
    }
    //overwrite
    scores.insert(String::from("Blue"), 25);
    //get or insert in two different ways
    // with `get` and the related handling of options
    let blue_scores = scores.get("Blue").unwrap();
    println!("blue: {}", blue_scores);
}