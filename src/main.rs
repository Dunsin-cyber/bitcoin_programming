mod exercise_2;


#[derive(Clone)]
struct FiniteElement {
    num:u32,
    prime: u32
}

impl FiniteElement {
    fn new(num:u32, prime:u32) -> Self {
            if num >= prime  {
                panic!("Num {} not in field range 0 to {}", num, prime);
            }
        Self {num, prime}
    }

    fn repr(&self) -> String {
     return  format!( "FieldElement_{}({})", self.prime, self.num);
    }

    fn __eq__ (&self, other: Option<FiniteElement>) -> bool {
  if let Some(element) = other {
    self.num == element.num && self.prime == element.prime
  } else {
    false
}
}
//Example 1
 fn __ne__(&self, other:Option<FiniteElement>) -> bool {
    if let Some(element) = other {
        self.num != element.num || self.prime != element.prime
    } else {
        false
    }
 }
}

fn main() {
    println!("Hello, world!");
    let element_1 = FiniteElement::new(7, 13);
    let element_2 = FiniteElement::new(6, 13);

    println!("Repr func called  for element_1 at {}", element_1.repr());
    println!("Repr func called  for element_2 at {}", element_2.repr());
    println!("__Eq__ func called {}", element_1.__eq__(Option::Some(element_2.clone())));
    println!("__Ne__ func called {}", element_1.__ne__(Option::Some(element_2)));

    
    exercise_2::addition()

}