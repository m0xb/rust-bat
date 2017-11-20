#[allow(non_snake_case)]
fn noTriples(array: Vec<i32>) -> bool {
    for i in 0..array.len() {
        if i+2 < array.len() && array[i] == array[i+1] && array[i+1] == array[i+2] {
            return false;
        }
    }
    return true;
}

fn main() {
    let tests = vec![
        vec![1, 1, 2, 2, 1],
        vec![1, 1, 2, 2, 2, 1],
        vec![1, 1, 1, 2, 2, 2, 1],
        vec![1, 1, 2, 2, 1, 2, 1],
        vec![1, 2, 1],
        vec![1, 1, 1],
        vec![1, 1],
        vec![1],
        vec![]];
    for i in tests {
        println!("{}", noTriples(i));
    }
}




