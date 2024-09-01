fn main() {
    println!("Hello, world!");
    let abib = FiniteElement::new(3, 20);

    println!("Repr func called at {}", abib.repr());

}

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
}