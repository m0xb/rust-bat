#[allow(non_snake_case)]
fn fizzArray(size: i32) -> Vec<i32> {
    let mut fizz = vec![];
    for i in 0..size {
        fizz.push(i);
    }
    return fizz;
}

fn main() {
    let tests = vec![
        4,
        1,
        10,
        0,
        2,
        7];
    for i in tests {
        println!("{:?}", fizzArray(i));
    }
}

