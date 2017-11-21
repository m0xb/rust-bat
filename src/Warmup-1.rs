#[allow(non_snake_case)]
use std::cmp;
fn sleepIn(b1: bool, b2: bool) -> bool {
    if b2 == true {
        return true;
    }
    if b1 == false {
        return true;
    }
    return false;
}

fn parrotTrouble(b1: bool, int: i32) -> bool {
    if b1 == true && (int < 7 || int > 20) {
        return true;
    }
    return false;
}

fn makes10(a: i32, b: i32) -> bool {
    if (a == 10 || b == 10) || a + b == 10 {
        return true;
    }
    return false;
}

fn nearHundred(n: i32) -> bool {
    if (n > 89 && n < 111) || (n > 189 && n < 211) {
        return true;
    }
    return false;
}

fn posNeg(a: i32, b: i32, negative: bool) -> bool {
    if negative == false && (a < 0 && b >= 0) {
        return true;
    } else if negative == false && (a >= 0 && b < 0) {
        return true;
    } else if negative == true && a < 0 && b < 0 {
        return true;
    } else {
        return false;
    }
}

fn frontBack(s: String) -> String {
    let mut new_s = String::from(""); 
    if s.len() > 2 {
        new_s.push(s.chars().nth(s.len()-1).unwrap());
        for i in 1..s.len()-1 {
            new_s.push(s.chars().nth(i).unwrap());
        }
        new_s.push(s.chars().nth(0).unwrap());
        return new_s;
    } else if s.len() == 2 {
        new_s.push(s.chars().nth(1).unwrap());
        new_s.push(s.chars().nth(0).unwrap());
        return new_s;
    } else if s.len() == 1 {
        new_s.push(s.chars().nth(0).unwrap());
        return new_s;
    } else {
        return new_s;
    }
}

fn front3(s: String) -> String {
    let mut new_s = String::from("");
    let s_len = s.len();
    if s_len > 2 {
        let mut i = 0;
        while i < 3 {
            new_s.push(s.chars().nth(0).unwrap());
            new_s.push(s.chars().nth(1).unwrap());
            new_s.push(s.chars().nth(2).unwrap());
            i += 1;
        }
        return new_s;
    } else if s_len <= 2 {
        let mut i = 0;
        while i < 3 {
            if s_len == 1 {
                new_s.push(s.chars().nth(0).unwrap());
            }
            if s_len == 2 {
                new_s.push(s.chars().nth(0).unwrap());
                new_s.push(s.chars().nth(1).unwrap());
            }
        i += 1;
        }
        return new_s
    } else {
        return new_s;
    }

}

fn backAround(s: String) -> String {
    let mut new_s = String::with_capacity(s.len() + 2);
    let s_len = s.len();
    new_s.push(s.chars().nth(s_len - 1).unwrap());
    for i in 0..s_len {
        new_s.push(s.chars().nth(i).unwrap());
    } 
    new_s.push(s.chars().nth(s_len - 1).unwrap());
    return new_s;
}

fn or35(n: i32) -> bool {
    if n % 3 == 0 || n % 5 == 0 {
        return true;
    }
    return false;
}

fn front22(s: String) -> String {
    let s_len = s.len();
    let mut s_new = String::with_capacity(4);
    if s_len > 2 {
        let mut s_sub = String::with_capacity(2);
        s_sub.push(s.chars().nth(0).unwrap());
        s_sub.push(s.chars().nth(1).unwrap());
        s_new.push_str(&s_sub);
        s_new.push_str(&s);
        s_new.push_str(&s_sub);
        return s_new;
    } else if s_len <= 2 && s_len > 0 {
        s_new.push_str(&s);
        s_new.push_str(&s);
        s_new.push_str(&s);
        return s_new;    
    } else {
        return s_new;
    }
}

    
fn notString(s: String) -> String {
 
    let not_s = String::from("not ");
    let s_len = s.len();
    let mut new_s = String::with_capacity(s_len + 4);
    if s_len > 2 && s.chars().nth(0).unwrap() == 'n' && s.chars().nth(1).unwrap() == 'o' && s.chars().nth(2).unwrap() == 't' {
        return s;
    } else {
        new_s.push_str(&not_s);
        new_s.push_str(&s);
        return new_s;
    }
}

