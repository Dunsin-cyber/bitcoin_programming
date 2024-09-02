
use crate::finite_element::FiniteElement;
use std::ops::Div;


impl Div for FiniteElement {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot divide two numbers in different Fields")
        }
        else {
            let num = self.num * rhs.num.pow(self.prime - 2) % self.prime;
            
            Self::new (num, self.prime)
        }
    }
    
}