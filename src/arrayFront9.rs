#[allow(non_snake_case)]
fn arrayFront9(array: Vec<i32>) -> bool {
    for i in 0..array.len() {
        if i < 4 && array[i] == 9 {
            return true;
        }
    }
    return false;
}

fn main() {
    let tests = vec![
        vec![1, 2, 9, 3, 4],
        vec![1, 2, 3, 4, 9],
        vec![1, 2, 3, 4, 5],
        vec![9, 2, 3],
        vec![1, 9, 9],
        vec![1, 2, 3],
        vec![1, 9],
        vec![5, 5],
        vec![2],
        vec![9],
        vec![],
        vec![3, 9, 2, 3 , 3]];
    for i in tests {
        println!("{}", arrayFront9(i));
    }
}

    

