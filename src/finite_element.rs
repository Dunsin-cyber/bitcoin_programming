
#[derive(Clone, PartialEq)]
pub struct FiniteElement {
  pub  num:u32,
   pub prime: u32
}


 impl FiniteElement {
   pub fn new(num:u32, prime:u32) -> Self {
            if num >= prime  {
                panic!("Num {} not in field range 0 to {}", num, prime);
            }
        Self {num, prime}
    }

   pub fn repr(&self) -> String {
     return  format!( "FieldElement_{}({})", self.prime, self.num);
    }

  pub  fn __eq__ (&self, other: Option<FiniteElement>) -> bool {
        if let Some(element) = other {
          self.num == element.num && self.prime == element.prime
        } else {
          false
      }
}

//Example 1
pub fn __ne__(&self, other:Option<FiniteElement>) -> bool {
    if let Some(element) = other {
        self.num != element.num || self.prime != element.prime
    } else {
        false
    }
 }

pub fn pow(self, element:u32) -> Self {
  let num = self.num.pow(element) % self.prime;

  Self::new(num, self.prime)
}
 }