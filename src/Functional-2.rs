fn noNeg(list: Vec<i32>) -> Vec<i32> {
    let no_neg: Vec<i32> = list.into_iter().filter(|&x| !x.is_negative()).collect();
    return no_neg
}

fn main() {
    let noNeg_tests = vec![
        vec![1, -2],
        vec![-3, -3, 3, 3],
        vec![-1, -1, -1],
        vec![],
        vec![0, 1, 2],
        vec![3, -10, 1, -1, 4, -400],
        vec![-1, 3, 1, -1, -10, -100, -111, 5]];
    for i in noNeg_tests {
        println!("noNeg: {:?}", noNeg(i));
    }
}