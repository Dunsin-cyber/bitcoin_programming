use crate::chapter_2::point::Point;

pub fn exercise_2 () {
    let q = Point::new(Some(-1),Some(-1),5, 7);
    let r= Point::new(Some(18),Some(77),5, 7);
   


    println!("{}", q.__ne__(r) );

}