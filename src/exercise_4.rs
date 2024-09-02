

use num_bigint::BigUint;
use num_traits::FromPrimitive;
/* 
Exercise 4
 Solve the following equations in F97
 (again, assume ⋅ and exponentiation are field
 versions):
 • 95 ⋅ 45 ⋅ 31
 • 17 ⋅ 13 ⋅ 19 ⋅ 44
 • 127 ⋅ 7749
*/

pub fn exercise_4 () {
        let a = (95 * 45 * 31) % 97;
        let b = (17 * 13 * 19 * 44) % 97;
        let first_digit:u128 = 12;
        let sec_digit:u128 = 77;
        let first_pow:u32 = 7;
        let sec_pow:u32 = 49;
        let val_1: u128 = first_digit.pow(first_pow);
        let val_2: BigUint = BigUint::from(sec_digit).pow(sec_pow);
        let modulo = BigUint::from_u32(97).unwrap();
        let c = (val_1 * val_2.clone()) % modulo ;




        println!("{}", a);
        println!("{}", b);
        println!("{}", c)
}