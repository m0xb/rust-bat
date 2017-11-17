pub fn monkey_trouble(monkey_a: bool, monkey_b: bool) -> bool {
    if monkey_a == monkey_b {
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("{}", monkey_trouble(true, true));
}

