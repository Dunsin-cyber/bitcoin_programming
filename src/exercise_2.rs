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

fn return_pos_result (result: i32, modulo: i32) -> i32 {
    if result < 0 {
       return result + modulo
    } else {
        result
    }

}