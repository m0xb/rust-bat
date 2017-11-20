fn array667(array: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut i = 0;
    while i < array.len() {
        if i+1 < array.len() && array[i] == 6 && (array[i+1] == 6 || array[i+1] == 7) {
            count += 1;
        }
        i += 1;
    }
    return count;
}

fn main() {
    let tests = vec![
        vec![6, 6, 2], 
        vec![6, 6, 2, 6], 
        vec![6, 7, 2, 6],
        vec![6, 6, 2, 6, 7],
        vec![1, 6, 3],
        vec![6, 1],
        vec![],
        vec![3, 6, 7, 6],
        vec![3, 6, 6, 7],
        vec![6, 3, 6, 6,],
        vec![6, 7, 6, 6],
        vec![1, 2, 3, 5, 6],
        vec![1, 2, 4, 6, 6]];
    for i in tests {
        println!("{}", array667(i));
    }
}

        
