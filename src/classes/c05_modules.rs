use std::fmt::{Debug, Formatter};
///     https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

// imports, see Crate.toml
use libtest::toplevel_fun;
use libtest::pubmod::pubmodfun;
// local alias
use libtest::PubEnum as PE;

pub fn externalcall(){
    // let us now try to use the imported functions
    let s = toplevel_fun();
    let ss = pubmodfun();
    println!("Got {} and {}", s, ss);

    // we can also declare something of the imported Enum
    let _en = PE::P1;
    // println!("Enum {:?}",_en);

    use rand::random;
    let x: u8 = random();
    println!("Random u8: {}", x);
}
