pub fn addition () {

    let a = (44 + 33) % 57;
    let b = (9 -29) % 57;
    let c = ( 17 + 42 + 49) % 57;
    let d = ( 52 - 30 - 38) % 57;

    println!("Answer for a in exercise 2 is {}",  return_pos_result(a, 57));
    println!("Answer for b in exercise 2 is {}", return_pos_result(b, 57)); 
    println!("Answer for c in exercise 2 is {}", return_pos_result(c, 57)); 
    println!("Answer for d in exercise 2 is {}", return_pos_result(d, 57));
}

/* 
In Rust, the % operator computes the remainder of division, 
but it follows the rules of "truncating division,"
 which means the remainder will have the same sign 
 as the dividend (the left-hand operand). 
 This can lead to results that are different from what you might expect 
 if you're looking for a positive remainder, especially when dealing with negative numbers.
*/

pub fn return_pos_result (result: i32, modulo: i32) -> i32 {
    if result < 0 {
       return result + modulo
    } else {
        result
    }

}