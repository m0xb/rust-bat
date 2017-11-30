fn countEvens (array: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in array {
        if i % 2 == 0 {
            count += 1;}
    }
    return count
}

fn sum13(array: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < array.len() {
        sum += array[i];
        if array[i] == 13 {
            sum -= array[i];
            if i+1 < array.len() {
                i += 1;
            }
        }
        i += 1;
    }
    return sum
}

fn lucky13(array: Vec<i32>) -> bool {
    for i in array {
        if i == 1 || i == 3 {
            return false
        }
    }
    return true
}

fn fizzArray(n: i32) -> Vec<i32> {
    let mut array: Vec<i32> = Vec::with_capacity(n as usize);
    for i in 0..n {
        array.push(i);
    }
    return array;
    }


fn no14(array: Vec<i32>) -> bool {
    if array.len() < 2 {
        return true
    }
    let mut a = false;
    let mut b = false;
    for i in array {
        if i == 1 {
            a = true;
        }
        if i == 4 {
            b = true;
        }
    }
    return a ^ b;
}

fn matchUp(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..nums1.len() {
        let temp = (nums1[i] - nums2[i]).abs();
        if temp == 1 || temp == 2 {
            count += 1;
        }
    }
    return count;
}

fn modThree(array: Vec<i32>) -> bool {
    if array.len() > 2 {
       for i in 0..(array.len() - 2) {
               if array[i] % 2 == array[i+1] % 2 && array[i+1] % 2 == array[i+2] % 2 {
                    return true
                }
            }
        }
    return false
}
//Adaptation of Gregor Ulm's solution.
fn isEverywhere(array: Vec<i32>, val: i32) -> bool {
    let mut i = 0;
    let mut b1 = true;
    let mut b2 = true;
    while i < array.len() {
        if array[i] != val {
            b1 = false;
        }
        if i + 1 < array.len() && array[i+1] != val {
            b2 = false;
            }
        i += 2;
    }
    return b1 || b2
}

fn sameEnds(array: Vec<i32>, len: usize) -> bool {
    return &array[0..len] == &array[(array.len() - len)..array.len()]; 
}

fn shiftLeft(array: Vec<i32>) -> Vec<i32> {
    let a_len = array.len();
    if a_len > 1 {
        let mut my_vec: Vec<i32> = Vec::with_capacity(a_len); 
        let left = array[1];
        let right = array[0];
        my_vec.push(left);
        let mut i = 2;
        while i < a_len {
            my_vec.push(array[i]);
            i += 1;
        }
        my_vec.push(right);
        return my_vec;   
    }
    return array;
}

fn post4(array: Vec<i32>) -> Vec<i32> {
    let mut index = array.len() - 1;
    while index > 0 {
        if array[index as usize] == 4 {
            break
        }
        index -= 1;
    } 
    return array[(index + 1)..].to_vec();
}

fn withoutTen(mut array: Vec<i32>) -> Vec<i32> {    
    let mut i = 0;
    while i < array.len() {
        if array[i] == 10 {
            array.remove(i);
            array.push(0);
        } else {
            i += 1;
        }
    }
    return array;
}

fn fizzBuzz(start: i32, end: i32) -> Vec<String> {
    let mut my_vec: Vec<String> = Vec::with_capacity((end - start) as usize);
    for i in start..end {
        if i % 15 == 0 {
            my_vec.push("FizzBuzz".to_string());
        } else if i % 5 == 0 {
            my_vec.push("Buzz".to_string());
        } else if i % 3 == 0 {
            my_vec.push("Fizz".to_string());
        } else {
            my_vec.push(i.to_string());
        }
    }
    return my_vec;
}

fn bigDiff(array: Vec<i32>) -> i32 {
    let mut big = array[0];
    let mut small = array[0];
    for i in 0..array.len() {
        if big < array[i] {
            big = array[i];
        }

        if small > array[i] {
            small = array[i];
        }
    }
    return big - small;
}

fn sum67(array: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut index = 0;
    while index < array.len() {
        if array[index as usize] != 6 {
            sum += array[index as usize];
        } else {
            while array[index as usize] != 7 {
                index += 1;
            }
        }
        index += 1;
    }
    return sum;
}

fn sum28(array: Vec<i32>) -> bool {
    let mut sum = 0;
    for i in array {
        if i == 2 {
            sum += i;
        }
    }
    return sum == 8
}

