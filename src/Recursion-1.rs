fn factorial(num: u32) -> u32 {
    let mut val = num;
    if num > 1 {
        val *= factorial(num - 1);
    }
    return val;
}

fn bunnyEars2(bunnies: u32) -> u32 {
    if bunnies == 0 {
          return 0;
    }
    if bunnies % 2 == 0 {
        return 3 + bunnyEars2(bunnies - 1)
    }
    return 2 + bunnyEars2(bunnies - 1);
}

fn count7(int: u32) -> u32 {
    if int == 0 {
        return 0
    } else if int % 10 == 7 {
        return 1 + count7(int/10)
    } else {
        return count7(int / 10)
    }
}

fn countX(mut s: String) -> u32 {
    if s.len() > 0 && s.chars().nth(s.len() - 1).unwrap() == 'x' {
        s.pop();
        return 1 + countX(s);
    } else if s.len() > 0 {
        s.pop();
        return countX(s);
    } else {
        return 0;
    }
}
//Starting to get the hang of this...  I think.  It takes about 15 minutes to do one of these.  So 4 per hour.
fn changePi(s: String) -> String {
    if s.len() > 0 && s.contains("pi") {
        return changePi(s.replace("pi", "3.14"))
    } else {
        return s;
    }
}

fn array11(array: Vec<i32>, index: usize) -> usize {
    if array.len() > 0 && index < array.len() {
        if array[index] == 11 {
            return 1 + array11(array, index + 1)
        } else {
            return array11(array, index + 1)
        }
    } else {
        return 0;
    }
}

//Adaptation of Greg Orlum's solution...  How the fuck does this work?  How am I iterating through the string?  Is this witchcraft?
fn pairStar(s: String) -> String {
    if s.len() > 1 {
        if s.chars().nth(0).unwrap() == s.chars().nth(1).unwrap() {
            return s[0..1].to_string() + "*" + &pairStar(s[1..s.len()].to_string())
        } else {
            return s[0..1].to_string() + &pairStar(s[1..s.len()].to_string())
        }
    } else {
        return s
    }
}


fn main() {
    let factorial_tests = vec![1, 2, 3, 4, 5, 6, 7, 8, 12];
    for i in factorial_tests {
        println!("factorial: {}", factorial(i));
    }

    println!("\n");

    let bunnyEars2_tests = vec![0, 1, 2, 3, 4, 5, 6, 10];
    for i in bunnyEars2_tests {
        println!("bunnyEars2: {}", bunnyEars2(i));
    }

    println!("\n");

    let count7_tests = vec![
    717,
    7,
    123,
    77,
    7123,
    771237,
    771737,
    47571,
    777777,
    70701277,
    777576197,
    99999,
    99799];
    for i in count7_tests {
        println!("count7: {}", count7(i));
    }

    println!("\n");

    let countX_tests = vec![
    "xxhixx",
    "xhixhix",
    "hi",
    "h",
    "x",
    "",
    "hihi",
    "hiAAhi12hi"];
    for i in countX_tests {
        println!("countX: {}", countX(i.to_string()));
    }

    println!("\n");

    let changePi_tests = vec![
    "xpix",
    "pipi",
    "pip",
    "pi",
    "hip",
    "p",
    "x",
    "",
    "pixx",
    "xyzzy"];
    for i in changePi_tests {
        println!("changePi: {}", changePi(i.to_string()));
    }

    println!("\n");

    let array11_tests = vec![
        (vec![1, 2, 11], 0),
        (vec![11, 11], 0),
        (vec![1, 2, 3, 4], 0),
        (vec![1, 11, 3, 11, 11], 0),
        (vec![11], 0),
        (vec![1], 0),
        (vec![], 0),
        (vec![11, 2, 3, 4, 11, 5], 0),
        (vec![11, 5, 11], 0)];
    for i in array11_tests {
        println!("array11: {}", array11(i.0, i.1));
    }

    println!("\n");

    let pairStar_tests = vec![
    "hello",
    "xxyy",
    "aaaa",
    "aaab",
    "aa",
    "a",
    "",
    "noadjacent",
    "abba",
    "abbba"];
    for i in pairStar_tests {
        println!("pairStar: {}", pairStar(i.to_string()));
    }

}