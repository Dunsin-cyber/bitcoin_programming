
/* 
 Exercise 5
 For k = 1, 3, 7, 13, 18, what is this set in F19
 ?
 {k ⋅ 0, k ⋅ 1, k ⋅ 2, k ⋅ 3, ... k ⋅ 18}
 Do you notice anything about these sets?
*/

pub fn exercise_5 () {
    let k = vec![1,3,7, 13, 18];
    let arry = [0, 1,2, 3, 5, 6, 7, 8, 9, 10, 11, 12,13,14,15,16,17,18];
    // let field = []

    for k_ in k.iter() {
        let field: Vec<u32> = arry.iter().map(|x| (k_ * x) % 19 ).collect();

            println!("{:?}", field)
    }
}