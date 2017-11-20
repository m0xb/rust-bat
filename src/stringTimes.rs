#[allow(non_snake_case)]
fn stringTimes(s: String, num: i32) -> String {
    let mut new_s = String::from("");
    let mut i = 0;
    while i < num {
        new_s.push_str(&s);
        i += 1;
    }
    return new_s;
}

fn main() {
   let tests = vec![
       ("Hi", 2),
       ("Hi", 3),
       ("Hi", 1),
       ("Hi", 0),
       ("Hi", 5),
       ("Oh Boy!", 2),
       ("x", 4),
       ("", 4),
       ("code", 2),
       ("code", 3)];
   for i in tests {
       println!("{}", stringTimes(i.0.to_string(), i.1));
   }
}