fn in1020(a: i32, b: i32) -> bool {
    if a > 9 && a < 21 || b > 9 && b < 21 {
        return true;
    }
    return false;
}

fn hasTeen(a: i32, b: i32, c: i32) -> bool {
    for i in 13..20 {
        if i == a || i == b || i == c {
            return true;
        }
    }
    return false;
}

fn mixStart(s: String) -> bool {
    let s_len = s.len();
    if s_len > 2 && s.chars().nth(1).unwrap() == 'i' && s.chars().nth(2).unwrap() == 'x' {
        return true;
    }
    return false;
}

fn startOz(s: String) -> String {
    let mut new_s = String::with_capacity(2);
    let s_len = s.len();
    if s.len() > 0 {
        if s.chars().nth(0).unwrap() == 'o' {
            new_s.push('o');
        }
        if s_len > 1 && s.chars().nth(1).unwrap() == 'z' {
            new_s.push('z');
        }
    }
    return new_s;
}

fn intMax(a: i32, b: i32, c: i32) -> i32 {
    if cmp::max(a, b) > cmp::max(b, c) {
        return a;
    } else if cmp::max(a, b) < cmp::max(b, c) {
        return c;
    } else {
        return b;
    }
}

fn close10(a: i32, b: i32) -> i32 {
    let a_diff = (10 - a).abs();
    let b_diff = (10 - b).abs();
    if a_diff == b_diff {
        return 0;
    } else if cmp::min(a_diff, b_diff) == a_diff {
        return a;
    } else {
        return b;
    }
}

fn in3050(a: i32, b: i32) -> bool {
    if ((a > 29 && a < 41) && (b > 29 && b < 41)) || ((a > 39 && a < 51) && (b > 39 && b < 51)) {
        return true;
    }
    return false;
}

fn max1020(a: i32, b: i32) -> i32 {
    if (a > 9 && a < 21) || (b > 9 && b < 21) {
        if (a > 9 && a < 21) && (b > 9 && b < 21) { 
            return cmp::max(a, b);
        }
        else if (a > 9 && a < 21) {
            return a;
        }
        else {
            return b;
        }
    }
    return 0;
}

fn stringE(s: String) -> bool {
    let mut count = 0;
    let s_len = s.len();
    for i in 0..s_len {
        if s.chars().nth(i).unwrap() == 'e' {
            count  += 1;
        }
    }
    if count > 0 && count < 4 {
        return true;
    }
    return false;
}

fn lastDigit(a: i32, b: i32) -> bool {
    if a % 10 == b % 10 {
        return true;
    }
    return false;
}

fn endUp(s: String) -> String {
    let s_len = s.len();
    let mut new_s = String::with_capacity(s_len);
    if s_len < 3 {
        new_s.push_str(&s.to_uppercase());
        return new_s;
    } else {
        for i in 0..s_len-3 {
        new_s.push(s.chars().nth(i).unwrap());
        }
        new_s += &s[(s_len - 3)..s_len].to_uppercase().to_string();
        return new_s;
    }
}

fn everyNth(s: String, n: i32) -> String {
    let s_len = s.len();
    let mut i = 0;
    let mut new_s = String::from("");
    while i < s_len {
        new_s.push(s.chars().nth(i).unwrap());
        i += n as usize;
   }
    return new_s;
}

