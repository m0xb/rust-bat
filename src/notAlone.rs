use std::cmp;
#[allow(non_snake_case)]
fn notAlone(array: Vec<i32>, value: i32) -> Vec<i32> {
    let mut s_array = vec![];
    let a_len = array.len();
    if a_len == 0 {
        return s_array;
    } else if a_len == 1 {
        s_array.push(array[0]);
        return s_array;
    } else if a_len == 2 {
        s_array.push(array[0]);
        s_array.push(array[1]);
        return s_array;
    } else {
        for i in 0..a_len {
            if i > 0 && i + 1 < a_len && array[i] == value && (array[i - 1] != value && array[i+1] != value) && (array[i - 1] > value || array[i + 1] > value) {
                s_array.push(cmp::max(array[i - 1], array[i + 1]));
            } else {
                s_array.push(array[i]);
            }
        }
    }
    return s_array;    
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], 2),
        (vec![1, 2, 3, 2, 5, 2], 2),
        (vec![3, 4], 3),
        (vec![3, 3], 3),
        (vec![1, 3, 1, 2], 1),
        (vec![3], 3),
        (vec![], 3),
        (vec![7, 1, 6], 1),
        (vec![1, 1, 1], 1),
        (vec![1, 1, 1, 2], 1)];
    for i in tests {
        println!("{:?}", notAlone(i.0, i.1));
    }
}


        
        
