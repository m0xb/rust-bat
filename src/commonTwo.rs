#[allow(non_snake_case)]
fn commonTwo(list1: Vec<char>, list2: Vec<char>) -> i32 {
    let l1_len = list1.len();
    let l2_len = list2.len();
    let mut i = 0;
    let mut j = 0;
    let mut common = 0;
    while i < l1_len && j < l2_len {
        if list1[i] > list2[j] {
            j += 1;
        } 
        else if list1[i] < list2[j] {
            i += 1;
        } else {
            common += 1;
            let temp_str = list1[i];
            while (i < l1_len && j < l2_len) && list1[i] == list2[j] && list1[i] == temp_str {
                i += 1;
                j += 1;
            }
        }
    }
    return common;
}

fn main() {
    println!("{}", commonTwo(vec!['a', 'c', 'x'], vec!['a', 'b', 'c', 'x', 'z']));
    println!("{}", commonTwo(vec!['a', 'a', 'b', 'b', 'c'], vec!['b', 'b', 'b', 'x']));
    println!("{}", commonTwo(vec!['a', 'b', 'c', 'd'], vec!['a', 'b', 'c', 'd', 'd', 'd']));
    println!("{}", commonTwo(vec!['a','b'], vec!['a','b']));
    println!("{}", commonTwo(vec!['a', 'b', 'b', 'b'], vec!['a', 'b', 'b', 'b']));
    println!("{}", commonTwo(vec!['a', 'b', 'c', 'd'], vec!['a', 'b']));
    println!("{}", commonTwo(vec!['a', 'a', 'b', 'b', 'c'], vec!['b', 'b', 'b', 'x']));
    println!("{}", commonTwo(vec![], vec![]));
    println!("{}", commonTwo(vec!['a', 'a'], vec!['b', 'b']));   
    println!("{}", commonTwo(vec!['b'], vec!['a']));
    println!("{}", commonTwo(vec!['a'], vec!['a']));
    println!("{}", commonTwo(vec!['a'], vec!['b', 'c'])); 
    println!("{}", commonTwo(vec!['a', 'b'], vec!['c']));
    println!("{}", commonTwo(vec!['a', 'b'], vec![]));
}



