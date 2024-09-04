use crate::chapter_2::point::Point;
use std::ops::Add;


impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, rhs);
        }

        if self.x == None {
            return rhs
        }
        if rhs.x == None {
            return self
        } 
        if self.x == rhs.x && self.y != rhs.y {
            return Point {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            };
        }
        else {
            return Point {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            };
        }
    }
}