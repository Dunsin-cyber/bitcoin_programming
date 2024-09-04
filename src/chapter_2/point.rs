use std::ops::Add;

#[derive(Debug)]
pub struct Point {
   pub x:Option<i32>,
   pub y:Option<i32>,
   pub a:i32,
   pub b:i32

}

impl Point {
  pub fn new( x:Option<i32>, y:Option<i32>, a:i32, b:i32) -> Self {
        Self {x,y,a,b};

        match (x, y) {
        (Some(x_), Some(y_)) => {
      if y_.pow(2) != x_.pow(3) + a * x_ + b {
          panic!("{} and {} does not exist on the curve", x_,y_);
        }
        }
        _ => {
            Self {x,y,a,b};
        }
      }
      Self {x,y,a,b}

    }

    /*  Exercise 2
 Write the __ne__ method for Point. */
    
    pub fn __ne__(self, other:Point) -> bool {
        if self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b  {
           return true
        } 
        false
    }

}


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