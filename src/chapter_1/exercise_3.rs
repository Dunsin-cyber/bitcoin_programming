use std::ops::Sub;

use crate::{chapter_1, finite_element::FiniteElement};


impl Sub for FiniteElement {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot subtract two numbers in different Fields");
            
        } else {
            let num = ((self.num as i32 - rhs.num as i32) % self.prime as i32) as i32;
            let res = chapter_1::exercise_2::return_pos_result(num, self.prime as i32);
            Self::new(res, self.prime)
        }
    }

}