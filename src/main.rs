mod finite_element;
mod finite_element_imp;
mod chapter_1;
mod chapter_2;


use crate::finite_element::FiniteElement;
use crate::chapter_1::exercise_2;





//CHAPTER !
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
    chapter_1::exercise_4::exercise_4();



    
    //EXERCISE 5
    println!("--------------EXERCISE 5----------");
    chapter_1::exercise_5::exercise_5();


     //EXERCISE 6
    println!("--------------EXERCISE 6----------");
    let a = FiniteElement::new(3, 13);
    let b = FiniteElement::new(12, 13);
    let c = FiniteElement::new(10, 13);
     println!("{}", a*b == c );

     
     

     //EXERCISE 7
    println!("--------------EXERCISE 7----------");
    // let d = FiniteElement::new(3, 13);
    // let e = FiniteElement::new(8, 13);
    //  println!("{}", d.pow(2) == e );
    chapter_1::exercise_7::exercise_7();


     //EXERCISE 8
     println!("--------------EXERCISE 8----------");
     chapter_1::exercise_8::exercise_8();

     
      //EXERCISE 9
      println!("--------------EXERCISE 9----------");
      let x = FiniteElement::new(3, 31);
      let y = FiniteElement::new(1, 31);
      let z = FiniteElement::new(4, 31);
      println!("{}", x/y == z );



 //TESTING POWER FUNC
      println!("--------------TESTING POWER FUNC----------");
      let xx = FiniteElement::new(3, 5);
      let xx_ =  xx.pow(3);  
      println!("{}", xx_.repr() );
      


      println!("--------------CHAPTER 2----------");
      println!("--------------CHAPTER 2----------");
      println!("--------------CHAPTER 2----------");
      println!("--------------CHAPTER 2----------");
      println!("--------------CHAPTER 2----------");

      println!("--------------CHAPTER 2 EXERCISE 1----------");
      chapter_2::exercise_1::exercise_1();

      println!("--------------CHAPTER 2 EXERCISE 2----------");
      chapter_2::exercise_2::exercise_2();


      println!("--------------CHAPTER 2 EXERCISE 4----------");
      chapter_2::exercise_4::exercise_4();

      
      println!("--------------CHAPTER 2 EXERCISE 6----------");
      chapter_2::exercise_6::exercise_6();


}