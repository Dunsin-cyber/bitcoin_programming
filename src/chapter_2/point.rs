pub struct Point {
   pub x:i32,
   pub y:i32,
   pub a:i32,
   pub b:i32

}

impl Point {
  pub fn new( x:i32, y:i32, a:i32, b:i32) {
        Self {x, y, a, b};

        if y.pow(2) != x.pow(3) + a * x + b {
            panic!("{} and {} does not exist on the curve", x,y);
        }

    }

}