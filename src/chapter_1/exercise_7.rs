/* 
Exercise 7
 For p = 7, 11, 17, 31, what is this set in Fp
 ?
 {1(p – 1), 2(p – 1), 3(p – 1), 4(p – 1), ... (p – 1)(p – 1)
*/

use num_bigint::BigUint;
use num_traits::FromPrimitive;

pub fn exercise_7 () {
    let p: [u32; 4] = [7, 11, 17, 31];
    let mut  i = BigUint::from_u32(1).unwrap();

    for &x in p.iter() {
        let mut arry: Vec<BigUint> = Vec::new();
        while i < x.into() {
           let val:BigUint = (i.pow(x-1) % x as u128) as BigUint;
           arry.push(val);

            i += BigUint::from_u32(1).unwrap();
        }
        i = BigUint::from_u32(1).unwrap();
        println!("values for {} is {:?}", x,arry )
    }
}