#[allow(non_snake_case)]
fn main() {
    //sleepIn
    let sleepIn_tests = vec![
        (false, false),
        (true, false),
        (false, true),
        (true, true)];
    for i in sleepIn_tests {
        println!("sleepIn: {}", sleepIn(i.0, i.1));
    }
    println!("\n");

    let parrotTrouble_tests = vec![
        (true, 6),
        (true, 7),
        (false, 6),
        (true, 21),
        (false, 21),
        (false, 20),
        (true, 24),
        (false, 23),
        (true, 20),
        (false, 12)];
    for i in parrotTrouble_tests {
        println!("parrotTrouble: {}", parrotTrouble(i.0, i.1));
    }
    
    println!("\n");

    let makes10_tests = vec![
        (9, 10),
        (9, 9),
        (1, 9),
        (10, 1),
        (10, 10),
        (8, 2),
        (8, 3),
        (10, 42),
        (12, -2)];
    for i in makes10_tests {
        println!("makes10: {}", makes10(i.0, i.1));
    }

    println!("\n");

    let nearHundred_tests = vec![
        93,
        90,
        89,
        110,
        111,
        121,
        -101,
        -209,
        190,
        209,
        0,
        5,
        -50,
        191,
        189,
        200,
        210,
        211,
        290];
    for i in nearHundred_tests {
        println!("nearHundred: {}", nearHundred(i));
    }

    println!("\n");

    let posNeg_tests = vec![
        (1, -1, false),
        (-1, 1, false),
        (-4, -5, true),
        (-4, -5, false),
        (-4, 5, false),
        (-4, 5, true),
        (1, 1, false),
        (-1, -1, false),
        (1, -1, true),
        (-1, 1, true),
        (1, 1, true),
        (-1, -1, true),
        (5, -5, false),
        (-6, 6, false),
        (-5, -6, false),
        (-2, -1, false),
        (1, 2, false),
        (-5, 6, true),
        (-5, -5, true)];
    for i in posNeg_tests {
        println!("posNeg: {}", posNeg(i.0, i.1, i.2));
    }
    
    println!("\n");

    let frontBack_tests = vec![
        "code",
        "a",
        "ab",
        "abc",
        "",
        "Chocolate",
        "aavJ",
        "hello"];
    for i in frontBack_tests {
        println!("frontBack: {}", frontBack(i.to_string()));
    }
    
    println!("\n");

    let front3_tests = vec![
        "Java",
        "Chocolate",
        "abc",
        "abcXYZ",
        "ab",
        "ab",
        "a",
        ""];

    for i in front3_tests {
        println!("front3: {}", front3(i.to_string()));
    }

    println!("\n");

    let backAround_tests = vec![
        "cat",
        "Hello",
        "a",
        "abc",
        "read",
        "boo"];
    for i in backAround_tests {
        println!("backAround: {}", backAround(i.to_string()));
    }

    println!("\n");

    let or35_tests = vec![
        3,
        10,
        8,
        15,
        5,
        9,
        4,
        7,
        6,
        17,
        18,
        29,
        20,
        21,
        22,
        45,
        99,
        100,
        101,
        121,
        122,
        123];
    for i in or35_tests {
         println!("or35: {}", or35(i));
    }
    
    println!("\n");
    
    let front22_tests = vec![
        "kitten",
        "Ha",
        "abc",
        "ab",
        "a",
        "",
        "Logic"];
    for i in front22_tests {
        println!("front22: {}", front22(i.to_string()));
    }

    println!("\n");

    let notString_tests = vec![
        "not candy",
        "not x",
        "not bad",
        "bad",
        "not",
        "is not",
        "no"];
    for i in notString_tests {
        println!("notString: {}", notString(i.to_string()));
    }

    println!("\n");

    let in1020_tests = vec![
        (12, 99),
        (21, 12),
        (8, 99),
        (99, 10),
        (20, 20),
        (21, 21),
        (9, 9)];
    for i in in1020_tests {
        println!("in1020: {}", in1020(i.0, i.1));
    }
    
    println!("\n");

    let hasTeen_tests = vec![
        (13, 20, 10),
        (20, 19, 10),
        (20, 10, 13),
        (1, 20, 12),
        (19, 20, 12),
        (12, 20, 19),
        (12, 9, 20),
        (12, 18, 20),
        (14, 2, 20),
        (4, 2, 20),
        (11, 22, 22)];
    for i in hasTeen_tests {
        println!("hasTeen: {}", hasTeen(i.0, i.1, i.2));
 
    }
    
    println!("\n");

    let mixStart_tests = vec![
        "mix snacks",
        "pix snacks",
        "piz snakcs",
        "nix",
        "ni",
        "n",
        ""];
    for i in mixStart_tests {
        println!("mixStart: {}", mixStart(i.to_string()));
    }
    
    println!("\n");

    let startOz_tests = vec![
        "ozymandias",
        "bzoo",
        "oxx",
        "oz",
        "ounce",
        "o",
        "abc",
        "",
        "zoo",
        "aztec",
        "zzzz",
        "oznic",
        "z"];
    for i in startOz_tests {
        println!("startOz: {}", startOz(i.to_string()));
    }
    
    println!("\n");

    let intMax_tests = vec![
        (1, 2, 3),
        (1, 3, 2),
        (3, 2, 1),
        (9, 3, 3),
        (3, 9, 3),
        (3, 3, 9),
        (8, 2, 3),
        (-3, -1, -2),
        (6, 2, 5),
        (5, 6, 2),
        (5, 2, 6)];
    for i in intMax_tests {
        println!("intMax: {}", intMax(i.0, i.1, i.2));
    }
    
    println!("\n");

    let close10_tests = vec![
        (8, 13),
        (13, 8),
        (13, 7),
        (7, 13),
        (9, 13),
        (13, 8),
        (10, 12),
        (11,10),
        (5, 21),
        (0, 20),
        (10, 10)];
    for i in close10_tests {
        println!("close10: {}", close10(i.0, i.1));
    }

    println!("\n");

    let in3050_tests = vec![
        (30, 31),
        (30, 41),
        (40, 50),
        (40, 51),
        (39, 50),
        (50, 39),
        (40, 39),
        (49, 48),
        (50, 40),
        (50, 51),
        (35, 36),
        (35, 45)];
    for i in in3050_tests {
        println!("in3050: {}", in3050(i.0, i.1));
    }
    
    println!("\n");

    let max1020_tests = vec![
        (11, 19),
        (19, 11),
        (11, 9),
        (9, 21),
        (10, 21),
        (21, 10),
        (9, 11),
        (23, 10),
        (20, 10),
        (7, 20),
        (17, 16)];
    for i in max1020_tests {
        println!("max1020: {}", max1020(i.0, i.1));
    }
    
    println!("\n");
    
    let stringE_tests = vec![
        "Hello",
        "Heelle",
        "Heelele",
        "Hll",
        "e",
        ""];
    for i in stringE_tests {
        println!("stringE: {}", stringE(i.to_string()));
    }

    println!("\n");

    let lastDigit_tests = vec![
        (7, 17),
        (6, 17), 
        (3, 113),
        (114, 113),
        (114, 4),
        (10, 0),
        (11, 0)];
    for i in lastDigit_tests {
        println!("lastDigit: {}", lastDigit(i.0, i.1));
    }
    
    println!("\n");
    let endUp_tests = vec![
        "Hello",
        "hi there",
        "hi",
        "woo hoo",
        "xyz12",
        "x",
        ""];
    for i in endUp_tests {
        println!("endUp: {}", endUp(i.to_string()));
    }
    
    println!("\n");
    
    let everyNth_tests = vec![
        ("Miracle", 2),
        ("abcdefg", 2),
        ("abcdefg", 3),
        ("Chocolate", 3),
        ("Chocolates", 3),
        ("Chocolates", 4),
        ("Chocolates", 100)];
    for i in everyNth_tests {
        println!("everyNth: {}", everyNth(i.0.to_string(), i.1));
    }
}
