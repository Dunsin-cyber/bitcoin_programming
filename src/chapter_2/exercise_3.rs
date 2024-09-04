use crate::chapter_2::point::Point;
use std::ops::Add;


impl Add for Point {
    type Output = Self;
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
        if self.x != rhs.x {
            let s = rhs.y - self.y / rhs.x - self.x;
            let x3 = s.pow(2) - self.x -rhs.x;
            let y3 = s * (self.x - x3) - self.y;
           
            return Point {x:x3, y:y3, a:self.a, b:self.b } 
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