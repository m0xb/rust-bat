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

mod tests {
    use super::*;

    #[test]
    fn test_monkeyTrouble() {
        assert_eq!(true, monkey_trouble(true, true));
        assert_eq!(true, monkey_trouble(false, false));
        assert_eq!(false, monkey_trouble(false, true));
        assert_eq!(false, monkey_trouble(true, false));
    }
}


