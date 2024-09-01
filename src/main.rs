fn main() {
    println!("Hello, world!");
    let abib = FiniteElement::new(40, 20);

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
}