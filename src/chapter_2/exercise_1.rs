/*  
Exercise 1
 Determine which of these points are on the curve y2 = x3 + 5x + 7:
 (2,4), (–1,–1), (18,77), (5,7)
*/
use crate::chapter_2::point::Point;

pub fn exercise_1 () {
    // let p = Point::new(2,4,5, 7);
    let q = Point::new(-1,-1,5, 7);
    let r= Point::new(18,77,5, 7);
    // let s = Point::new(5,7,5, 7);


    // println!("{:?}", p);
    println!("{:?}", q);
    println!("{:?}", r);
    // println!("{:?}", s);
}