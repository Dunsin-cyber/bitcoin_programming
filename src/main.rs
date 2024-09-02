mod exercise_2;
mod finite_element;
mod exercise_3;
mod finite_element_imp;
mod exercise_4;
mod exercise_5;
mod exercise_6;

use std::ops::Mul;

use crate::finite_element::FiniteElement;






fn main() {
    println!("Hello, world!");
    let element_1 = FiniteElement::new(7, 13);
    let element_2 = FiniteElement::new(6, 13);

    println!("Repr func called  for element_1 at {}", element_1.repr());
    println!("Repr func called  for element_2 at {}", element_2.repr());
    println!("__Eq__ func called {}", element_1.__eq__(Option::Some(element_2.clone())));
    println!("testting eq {}", element_1 == element_2.clone());
    let cc = element_1.clone() + element_2.clone();
    println!("testting Add {}", cc.repr() );
    println!("--------------EXERCISE 2----------");
    exercise_2::addition();

    //EXERCISE 3
    println!("--------------EXERCISE 3----------");
    let ccc = element_2.clone() - element_1.clone();
    println!("testting subtract {}", ccc.repr() );


    //EXERCISE 4
    println!("--------------EXERCISE 4----------");
    exercise_4::exercise_4();



    
    //EXERCISE 5
    println!("--------------EXERCISE 5----------");
    exercise_5::exercise_5();


     //EXERCISE 6
    println!("--------------EXERCISE 6----------");
    let a = FiniteElement::new(3, 13);
    let b = FiniteElement::new(12, 13);
    let c = FiniteElement::new(10, 13);
     println!("{}", a*b == c )



}