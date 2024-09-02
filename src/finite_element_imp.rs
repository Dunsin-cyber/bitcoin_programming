
use std::ops::Add;

use crate::finite_element::FiniteElement;

trait PartialEq {
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq for FiniteElement {
    fn eq(&self, other: &Self) -> bool {
        if  self.num == other.num && self.prime == other.prime {
                true           
          } else {
            false
        }
    }
}


impl Add for FiniteElement {


    type Output = Self;
 fn add(self, other:FiniteElement) -> Self {

        if self.prime != other.prime {
            panic!("'Cannot add two numbers in different Fields");
            
        } else {
            let num = (self.num + other.num) % self.prime;
            // let res = exercise_2::return_pos_result(self.num, self.prime);
            Self::new(num, self.prime)
        }
    }
}

