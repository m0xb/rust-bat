#[allow(non_snake_case)]
fn fizzBuzz(left: i32, right: i32) -> Vec<String> {
    let fizz = String::from("Fizz");
    let buzz = String::from("Buzz");
    let fizzbuzz = String::from("FizzBuzz");
    let mut fizzBuzz: Vec<String> = Vec::new();
    for i in left..right {
        if i % 3 == 0 && i % 5 == 0 {
            let _fizzbuzz_temp = &fizzbuzz;
            fizzBuzz.push(_fizzbuzz_temp.to_string());                                            
        } else if i % 3 == 0 {
            let _fizz_temp = &fizz;
            fizzBuzz.push(_fizz_temp.to_string());
        } else if i % 5 == 0 {
            let _buzz_temp = &buzz;
            fizzBuzz.push(_buzz_temp.to_string());
        }  else {
            fizzBuzz.push(i.to_string());
        }
    }
    return fizzBuzz;
}
fn main() {
    let tests = vec![
        (1, 6),
        (1, 8),
        (1, 11),
        (1, 16),
        (1, 4),
        (1, 2),
        (50, 56),
        (15, 17),
        (30, 36),
        (1000, 1006),
        (99, 102),
        (14, 20)];
    for i in tests {
        println!("{:?}", fizzBuzz(i.0, i.1));
    }
}


