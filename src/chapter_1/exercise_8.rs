
use num_bigint::BigUint;
use num_traits::FromPrimitive;

/* 
Exercise 8
 Solve the following equations in F31
 :
 • 3 / 24
 • 17^–3
 • 4^-4 ⋅ 11
*/

pub fn exercise_8 () {
     let modulo = BigUint::from_u32(31).unwrap();
    let a:BigUint = BigUint::from_u32(3).unwrap() * (BigUint::from_u32(24).unwrap().pow(31-2) % modulo.clone() ) % modulo.clone();
    let pow_1 = (31_u32-4_u32);
    let pow_2 = (31_u32-5_u32);
    let b: BigUint = BigUint::from_u32(17).unwrap().pow(pow_1)  % modulo.clone();
    let left_side :  BigUint = BigUint::from_u32(1).unwrap() * BigUint::from_u32(4).unwrap().pow(pow_2) %  modulo.clone();
    let c = left_side * BigUint::from_u32(11).unwrap() % modulo.clone();
  
  
    println!("{}", a);
    println!("{}", b);
    println!("{}", c)




}