fn only14(array: Vec<i32>) -> bool {
    for i in array {
        if i != 1 && i != 4 {
            return false
        }
    }
    return true
}

//Feels cumbersome, should be a way to simplify it.
fn has77(array: Vec<i32>) -> bool {
    for i in 0..array.len() {
        if i + 1 < array.len() && array[i] == 7 {
            if array[i + 1] == 7 {
                return true
            } else if i + 2 < array.len() && array[i + 2] == 7 {
                return true
            }
        }
    }
    return false;
}

fn haveThree(array: Vec<i32>) -> bool {
    let mut count = 0;
    for i in 0..array.len() {
        if i + 1 < array.len() && array[i] == 3 && array[i + 1] == 3 {
            return false
        } else if array[i] == 3 {
            count += 1;
        }
    }
    return count == 3
}

fn tripleUp(array: Vec<i32>) -> bool {
    if array.len() > 2 {
        for i in 0..(array.len() - 2) {
            if array[i] == array[i+1] - 1 && array[i + 1] ==  array[i + 2] - 1 {
                return true;
            }
        }
    }
    return false;
}
//This would likely be shorter if I didn't make array mutable.
fn tenRun(mut array: Vec<i32>) -> Vec<i32> {
    let mut i = 0 as usize;
    while i < array.len() {
        if array[i] % 10 == 0 {
            let temp = array[i];
            while i + 1 < array.len() && array[i + 1] % 10 != 0 {
                array[i+1] = temp;
                i += 1;
            }
        }
        i += 1;
    }
    return array;
}

//Can this be done as simply without using std::cmp?
use std::cmp;
fn notAlone(mut array: Vec<i32>, val: i32) -> Vec<i32> {
    if array.len() > 2 {
        for i in 1..array.len() {
            if i + 1 < array.len() && array[i] == val {
                if array[i] < array[i - 1] || array[i] < array[i + 1] {
                    array[i] = cmp::max(array[i - 1], array[i + 1]);
                }
            }
        }
    }
    return array;
}

//Will refactor to not use cmp::max.
fn zeroMax(mut array: Vec<i32>) -> Vec<i32> {
    for i in 0..array.len() {
        if array[i] == 0 {
            for j in i..array.len() {
                if array[j] % 2 != 0 {
                    array[i] = cmp::max(array[i], array[j]);
                }
            }
        }
    }
    return array;
}

fn centeredAverage(array: Vec<i32>) -> i32 {
    let mut smallest = array[0];
    let mut biggest = array[0];
    let mut sum = 0;
    let a_len = array.len() as i32;
    for i in array {
        if i > biggest {
            biggest = i;
        }
        if i < smallest {
            smallest = i;
        }
        sum += i;
    }
    return (sum - biggest - smallest)/(a_len - 2);
}

fn has22(array: Vec<i32>) -> bool {
    for i in 0..array.len() {
        if i + 1 < array.len() && array[i..(i + 2)] == [2, 2] {
            return true;
        }
    }
    return false;
}

fn more14(array: Vec<i32>) -> bool {
    let mut c1 = 0;
    let mut c4 = 0;
    for i in array {
        if i == 1 {
            c1 += 1;
        }
        if i == 4 {
            c4 += 1;
        }
    }
    return c1 > c4;
}

//I got fancy with this one.
fn fizzArray2(n: i32) -> Vec<String> {
    return (0..n).map(|x| x.to_string()).collect();
}

fn either24(array: Vec<i32>) -> bool {
    let mut two2 = false;
    let mut four4 = false;
    for i in 0..array.len() {
        if i + 1 < array.len() {
            if array[i..(i + 2)] == [2, 2] {
                two2 = true;
            }
            if array[i..(i + 2)] == [4, 4] {
                four4 = true;
            }
        }
    }
    return two2 ^ four4;
}

fn has12(array: Vec<i32>) -> bool {
    let mut has1 = false;
    let mut has2 = false;
    for i in 0..array.len() {
        if array[i] == 1 {
            has1 = true;
            for j in i..array.len() {
                if array[j] == 2 {
                    has2 = true;
                }
            }
        }
    }
    return has1 && has2;
}

//This one was tricky.
fn twoTwo(array: Vec<i32>) -> bool {
    let mut i = 0 as usize;
    while i < array.len() {
        if array[i] == 2 {
            if i + 1 < array.len() && array[i + 1] == 2 {
                i += 2;
            } else {
                return false;
            }
        }
        i += 1;
    }
    return true
}

