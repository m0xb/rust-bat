fn arrayCount9(list: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in list {
        if i == 9 {
            count += 1;
        }
    }
    return count
}

fn main() {
    println!("{}", arrayCount9(vec![1, 2, 9]));
    println!("{}", arrayCount9(vec![1, 9, 9]));
    println!("{}", arrayCount9(vec![1, 9, 9, 3, 9]));
    println!("{}", arrayCount9(vec![1, 2, 2]));
    println!("{}", arrayCount9(vec![]));
    println!("{}", arrayCount9(vec![4, 2, 4, 3, 1]));
    println!("{}", arrayCount9(vec![9, 4, 4, 3, 1]));
}

