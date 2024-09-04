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
            match (self.x,self.y, rhs.x, rhs.y) {
                (Some(first_x),Some(first_y), Some(sec_x), Some(sec_y)) => {

                    let s = sec_y - first_y / sec_x - first_x;
                    let x3 = s.pow(2) - first_x - sec_x;
                    let y3 = s * (first_x - x3) - first_y;
                    
                    return Point {x:Some(x3), y:Some(y3), a:self.a, b:self.b } 
                
                }
                _ => {
                    return Point {
                        x: None,
                        y: None,
                        a: self.a,
                        b: self.b,
                    };
                }
            }
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