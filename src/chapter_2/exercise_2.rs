use crate::chapter_2::point::Point;

pub fn exercise_2 () {
    let q = Point::new(-1,-1,5, 7);
    let r= Point::new(18,77,5, 7);
   


    println!("{}", q.__ne__(r) );

}