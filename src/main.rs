mod exercise_2;
mod finite_element;
mod exercise_3;
mod finite_element_imp;

use crate::finite_element::FiniteElement;






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