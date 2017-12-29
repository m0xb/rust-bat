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
    return 3 - bunnies%2 + bunnyEars2(bunnies - 1);
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
//This can [very probably] be simplified.
fn countAbc(s: String) -> u32 {
    if s.len() > 2 {
        if s.chars().nth(0).unwrap() == 'a' && s.chars().nth(1).unwrap() == 'b' {
            if s.chars().nth(2).unwrap() == 'a' {
                return 1 + &countAbc(s[2..s.len()].to_string())
            } else if s.chars().nth(2).unwrap() == 'c' {
                return 1 + &countAbc(s[2..s.len()].to_string())
            } else {
                return countAbc(s[3..s.len()].to_string())
            }
        } else {
            return countAbc(s[3..s.len()].to_string())
        }
    } else {
        return 0
    }
}

fn countHi2(s: String) -> u32 {
    if s.len() > 1 {
        if &s[0..2] == "xh" {
            return countHi2(s[2..s.len()].to_string())
        } else if &s[0..2] == "hi" {
            return 1 + countHi2(s[2..s.len()].to_string())
        } else {
            return countHi2(s[1..s.len()].to_string())
        }
    } else {
        return 0
    }
}

fn strCount(s: String, sub: String) -> u32 {
    if s.len() >= sub.len() {
        if &s[0..sub.len()] == sub {
            return 1 + strCount(s[sub.len()..s.len()].to_string(), sub)
        } else {
            return strCount(s[1..s.len()].to_string(), sub)
        }
    } else {
        return 0
    }
}

fn bunnyEars(bunnies: u32) -> u32 {
    if bunnies > 0 {
        return 2 + bunnyEars(bunnies - 1)
    } else {
        return 0
    }
}

fn triangle(rows: u32) -> u32 {
    if rows > 0 {
        return rows + triangle(rows - 1)
    } else {
        return 0
    }
}

fn count8(num: u32) -> u32 {
    if num > 0 {
        if num % 100 == 88 {
            return 2 + count8(num / 10)
        } else if num % 10 == 8 {
            return 1 + count8(num / 10)
        } else {
            return count8(num / 10)
        }
    } else {
        return 0
    }
}

fn countHi(s: String) -> u32 {
    if s.len() > 1 {
        if &s[0..2] == "hi" {
            return 1 + countHi(s[2..s.len()].to_string())
        } else {
            return countHi(s[1..s.len()].to_string())
        }
    } else {
        return 0
    }
}

fn noX(s: String) -> String {
    if s.len() > 0 {
        if &s[0..1] == "x" {
            return noX(s[1..s.len()].to_string())
        } else {
            return s[0..1].to_string() + &noX(s[1..s.len()].to_string())
        }
    } else {
        return s
    }
}

fn array220(array: Vec<i32>, start_index: usize) -> bool {
    if start_index + 1 < array.len() {
        if array[start_index + 1] == 10*array[start_index] {
            return true
        } else {
            return array220(array[(start_index + 1)..array.len()].to_vec(), start_index)
        }
    } else {
        return false
    }
}

fn endX(mut s: String) -> String {
    if s.len() > 0 {
        if s.chars().nth(0).unwrap() == 'x' {
            return endX(s[1..s.len()].to_string()) + "x"
        } else {
            return s.chars().nth(0).unwrap().to_string() + &endX(s[1..s.len()].to_string())
        }
    } else {
        return s
    }
}

fn count11(s: String) -> u32 {
    if s.len() > 1 {
        if &s[0..2] == "11" {
            return 1 + count11(s[2..s.len()].to_string())
        } else {
            return count11(s[1..s.len()].to_string())
        }
    } else {
        return 0
    }
}
//This doesn't quite work recursively I feel.  I spent a while trying to get this to work without using an iterator, but gave up.
//Using the find() method.
fn parenBit(s: String) -> String {
    if s.len() > 0 {
        if s.chars().nth(0).unwrap() == '(' {
            //let index = s.find(')');
            //return s[0..index.unwrap_or_else(|| 0)].to_string() + &")".to_string()
            let index = s.find(')');
            if let Some(res) = index {
                return s[0..res].to_string() + ")"
            } else {
                panic!("k1mp0ss1bru!!1!")
            }
        } else {
            return parenBit(s[1..s.len()].to_string())
        }
    } else {
        return s
    }
}

