
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

    fn eq (&self, other: Option<FiniteElement>) -> bool {
  if let Some(element) = other {
    self.num == element.num && self.prime == element.prime
  } else {
    false
}
}
}
fn main() {
    println!("Hello, world!");
    let element_1 = FiniteElement::new(3, 20);
    let element_2 = FiniteElement::new(4, 20);

    println!("Repr func called at {}", element_1.repr());
    println!("Repr func called at {}", element_1.eq(Option::Some(element_2)));

}