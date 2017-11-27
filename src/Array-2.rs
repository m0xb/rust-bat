fn countEvens (array: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in array {
        if i % 2 == 0 {
            count += 1;}
    }
    return count
}

fn sum13(array: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < array.len() {
        sum += array[i];
        if array[i] == 13 {
            sum -= array[i];
            if i+1 < array.len() {
                i += 1;
            }
        }
        i += 1;
    }
    return sum
}

fn lucky13(array: Vec<i32>) -> bool {
    for i in array {
        if i == 1 || i == 3 {
            return false
        }
    }
    return true
}


fn main() {
    let countEvens_tests = vec![
        vec![2, 1, 2, 3, 4],
        vec![2, 2, 0],
        vec![1, 3, 5],
        vec![],
        vec![11, 9, 0, 1],
        vec![2, 11, 9, 0],
        vec![2],
        vec![2, 5, 12]];
    for i in countEvens_tests {
        println!("countEvens: {}", countEvens(i));
    }
    
    println!("\n");

    let sum13_tests = vec![
        vec![1, 2, 2, 1],
        vec![1, 1],
        vec![1, 2, 2, 1, 13],
        vec![1, 2, 13, 2, 1, 13],
        vec![13, 1, 2, 13, 2, 1, 13],
        vec![],
        vec![13],
        vec![13, 13],
        vec![13, 0, 13],
        vec![13, 1, 13],
        vec![5, 7, 2],
        vec![5, 13, 2],
        vec![0],
        vec![13, 0]];
    for i in sum13_tests {
        println!("sum13: {}", sum13(i));
    }

    println!("\n");

    let lucky13_tests = vec![
        vec![0, 2, 4],
        vec![1, 2, 3],
        vec![1, 2, 4],
        vec![2, 7, 2, 8],
        vec![2, 7, 1, 8],
        vec![3, 7, 2, 8],
        vec![2, 7, 2, 1],
        vec![1, 2],
        vec![2, 2],
        vec![2],
        vec![3],
        vec![]];
    for i in lucky13_tests {
        println!("lucky13: {}", lucky13(i));
    }
}

