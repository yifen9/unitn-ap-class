// accessibility modifiers across modules

// use crate::full_files::c04_structs::Rectangle;
// DNC: Struct `Rectangle` is private [E0603]
use crate::classes::c04_structs::new_rhombus;
use crate::classes::c04_structs::Square;
use crate::classes::c04_structs::Rhombus;
use crate::classes::c04_structs::retu;
pub fn _showcase_access () {
    // Q: can i uncomment?
    // let a = retu();
    // println!("{}",a.username);

    // Q: can i uncomment?
    // let s : Square = Square{
    //     side : 01,
    // };

    // QUIZ: can i write the following:
    // let rr = Rhombus{ side: 0, acute_angle: 0 };

    let rr = new_rhombus();
    let _x = rr.side;
    // rr.side = 10;

    // let _a = rr.acute_angle;
}