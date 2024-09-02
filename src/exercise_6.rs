use std::ops::Mul;

use crate::finite_element::FiniteElement;



impl Mul for FiniteElement {

    type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            if self.prime != rhs.prime {
                panic!("Cannot multiply two numbers in different Fields")
            }
            else {
                let num:u32= (self.num * rhs.num) % self.prime;
                Self::new(num, self.prime)
            }
        }
}