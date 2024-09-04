/* 
 Exercise 4
 For the curve y2 = x3 + 5x + 7, what is (2,5) + (–1,–1)
*/

pub fn exercise_4 () {
    let s: i32 = -1 -5 / -1 -2;
    let x3 = s.pow(2) - 2 + 1;
    let y3 = s * (2 - x3) - 5;

    println!("x3 is {} and y3 is {}", x3,y3); 
}