fn strCopies(s: String, sub: String, n: u32) -> bool {
    if s.len() >= sub.len() && n > 0{
        if s[0..sub.len()] == sub {
               return strCopies(s[1..s.len()].to_string(), sub, n - 1)
        } else {
            return strCopies(s[1..s.len()].to_string(), sub, n)
        }
    } else if n == 0 {
        return true
    } else {
        return false
    }
}

fn fibonacci(n: u32) -> u32 {
    if n > 1 {
        return fibonacci(n - 2) + fibonacci(n - 1)
    } else {
        return n
    }
}

fn sumDigits(num: u32) -> u32 {
    if num > 0 {
        return num % 10 + sumDigits(num/10)
    } else {
        return 0
    }
}

//This may need to be expanded to cover negative and fractional exponents and raising negative numbers to odd exponents.
fn powerN(base: i32, n: i32) -> i32 {
    if n > 1 {
        return base * powerN(base, n - 1)
    } else {
        return base
    }
}

/*fn changeXY(mut s: String) -> String {
    if s.len() > 0 {
        if s.chars().nth(0).unwrap() == 'x' {

        }
    } else {
        return s
    }
}
*/

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

    println!("\n");

    let countAbc_tests = vec![
    "abc",
    "abcxxabc",
    "abaxxaba",
    "ababc",
    "abxbc",
    "aaabc",
    "hello",
    "",
    "ab",
    "aba",
    "aca",
    "aaa"];
    for i in countAbc_tests {
        println!("countAbc: {}", countAbc(i.to_string()));
    }

    println!("\n");

    let countHi2_tests = vec![
    "ahixhi",
    "ahibhi",
    "xhixhi",
    "hixhi",
    "hixhhi",
    "hihihix",
    "hihihix",
    "xhihihix",
    "xxhi",
    "hixxhi",
    "hi",
    "xxxx",
    "h",
    "x",
    "",
    "Hellohi"];
    for i in countHi2_tests {
        println!("countHi2: {} input: {}", countHi2(i.to_string()), i);
    }

    println!("\n");

    let strCount_tests = vec![
    ("catcowcat", "cat"),
    ("catcowcat", "cow"),
    ("catcowcat", "dog"),
    ("cacatcowcat", "cat"),
    ("xyx", "x"),
    ("iiiijj", "i"),
    ("iiiijj", "ii"),
    ("iiiijj", "iii"),
    ("iiiijj", "j"),
    ("iiiijj", "jj"),
    ("aaabababab", "ab"),
    ("aaabababab", "aa"),
    ("aaabababab", "a"),
    ("aaabababab", "b"),];
    for i in strCount_tests {
        println!("strCount: {}", strCount(i.0.to_string(), i.1.to_string()));
    }

    println!("\n");

    let bunnyEars_tests = vec![0, 1, 2, 3, 4, 5, 12, 50, 234];
    for i in bunnyEars_tests {
        println!("bunnyEars: {}", bunnyEars(i));
    }

    println!("\n");

    let triangle_tests = vec![0, 1, 2, 3, 4, 5, 6, 7];
    for i in triangle_tests {
        println!("triangle: {}", triangle(i));
    }

    println!("\n");

    let count8_tests = vec![8, 818, 8818, 8088, 123, 81238, 88788,
    8234, 2348, 23884, 0, 1818188, 8818181, 1080, 188, 88888, 9898, 78];
    for i in count8_tests {
        println!("count8: {}", count8(i));
    }

    println!("\n");

    let countHi_tests = vec![
    "xxhixx",
    "xhixhix",
    "hi",
    "hihih",
    "h",
    "",
    "ihihihihih",
    "ihihihihihi",
    "hiAAhi12hi",
    "xhixhxihihhhih",
    "ship"];
    for i in countHi_tests {
        println!("countHi: {}", countHi(i.to_string()));
    }

    println!("\n");

    let noX_tests = vec![
    "xaxb",
    "abc",
    "xx",
    "",
    "axxbxx",
    "Hellox"];
    for i in noX_tests {
        println!("noX: {}", noX(i.to_string()));
    }

    println!("\n");

    let array220_tests = vec![
    (vec![1, 2, 20], 0),
    (vec![3, 30], 0),
    (vec![3], 0),
    (vec![], 0),
    (vec![3, 3, 30, 4], 0),
    (vec![2, 19, 4], 0),
    (vec![20, 2, 21], 0),
    (vec![20, 2, 21, 210], 0),
    (vec![2, 200, 2000], 0),
    (vec![0, 0], 0),
    (vec![1, 2, 3, 4, 5, 6], 0),
    (vec![1, 2, 3, 4, 5, 50, 6], 0),
    (vec![1, 2, 3, 4, 5, 51, 6], 0),
    (vec![1, 2, 3, 4, 4, 50, 500, 6], 0)];
    for i in array220_tests {
        println!("array220:  Array: {:?}, Index: {}, Result: {}", i.0, i.1, array220(i.0.clone(), i.1));
    }

    println!("\n");

    let endX_tests = vec![
    "xxre",
    "xxhixx",
    "xhixhix",
    "hiy",
    "h",
    "x",
    "xx",
    "",
    "bxx",
    "bxax",
    "axaxax",
    "xxhxi"];
    for i in endX_tests {
        println!("endX: {}", endX(i.to_string()));
    }

    println!("\n");

    let count11_tests = vec![
    "11abc11",
    "abc11x11x11",
    "111",
    "1111",
    "1",
    "",
    "hi",
    "11x111x1111",
    "1x111",
    "1Hello1",
    "Hello"];
    for i in count11_tests {
        println!("count11: {}", count11(i.to_string()));
    }

    println!("\n");

    let parenBit_tests = vec![
    "xyz(abc)123",
    "x(hello)",
    "(xy)1",
    "not really (possible)",
    "(abc)",
    "(abc)xyz",
    "(abc)x",
    "(x)",
    "()",
    "res (ipsa) loquitor",
    "hello(not really)there",
    "ab(ab)ab"];
    for i in parenBit_tests {
        println!("parenBit: {}", parenBit(i.to_string()));
    }

    println!("\n");

    let strCopies_tests = vec![
        ("catcowcat", "cat", 2),
        ("catcowcat", "cow", 2),
        ("catcowcat", "cow", 1),
        ("iiijjj", "i", 3),
        ("iiijjj", "i", 4),
        ("iiijjj", "ii", 2),
        ("iiijjj", "ii", 3),
        ("iiijjj", "x", 3),
        ("iiijjj", "x", 0),
        ("iiiiij", "iii", 3),
        ("iiiiij", "iii", 4),
        ("ijiiiiij", "iiii", 2),
        ("ijiiiiij", "iiii", 3),
        ("dogcatdogcat", "dog", 2)];
    for i in strCopies_tests {
        println!("strCopies: {}", strCopies(i.0.to_string(), i.1.to_string(), i.2))
    }

    println!("\n");

    let fibonacci_tests = vec![0, 1, 2, 3, 4, 5, 6, 7];
    for i in fibonacci_tests {
        println!("fibonacci: {}", fibonacci(i));
    }

    println!("\n");

    let sumDigits_tests = vec![126, 49, 12, 10, 1, 0, 730, 1111, 11111, 10110, 235];
    for i in sumDigits_tests {
        println!("sumDigits: {}", sumDigits(i));
    }

    println!("\n");

    let powerN_tests = vec![
        (3, 1),
        (3, 2),
        (3, 3),
        (2, 1),
        (2, 2),
        (2, 3),
        (2, 4),
        (2, 5),
        (10, 1),
        (10, 2),
        (10, 3)];
    for i in powerN_tests {
        println!("powerN: {}", powerN(i.0, i.1))
    }

}