fn fizzArray3(start: i32, end: i32) -> Vec<i32> {
    return (start..end).collect();
}

fn pre4(mut array: Vec<i32>) -> Vec<i32> {
    for i in 0..array.len() {
        if array[i] == 4 {
            return array[0..i].to_vec();
        }
    }
    return array;
}

//I feel using insert() might not be very speedy...
fn zeroFront(mut array: Vec<i32>) -> Vec<i32> {
    for i in 0..array.len() {
        if array[i] == 0 {
            array.remove(i);
            array.insert(0, 0);
        }
    }
    return array;
}

fn evenOdd(array: Vec<i32>) -> Vec<i32> {
    let mut evens: Vec<i32> = Vec::with_capacity(array.len());
    let mut odds: Vec<i32> = Vec::with_capacity(array.len());
    for i in 0..array.len() {
        if array[i] % 2 == 0 {
            evens.push(array[i]);
        } else {
            odds.push(array[i]);
        }
    }
    evens.extend(odds);
    return evens;
}

fn main() {
    let countEvens_tests = vec![
        vec![2, 1, 2, 3, 4],
        vec![2, 2, 0],
        vec![1, 3, 5],
        vec![],
        vec![11, 9, 0, 1],
        vec![2, 11, 9, 0],
        vec![2],
        vec![2, 5, 12]];
    for i in countEvens_tests {
        println!("countEvens: {}", countEvens(i));
    }
    
    println!("\n");

    let sum13_tests = vec![
        vec![1, 2, 2, 1],
        vec![1, 1],
        vec![1, 2, 2, 1, 13],
        vec![1, 2, 13, 2, 1, 13],
        vec![13, 1, 2, 13, 2, 1, 13],
        vec![],
        vec![13],
        vec![13, 13],
        vec![13, 0, 13],
        vec![13, 1, 13],
        vec![5, 7, 2],
        vec![5, 13, 2],
        vec![0],
        vec![13, 0]];
    for i in sum13_tests {
        println!("sum13: {}", sum13(i));
    }

    println!("\n");

    let lucky13_tests = vec![
        vec![0, 2, 4],
        vec![1, 2, 3],
        vec![1, 2, 4],
        vec![2, 7, 2, 8],
        vec![2, 7, 1, 8],
        vec![3, 7, 2, 8],
        vec![2, 7, 2, 1],
        vec![1, 2],
        vec![2, 2],
        vec![2],
        vec![3],
        vec![]];
    for i in lucky13_tests {
        println!("lucky13: {}", lucky13(i));
    }
    
    println!("\n");

    let fizzArray_tests = vec![
        4,
        1, 
        10,
        0,
        2,
        7];
    for i in fizzArray_tests {
        println!("fizzArray: {:?}", fizzArray(i));
    }
   
    println!("\n");

    let no14_tests = vec![
        vec![1, 2, 3],
        vec![1, 2, 3, 4],
        vec![2, 3, 4],
        vec![1, 1, 4, 4],
        vec![2, 2, 4, 4],
        vec![2, 3, 4, 1],
        vec![2, 1, 1],
        vec![1, 4],
        vec![2],
        vec![2, 1],
        vec![1],
        vec![4],
        vec![],
        vec![1, 1, 1, 1],
        vec![9, 4, 4, 1],
        vec![4, 2, 3, 1],
        vec![4, 2, 3, 5],
        vec![4, 4, 2],
        vec![1, 4, 4]];
    for i in no14_tests {
        println!("no14: {}", no14(i));
    }

    println!("\n");

    let matchUp_tests = vec![
        (vec![1, 2, 3], vec![2, 3, 10]),
        (vec![1, 2, 3], vec![2, 3, 5]),
        (vec![1, 2, 3], vec![2, 3, 3]),
        (vec![5, 3], vec![5, 5]),
        (vec![5, 3], vec![4, 4]),
        (vec![5, 3], vec![3, 3]),
        (vec![5, 3], vec![2, 2]),
        (vec![5, 3], vec![1, 1]),
        (vec![5, 3], vec![0, 0]),
        (vec![4], vec![4]),
        (vec![4], vec![5])];
    for i in matchUp_tests {
        println!("matchUp: {}", matchUp(i.0, i.1));
    }
    
    println!("\n");

    let modThree_tests = vec![
        vec![2, 1, 3, 5],
        vec![2, 1, 2, 5],
        vec![2, 4, 2, 5],
        vec![1, 2, 1, 2, 1],
        vec![9, 9, 9],
        vec![1, 2, 1],
        vec![1, 2],
        vec![1],
        vec![],
        vec![9, 7, 2, 9],
        vec![9, 7, 2, 9, 2, 2],
        vec![9, 7, 2, 9, 2, 2, 6]];
    for i in modThree_tests {
        println!("modThree: {}", modThree(i));
    }

    println!("\n");

    let isEverywhere_tests = vec![
        (vec![1, 2, 1, 3], 1),
        (vec![1, 2, 1, 3], 2),
        (vec![1, 2, 1, 3, 4], 1),
        (vec![2, 1, 2, 1], 1),
        (vec![2, 1, 2, 1], 2),
        (vec![2, 1, 2, 3, 1], 2),
        (vec![3, 1], 3),
        (vec![3, 1], 2),
        (vec![3], 1),
        (vec![], 1),
        (vec![1, 2, 1, 2, 3, 2, 5], 2),
        (vec![1, 2, 1, 1, 1, 2], 2),
        (vec![2, 1, 2, 1, 1, 2], 2),
        (vec![2, 1, 2, 2, 2, 1, 1, 2], 2),
        (vec![2, 1, 2, 2, 2, 1, 2, 1], 2),
        (vec![2, 1, 2, 1, 2], 2),
        (vec![0, 1, 1, 0, 1, 1, 1], 1),
        (vec![0, 1, 0, 1, 1, 1, 1], 1),
        (vec![1, 1, 1], 1)];
    for i in isEverywhere_tests {
        println!("isEverywhere: {}", isEverywhere(i.0, i.1));
    }
    
    println!("\n");

    let sameEnds_tests = vec![
        (vec![5, 6, 45, 99, 13, 5, 6], 1),
        (vec![5, 6, 45, 99, 13, 5, 6], 2),
        (vec![5, 6, 45, 99, 13, 5, 6], 3),
        (vec![1, 2, 5, 2, 1], 1),
        (vec![1, 2, 5, 2, 1], 2),
        (vec![1, 2, 5, 2, 1], 0),
        (vec![1, 2, 5, 2, 1], 5),
        (vec![1, 1, 1], 0),
        (vec![1, 1, 1], 1),
        (vec![1, 1, 1], 2),
        (vec![1, 1, 1], 3),
        (vec![1], 1),
        (vec![], 0),
        (vec![4, 2, 4, 5], 1)];
    for i in sameEnds_tests {
        println!("sameEnds: {}", sameEnds(i.0, i.1));
    }

    println!("\n");

    let shiftLeft_tests = vec![
        vec![6, 2, 5, 3],
        vec![1, 2],
        vec![1],
        vec![],
        vec![1, 1, 2, 2, 4],
        vec![1, 1, 1],
        vec![1, 2, 3]];
    for i in shiftLeft_tests {
        println!("shiftLeft: {:?}", shiftLeft(i));
    }
    
    println!("\n");

    let post4_tests = vec![
        vec![2, 4, 1, 2],
        vec![4, 1, 4, 2],
        vec![4, 4, 1, 2, 3],
        vec![4, 2],
        vec![4, 4, 3],
        vec![4, 4],
        vec![4],
        vec![2, 2, 1, 4, 3, 2],
        vec![4, 1, 4, 2, 2, 2],
        vec![3, 4, 3, 2]];
    for i in post4_tests {
        println!("post4: {:?}", post4(i));
    }

    println!("\n");

    let withoutTen_tests = vec![
        vec![1, 10, 10, 2],
        vec![10, 2, 10],
        vec![1, 99, 10],
        vec![10, 13, 10, 14],
        vec![10, 13, 10, 14, 10],
        vec![10, 10, 3],
        vec![1],
        vec![13, 1],
        vec![10],
        vec![]];
    for i in withoutTen_tests {
        println!("withoutTen: {:?}", withoutTen(i));
    }
    
    println!("\n");

    let fizzBuzz_tests = vec![
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
    for i in fizzBuzz_tests {
        println!("fizzBuzz: {:?}", fizzBuzz(i.0, i.1));
    }

    println!("\n");

    let bigDiff_tests = vec![
        vec![10, 3, 5, 6],
        vec![7, 2, 10, 9],
        vec![2, 10, 7, 2],
        vec![2, 10],
        vec![10, 2],
        vec![10, 0],
        vec![2, 3],
        vec![2, 2],
        vec![2],
        vec![5, 1, 6, 1, 9, 9],
        vec![7, 6, 8, 5],
        vec![7, 7, 6, 8, 5, 5, 6]];
    for i in bigDiff_tests {
        println!("bigDiff: {}", bigDiff(i));
    }

    println!("\n");
    
    let sum67_tests = vec![
        vec![1, 2, 2],
        vec![1, 2, 2, 6, 99, 99, 7],
        vec![1, 1, 6, 7, 2],
        vec![1, 6, 2, 2, 7, 1, 6, 99, 99, 7],
        vec![1, 6, 2, 6, 7, 1, 6, 99, 99, 7],
        vec![2, 7, 6, 2, 6, 7, 2, 7],
        vec![2, 7, 6, 2, 6, 2, 7],
        vec![1, 6, 7, 7],
        vec![6, 7, 1, 6, 7, 7],
        vec![6, 8, 1, 6, 7],
        vec![],
        vec![6, 7, 11],
        vec![11, 6, 7, 11],
        vec![2, 2, 6, 7, 7]];
    for i in sum67_tests {
        println!("sum67: {}", sum67(i));
    }
    
    println!("\n");
    
    let sum28_tests = vec![
        vec![2, 3, 2, 2, 4, 2],
        vec![2, 3, 2, 2, 4, 2, 2],
        vec![1, 2, 3, 4],
        vec![2, 2, 2, 2],
        vec![1, 2, 2, 2, 2, 4],
        vec![],
        vec![2],
        vec![8],
        vec![2, 2, 2],
        vec![2, 2, 2, 2, 2],
        vec![1, 2, 2, 1, 2, 2],
        vec![5, 2, 2, 2, 4, 2]];
    for i in sum28_tests {
        println!("sum28: {}", sum28(i));
    }

    println!("\n");

    let only14_tests = vec![
        vec![1, 4, 1, 4],
        vec![1, 4, 2, 4],
        vec![1, 1],
        vec![4, 1],
        vec![2],
        vec![],
        vec![1, 4, 1, 3],
        vec![3, 1, 3],
        vec![1],
        vec![4],
        vec![3, 4],
        vec![1, 3, 4],
        vec![1, 1, 1],
        vec![1, 1, 1, 5],
        vec![4, 1, 4, 1]];
    for i in only14_tests {
        println!("only14: {}", only14(i));
    }

    println!("\n");

    let has77_tests = vec![
        vec![1, 7, 7],
        vec![1, 7, 1, 7],
        vec![1, 7, 1, 1, 7],
        vec![7, 7, 1, 1, 7],
        vec![2, 7, 2, 2, 7, 2],
        vec![2, 7, 2, 2, 7, 7],
        vec![7, 2, 7, 2, 2, 7],
        vec![7, 2, 6, 2, 2, 7],
        vec![7, 7, 7],
        vec![7, 1, 7],
        vec![7, 1, 1],
        vec![1, 2],
        vec![1, 7],
        vec![]];

    for i in has77_tests {
        println!("has77: {}", has77(i));
    }
    
    println!("\n");

    let haveThree_tests = vec![
        vec![3, 1, 3, 1, 3],
        vec![3, 1, 3, 3],
        vec![3, 4, 3, 3, 4],
        vec![1, 3, 1, 3, 1, 2],
        vec![1, 3, 1, 3, 1, 3],
        vec![1, 3, 3, 3, 1, 3],
        vec![1, 3, 1, 3, 1, 3, 4, 3],
        vec![3, 4, 3, 4, 3, 4, 4],
        vec![3, 3, 3, 3],
        vec![1, 3],
        vec![3],
        vec![1]];
    for i in haveThree_tests {
        println!("haveThree: {}", haveThree(i));
    }
    
    println!("\n");

    let tripleUp_tests = vec![
        vec![1, 4, 5, 6, 2],
        vec![1, 2, 3],
        vec![1, 2, 4],
        vec![1, 2, 4, 5, 7, 6, 5, 6, 7, 6],
        vec![1, 2, 4, 5, 7, 6, 5, 7, 7, 6],
        vec![1, 2],
        vec![1],
        vec![],
        vec![10, 9, 8, -100, -99, -98, 100],
        vec![10, 9, 8, -100, -99, 99, 100],
        vec![-100, -99, -99, 100, 101, 102],
        vec![2, 3, 5, 6, 8, 9, 2, 3]];
    for i in tripleUp_tests {
        println!("tripleUp: {}", tripleUp(i));
    }
    
    println!("\n");

    let tenRun_tests = vec![
        vec![2, 10, 3, 4, 20, 5],
        vec![2, 10, 3, 4, 20, 5, 5],
        vec![2, 10, 3, 4, 20, 5, 30, 5, 5],
        vec![10, 1, 20, 2],
        vec![10, 1, 9, 20],
        vec![1, 2, 50, 1],
        vec![1, 20, 50, 1],
        vec![10, 10],
        vec![10, 2],
        vec![0, 0],
        vec![1, 2],
        vec![1],
        vec![]];
    for i in tenRun_tests {
        println!("tenRun: {:?}", tenRun(i));
    }
    
    println!("\n");

    let notAlone_tests = vec![
        (vec![1, 2, 3], 2),
        (vec![1, 2, 3, 2, 5, 2], 2),
        (vec![3, 4], 3),
        (vec![3, 3], 3),
        (vec![1, 3, 1, 2], 1),
        (vec![3], 3),
        (vec![], 3),
        (vec![7, 1, 6], 1),
        (vec![1, 1, 1], 1),
        (vec![1, 1, 1, 2], 1)];
    for i in notAlone_tests {
        println!("notAlone: {:?}", notAlone(i.0, i.1));
    }
    
    println!("\n");

    let zeroMax_tests = vec![
        vec![0, 5, 0, 3],
        vec![0, 4, 0, 3],
        vec![0, 1, 0],
        vec![0, 1, 5],
        vec![0, 2, 0],
        vec![1],
        vec![0],
        vec![],
        vec![7, 0, 4, 3, 0, 2],
        vec![7, 0, 4, 3, 0, 1],
        vec![7, 0, 4, 3, 0, 0],
        vec![7, 0, 1, 0, 0, 7]];
    for i in zeroMax_tests {
        println!("zeroMax: {:?}", zeroMax(i));
    }

    println!("\n");

    let centeredAverage_tests = vec![
        vec![1, 2, 3, 4, 100],
        vec![1, 1, 5, 5, 10, 8, 7],
        vec![-10, -4, -2, -4, -2, 0],
        vec![5, 3, 4, 6, 2],
        vec![5, 3, 4, 0, 100],
        vec![100, 0, 5, 3, 4],
        vec![4, 0, 100],
        vec![0, 2, 3, 4, 100],
        vec![1, 1, 100],
        vec![7, 7, 7],
        vec![1, 7, 8],
        vec![1, 1, 99, 99],
        vec![1000, 0, 1, 99],
        vec![4, 4, 4, 4, 5],
        vec![4, 4, 4, 1, 5],
        vec![6, 4, 8, 12, 3]];
    for i in centeredAverage_tests {
        println!("centeredAverage: {}", centeredAverage(i));
    }
    
    println!("\n");

    let has22_tests = vec![
        vec![1, 2, 2],
        vec![1, 2, 1, 2],
        vec![2, 1, 2],
        vec![2, 2, 1, 2],
        vec![1, 3, 2],
        vec![1, 3, 2, 2],
        vec![2, 3, 2, 2],
        vec![4, 2, 4, 2, 2, 5],
        vec![1, 2],
        vec![2, 2],
        vec![2],
        vec![],
        vec![3, 3, 2, 2],
        vec![5, 2, 5, 2]];
    for i in has22_tests {
        println!("has22: {}", has22(i));
    }
    
    println!("\n");

    let more14_tests = vec![
        vec![1, 4, 1],
        vec![1, 4, 1, 4],
        vec![1, 1],
        vec![1, 6, 6],
        vec![1],
        vec![1, 4],
        vec![6, 1, 1],
        vec![1, 6, 4],
        vec![1, 1, 4, 4, 1],
        vec![1, 1, 6, 4, 4, 1],
        vec![],
        vec![4, 1, 4, 6],
        vec![4, 1, 4, 6, 1],
        vec![1, 4, 1, 4, 1, 6]];
    for i in more14_tests {
        println!("more14: {}", more14(i));
    }

    println!("\n");

    let fizzArray2_tests = vec![4, 10, 2, 1, 0, 7, 9, 11];
    for i in fizzArray2_tests {
        println!("fizzArray2: {:?}", fizzArray2(i));
    }

    println!("\n");

    let either24_tests = vec![
        vec![1, 2, 2],
        vec![4, 4, 1],
        vec![4, 4, 1, 2, 2],
        vec![1, 2, 3, 4],
        vec![3, 5, 9],
        vec![1, 2, 3, 4, 4],
        vec![2, 2, 3, 4],
        vec![1, 2, 3, 2, 2, 4],
        vec![1, 2, 3, 2, 2, 4, 4],
        vec![1, 2],
        vec![2, 2],
        vec![4, 4],
        vec![2],
        vec![]];
    for i in either24_tests {
        println!("either24: {}", either24(i));
    }

    println!("\n");

    let has12_tests = vec![
        vec![1, 3, 2],
        vec![3, 1, 2],
        vec![3, 1, 4, 5, 2],
        vec![3, 1, 4, 5, 6],
        vec![3, 1, 4, 1, 6, 2],
        vec![2, 1, 4, 1, 6, 2],
        vec![2, 1, 4, 1, 6],
        vec![1],
        vec![2, 1, 3],
        vec![2, 1, 3, 2],
        vec![2],
        vec![3, 2],
        vec![3, 1, 3, 2],
        vec![3, 5, 9],
        vec![3, 5, 1],
        vec![3, 2, 1],
        vec![1, 2]];
    for i in has12_tests {
        println!("has12: {}", has12(i));
    }
    
    println!("\n");

    let twoTwo_tests = vec![
        vec![4, 2, 2, 3],
        vec![2, 2, 4],
        vec![2, 2, 4, 2],
        vec![1, 3, 4],
        vec![1, 2, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![2, 2],
        vec![2, 2, 7],
        vec![2, 2, 7, 2, 1],
        vec![4, 2, 2, 2],
        vec![2, 2, 2],
        vec![1, 2],
        vec![2],
        vec![1],
        vec![],
        vec![5, 2, 2, 3],
        vec![2, 2, 5, 2],
        vec![2, 2, 5, 2, 2, 2, 5, 2, 2, 5],
        vec![2, 2, 5, 2, 2, 2, 5, 2, 5],
        vec![3, 3, 2, 2, 2, 1, 1, 3]];
    for i in twoTwo_tests {
        println!("twoTwo: {}", twoTwo(i));
    }

    println!("\n");

    let fizzArray3_tests = vec![
        (5, 10),
        (11, 18),
        (1, 3),
        (1, 2),
        (1, 1),
        (1000, 1005)];
    for i in fizzArray3_tests {
        println!("fizzArray3: {:?}", fizzArray3(i.0, i.1));
    }
    
    println!("\n");

    let pre4_tests = vec![
        vec![1, 2, 4, 1],
        vec![3, 1, 4],
        vec![1, 4, 4],
        vec![1, 4, 4, 2],
        vec![1, 3, 4, 2, 4],
        vec![4, 4],
        vec![3, 3, 4],
        vec![1, 2, 1, 4],
        vec![2, 1, 4, 2],
        vec![2, 1, 2, 1, 4, 2]];
    for i in pre4_tests {
        println!("pre4: {:?}", pre4(i));
    }
    
    println!("\n");

    let zeroFront_tests = vec![
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 0, 1],
        vec![1, 0],
        vec![0, 1],
        vec![1, 1, 1, 0],
        vec![2, 2, 2, 2],
        vec![0, 0, 1, 0],
        vec![-1, 0, 0, -1, 0],
        vec![0, -3, 0, -3],
        vec![],
        vec![9, 9, 0, 9, 0, 9]];
    for i in zeroFront_tests {
        println!("zeroFront: {:?}", zeroFront(i));
    }
    
    println!("\n");

    let evenOdd_tests = vec![
        vec![1, 0, 1, 0, 0, 1, 1],
        vec![3, 3, 2],
        vec![2, 2, 2],
        vec![3, 2, 2],
        vec![1, 1, 0, 1, 0],
        vec![1],
        vec![1, 2],
        vec![2, 1],
        vec![]];
    for i in evenOdd_tests {
        println!("evenOdd: {:?}", evenOdd(i));
    }
}
