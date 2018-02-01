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
            if i + 1 < array.len() && array[i] == val && array[i - 1] != val && array[i + 1] != val {
                if val < array[i - 1] || val < array[i + 1]{
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
#[macro_use]
mod bat_formatter;
fn main() {

    printbat!(countEvens,
    vec![2, 1, 2, 3, 4] => 3,
    vec![2, 2, 0] => 3,
    vec![1, 3, 5] => 0,
    Vec::<i32>::new() => 0,
    vec![11, 9, 0, 1] => 1,
    vec![2, 11, 9, 0] => 2,
    vec![2] => 1,
    vec![2, 5, 12] => 2);

    println!();

    printbat!(bigDiff,
    vec![10, 3, 5, 6] => 7,
    vec![7, 2, 10, 9] => 8,
    vec![2, 10, 7, 2] => 8,
    vec![2, 10] => 8,
    vec![10, 2] => 8,
    vec![10, 0] => 10,
    vec![2, 3] => 1,
    vec![2, 2] => 0,
    vec![2] => 0,
    vec![5, 1, 6, 1, 9, 9] => 8,
    vec![7, 6, 8, 5] => 3,
    vec![7, 7, 6, 8, 5, 5, 6] => 3);

    println!();

    printbat!(centeredAverage,
    vec![1, 2, 3, 4, 100] => 3,
    vec![1, 1, 5, 5, 10, 8, 7] => 5,
    vec![-10, -4, -2, -4, -2, 0] => -3,
    vec![5, 3, 4, 6, 2] => 4,
    vec![5, 3, 4, 0, 100] => 4,
    vec![100, 0, 5, 3, 4] => 4,
    vec![4, 0, 100] => 4,
    vec![0, 2, 3, 4, 100] => 3,
    vec![1, 1, 100] => 1,
    vec![7, 7, 7] => 7,
    vec![1, 7, 8] => 7,
    vec![1, 1, 99, 99] => 50,
    vec![1000, 0, 1, 99] => 50,
    vec![4, 4, 4, 4, 5] => 4,
    vec![4, 4, 4, 1, 5] => 4,
    vec![6, 4, 8, 12, 3] => 6);

    println!();

    printbat!(sum13,
    vec![1, 2, 2, 1] => 6,
    vec![1, 1] => 2,
    vec![1, 2, 2, 1, 13] => 6,
    vec![1, 2, 13, 2, 1, 13] => 4,
    vec![13, 1, 2, 13, 2, 1, 13] => 3,
    Vec::<i32>::new() => 0,
    vec![13] => 0,
    vec![13, 13] => 0,
    vec![13, 0, 13] => 0,
    vec![13, 1, 13] => 0,
    vec![5, 7, 2] => 14,
    vec![5, 13, 2] => 5,
    vec![0] => 0,
    vec![13, 0] => 0);

    println!();

    printbat!(sum67,
    vec![1, 2, 2] => 5,
    vec![1, 2, 2, 6, 99, 99, 7] => 5,
    vec![1, 1, 6, 7, 2] => 4,
    vec![1, 6, 2, 2, 7, 1, 6, 99, 99, 7] => 2,
    vec![1, 6, 2, 6, 2, 7, 1, 6, 99, 99, 7] => 2,
    vec![2, 7, 6, 2, 6, 7, 2, 7] => 18,
    vec![2, 7, 6, 2, 6, 2, 7] => 9,
    vec![1, 6, 7, 7] => 8,
    vec![6, 7, 1, 6, 7, 7] => 8,
    vec![6, 8, 1, 6, 7] => 0,
    Vec::<i32>::new() => 0,
    vec![6, 7, 11] => 11,
    vec![11, 6, 7, 11] => 22,
    vec![2, 2, 6, 7, 7] => 11);

    println!();

    printbat!(has22,
    vec![1, 2, 2] => true,
    vec![1, 2, 1, 2] => false,
    vec![2, 1, 2] => false,
    vec![2, 2, 1, 2] => true,
    vec![1, 3, 2] => false,
    vec![1, 3, 2, 2] => true,
    vec![2, 3, 2, 2] => true,
    vec![4, 2, 4, 2, 2, 5] => true,
    vec![1, 2] => false,
    vec![2, 2] => true,
    vec![2] => false,
    Vec::<i32>::new() => false,
    vec![3, 3, 2, 2] => true,
    vec![5, 2, 5, 2] => false);

    println!();

    printbat!(lucky13,
    vec![0, 2, 4] => true,
    vec![1, 2, 3] => false,
    vec![1, 2, 4] => false,
    vec![2, 7, 2, 8] => true,
    vec![2, 7, 1, 8] => false,
    vec![3, 7, 2, 8] => false,
    vec![2, 7, 2, 1] => false,
    vec![1, 2] => false,
    vec![2, 2] => true,
    vec![2] => true,
    vec![3] => false,
    Vec::<i32>::new() => true);

    println!();

    printbat!(sum28,
    vec![2, 3, 2, 2, 4, 2] => true,
    vec![2, 3, 2, 2, 4, 2, 2] => false,
    vec![1, 2, 3, 4] => false,
    vec![2, 2, 2, 2] => true,
    vec![1, 2, 2, 2, 2, 4] => true,
    Vec::<i32>::new() => false,
    vec![2] => false,
    vec![8] => false,
    vec![2, 2, 2] => false,
    vec![2, 2, 2, 2, 2] => false,
    vec![1, 2, 2, 1, 2, 2] => true,
    vec![5, 2, 2, 2, 4, 2] => true);

    println!();

    printbat!(more14,
    vec![1, 4, 1] => true,
    vec![1, 4, 1, 4] => false,
    vec![1, 1] => true,
    vec![1, 6, 6] => true,
    vec![1] => true,
    vec![1, 4] => false,
    vec![6, 1, 1] => true,
    vec![1, 6, 4] => false,
    vec![1, 1, 4, 4, 1] => true,
    vec![1, 1, 6, 4, 4, 1] => true,
    Vec::<i32>::new() => false,
    vec![4, 1, 4, 6] => false,
    vec![4, 1, 4, 6, 1] => false,
    vec![1, 4, 1, 4, 1, 6] => true);

    println!();

    printbat!(fizzArray,
    4 => vec![0, 1, 2, 3],
    1 => vec![0],
    10 => vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    0 => Vec::<i32>::new(),
    2 => vec![0, 1],
    7 => vec![0, 1, 2, 3, 4, 5, 6]);

    println!();

    printbat!(only14,
    vec![1, 4, 1, 4] => true,
    vec![1, 4, 2, 4] => false,
    vec![1, 1] => true,
    vec![4, 1] => true,
    vec![2] => false,
    Vec::<i32>::new() => true,
    vec![1, 4, 1, 3] => false,
    vec![3, 1, 3] => false,
    vec![1] => true,
    vec![4] => true,
    vec![3, 4] => false,
    vec![1, 3, 4] => false,
    vec![1, 1, 1] => true,
    vec![1, 1, 1, 5] => false,
    vec![4, 1, 4, 1] => true);

    println!();

    printbat!(fizzArray2,
    4 => vec!["0", "1", "2", "3"],
    10 => vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    2 => vec!["0", "1"],
    1 => vec!["0"],
//    0 => Vec::<String>::new(),
    7 => vec!["0", "1", "2", "3", "4", "5", "6"],
    9 => vec!["0", "1", "2", "3", "4", "5", "6", "7", "8"],
    11 => vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]);

    println!();

    printbat!(no14,
    vec![1, 2, 3] => true,
    vec![1, 2, 3, 4] => false,
    vec![2, 3, 4] => true,
    vec![1, 1, 4, 4] => false,
    vec![2, 2, 4, 4] => true,
    vec![2, 3, 4, 1] => false,
    vec![2, 1, 1] => true,
    vec![1, 4] => false,
    vec![2] => true,
    vec![2, 1] => true,
    vec![1] => true,
    vec![4] => true,
    Vec::<i32>::new() => true,
    vec![1, 1, 1, 1] => true,
    vec![9, 4, 4, 1] => false,
    vec![4, 2, 3, 1] => false,
    vec![4, 2, 3, 5] => true,
    vec![4, 4, 2] => true,
    vec![1, 4, 4] => false);

    println!();

    printbat!(isEverywhere,
    vec![1, 2, 1, 3], 1 => true,
    vec![1, 2, 1, 3], 2 => false,
    vec![1, 2, 1, 3, 4], 1 => false,
    vec![2, 1, 2, 1], 1 => true,
    vec![2, 1, 2, 1], 2 => true,
    vec![2, 1, 2, 3, 1], 2 => false,
    vec![3, 1], 3 => true,
    vec![3, 1], 2 => false,
    vec![3], 1 => true,
    Vec::<i32>::new(), 1 => true,
    vec![1, 2, 1, 2, 3, 2, 5], 2 => true,
    vec![1, 2, 1, 1, 1, 2], 2 => false,
    vec![2, 1, 2, 1, 1, 2], 2 => false,
    vec![2, 1, 2, 2, 2, 1, 1, 2], 2 => false,
    vec![2, 1, 2, 2, 2, 1, 2, 1], 2 => true,
    vec![2, 1, 2, 1, 2], 2 => true);

    println!();

    printbat!(either24,
    vec![1, 2, 2] => true,
    vec![4, 4, 1] => true,
    vec![4, 4, 1, 2, 2] => false,
    vec![1, 2, 3, 4] => false,
    vec![3, 5, 9] => false,
    vec![1, 2, 3, 4, 4] => true,
    vec![2, 2, 3, 4] => true,
    vec![1, 2, 3, 2, 2, 4] => true,
    vec![1, 2, 3, 2, 2, 4, 4] => false,
    vec![1, 2] => false,
    vec![2, 2] => true,
    vec![4, 4] => true,
    vec![2] => false,
    Vec::<i32>::new() => false);

    println!();

    printbat!(matchUp,
    vec![1, 2, 3], vec![2, 3, 10] => 2,
    vec![1, 2, 3], vec![2, 3, 5] => 3,
    vec![1, 2, 3], vec![2, 3, 3] => 2,
    vec![5, 3], vec![5, 5] => 1,
    vec![5, 3], vec![4, 4] => 2,
    vec![5, 3], vec![3, 3] => 1,
    vec![5, 3], vec![2, 2] => 1,
    vec![5, 3], vec![1, 1] => 1,
    vec![5, 3], vec![0, 0] => 0,
    vec![4], vec![4] => 0,
    vec![4], vec![5] => 1);

    println!();

    printbat!(has77,
    vec![1, 7, 7] => true,
    vec![1, 7, 1, 7] => true,
    vec![1, 7, 1, 1, 7] => false,
    vec![7, 7, 1, 1, 7] => true,
    vec![2, 7, 2, 2, 7, 2] => false,
    vec![2, 7, 2, 2, 7, 7] => true,
    vec![7, 2, 7, 2, 2, 7] => true,
    vec![7, 2, 6, 2, 2, 7] => false,
    vec![7, 7, 7] => true,
    vec![7, 1, 7] => true,
    vec![7, 1, 1] => false,
    vec![1, 2] => false,
    vec![1, 7] => false,
    vec![7] => false);

    println!();

    printbat!(has12,
    vec![1, 3, 2] => true,
    vec![3, 1, 2] => true,
    vec![3, 1, 4, 5, 2] => true,
    vec![3, 1, 4, 5, 6] => false,
    vec![3, 1, 4, 1, 6, 2] => true,
    vec![2, 1, 4, 1, 6, 2] => true,
    vec![2, 1, 4, 1, 6] => false,
    vec![1] => false,
    vec![2, 1, 3] => false,
    vec![2, 1, 3, 2] => true,
    vec![2] => false,
    vec![3, 2] => false,
    vec![3, 1, 3, 2] => true,
    vec![3, 5, 9] => false,
    vec![3, 5, 1] => false,
    vec![3, 2, 1] => false,
    vec![1, 2] => true);

    println!();

    printbat!(modThree,
    vec![2, 1, 3, 5] => true,
    vec![2, 1, 2, 5] => false,
    vec![2, 4, 2, 5] => true,
    vec![1, 2, 1, 2, 1] => false,
    vec![9, 9, 9] => true,
    vec![1, 2, 1] => false,
    vec![1, 2] => false,
    vec![1] => false,
    Vec::<i32>::new() => false,
    vec![9, 7, 2, 9] => false,
    vec![9, 7, 2, 9, 2, 2] => false,
    vec![9, 7, 2, 9, 2, 2, 6] => true);

    println!();

    printbat!(haveThree,
    vec![3, 1, 3, 1, 3] => true,
    vec![3, 1, 3, 3] => false,
    vec![3, 4, 3, 3, 4] => false,
    vec![1, 3, 1, 3, 1, 2] => false,
    vec![1, 3, 1, 3, 1, 3] => true,
    vec![1, 3, 3, 1, 3] => false,
    vec![1, 3, 1, 3, 1, 3, 4, 3] => false,
    vec![3, 4, 3, 4, 3, 4, 4] => true,
    vec![3, 3, 3] => false,
    vec![1, 3] => false,
    vec![3] => false,
    vec![1] => false);

    println!();

    printbat!(twoTwo,
    vec![4, 2, 2, 3] => true,
    vec![2, 2, 4] => true,
    vec![2, 2, 4, 2] => false,
    vec![1, 3, 4] => true,
    vec![1, 2, 2, 3, 4] => true,
    vec![1, 2, 3, 4] => false,
    vec![2, 2] => true,
    vec![2, 2, 7] => true,
    vec![2, 2, 7, 2, 1] => false,
    vec![4, 2, 2, 2] => true,
    vec![2, 2, 2] => true,
    vec![1, 2] => false,
    vec![2] => false,
    vec![1] => true,
    Vec::<i32>::new() => true,
    vec![5, 2, 2, 3] => true,
    vec![2, 2, 5, 2] => false);

    println!();

    printbat!(sameEnds,
    vec![5, 6, 45, 99, 13, 5, 6], 1 => false,
    vec![5, 6, 45, 99, 13, 5, 6], 2 => true,
    vec![5, 6, 45, 99, 13, 5, 6], 3 => false,
    vec![1, 2, 5, 2, 1], 1 => true,
    vec![1, 2, 5, 2, 1], 2 => false,
    vec![1, 2, 5, 2, 1], 0 => true,
    vec![1, 2, 5, 2, 1], 5 => true,
    vec![1, 1, 1], 0 => true,
    vec![1, 1, 1], 1 => true,
    vec![1, 1, 1], 2 => true,
    vec![1, 1, 1], 3 => true,
    vec![1], 1 => true,
    Vec::<i32>::new(), 0 => true,
    vec![4, 2, 4, 5], 1 => false);

    println!();

    printbat!(tripleUp,
    vec![1, 4, 5, 6, 2] => true,
    vec![1, 2, 3] => true,
    vec![1, 2, 4] => false,
    vec![1, 2, 4, 5, 7, 6, 5, 6, 7, 6] => true,
    vec![1, 2, 4, 5, 7, 6, 5, 7, 7, 6] => false,
    vec![1, 2] => false,
    vec![1] => false,
    Vec::<i32>::new() => false,
    vec![10, 9, 8, -100, -99, -98, 100] => true,
    vec![10, 9, 8, -100, -99, 99, 100] => false,
    vec![-100, -99, -99, 100, 101, 102] => true,
    vec![2, 3, 5, 6, 8, 9, 2, 3] => false);

    println!();

    printbat!(fizzArray3,
    5, 10 => vec![5, 6, 7, 8, 9],
    11, 18 => vec![11, 12, 13, 14, 15, 16, 17],
    1, 3 => vec![1, 2],
    1, 2 => vec![1],
    1, 1 => Vec::<i32>::new(),
    1000, 1005 => vec![1000, 1001, 1002, 1003, 1004]);

    println!();

    printbat!(shiftLeft,
    vec![6, 2, 5, 3] => vec![2, 5, 3, 6],
    vec![1, 2] => vec![2, 1],
    vec![1] => vec![1],
    Vec::<i32>::new() => Vec::<i32>::new(),
    vec![1, 1, 2, 2, 4] => vec![1, 2, 2, 4, 1],
    vec![1, 1, 1] => vec![1, 1, 1],
    vec![1, 2, 3] => vec![2, 3, 1]);

    println!();

    printbat!(tenRun,
    vec![2, 10, 3, 4, 20, 5] => vec![2, 10, 10, 10, 20, 20],
    vec![10, 1, 20, 2] => vec![10, 10, 20, 20],
    vec![10, 1, 9, 20] => vec![10, 10, 10, 20],
    vec![1, 2, 50, 1] => vec![1, 2, 50, 50],
    vec![1, 20, 50, 1] => vec![1, 20, 50, 50],
    vec![10, 10] => vec![10, 10],
    vec![10, 2] => vec![10, 10],
    vec![0, 2] => vec![0, 0],
    vec![1, 2] => vec![1, 2],
    vec![1] => vec![1],
    Vec::<i32>::new() => Vec::<i32>::new());

    println!();

    printbat!(pre4,
    vec![1, 2, 4, 1] => vec![1, 2],
    vec![3, 1, 4] => vec![3, 1],
    vec![1, 4, 4] => vec![1],
    vec![1, 4, 4, 2] => vec![1],
    vec![1, 3, 4, 2, 4] => vec![1, 3],
    vec![4, 4] => Vec::<i32>::new(),
    vec![3, 3, 4] => vec![3, 3],
    vec![1, 2, 1, 4] => vec![1, 2, 1],
    vec![2, 1, 4, 2] => vec![2, 1],
    vec![2, 1, 2, 1, 4, 2] => vec![2, 1, 2, 1]);

    println!();

    printbat!(post4,
    vec![2, 4, 1, 2] => vec![1, 2],
    vec![4, 1, 4, 2] => vec![2],
    vec![4, 4, 1, 2, 3] => vec![1, 2, 3],
    vec![4, 2] => vec![2],
    vec![4, 4, 3] => vec![3],
    vec![4, 4] => Vec::<i32>::new(),
    vec![4] => Vec::<i32>::new(),
    vec![2, 4, 1, 4, 3, 2] => vec![3, 2],
    vec![4, 1, 4, 2, 2, 2] => vec![2, 2, 2],
    vec![3, 4, 3, 2] => vec![3, 2]);

    println!();

    printbat!(notAlone,
    vec![1, 2, 3], 2 => vec![1, 3, 3],
    vec![1, 2, 3, 2, 5, 2], 2 => vec![1, 3, 3, 5, 5, 2],
    vec![3, 4], 3 => vec![3, 4],
    vec![3, 3], 3 => vec![3, 3],
    vec![1, 3, 1, 2], 1 => vec![1, 3, 3, 2],
    vec![3], 3 => vec![3],
    Vec::<i32>::new(), 3 => Vec::<i32>::new(),
    vec![7, 1, 6], 1 => vec![7, 7, 6],
    vec![1, 1, 1], 1 => vec![1, 1, 1],
    vec![1, 1, 1, 2], 1 => vec![1, 1, 1, 2]);
//
    println!();

    printbat!(zeroFront,
    vec![1, 0, 0, 1] => vec![0, 0, 1, 1],
    vec![0, 1, 1, 0, 1] => vec![0, 0, 1, 1, 1],
    vec![1, 0] => vec![0, 1],
    vec![0, 1] => vec![0, 1],
    vec![1, 1, 1, 0] => vec![0, 1, 1, 1],
    vec![2, 2, 2, 2] => vec![2, 2, 2, 2],
    vec![0, 0, 1, 0] => vec![0, 0, 0, 1],
    vec![-1, 0, 0, -1, 0] => vec![0, 0, 0, -1, -1],
    vec![0, -3, 0, -3] => vec![0, 0, -3, -3],
    Vec::<i32>::new() => Vec::<i32>::new(),
    vec![9, 9, 0, 9, 0, 9] => vec![0, 0, 9, 9, 9, 9]);

    println!();

    printbat!(withoutTen,
    vec![1, 10, 10, 2] => vec![1, 2, 0, 0],
    vec![10, 2, 10] => vec![2, 0, 0],
    vec![1, 99, 10] => vec![1, 99, 0],
    vec![10, 13, 10, 14] => vec![13, 14, 0, 0],
    vec![10, 13, 10, 14, 10] => vec![13, 14, 0, 0, 0],
    vec![10, 10, 3] => vec![3, 0, 0],
    vec![1] => vec![1],
    vec![13, 1] => vec![13, 1],
    vec![10] => vec![0],
    Vec::<i32>::new() => Vec::<i32>::new());

    println!();

    printbat!(zeroMax,
    vec![0, 5, 0, 3] => vec![5, 5, 3, 3],
    vec![0, 4, 0, 3] => vec![3, 4, 3, 3],
    vec![0, 1, 0] => vec![1, 1, 0],
    vec![0, 1, 5] => vec![5, 1, 5],
    vec![0, 2, 0] => vec![0, 2, 0],
    vec![1] => vec![1],
    vec![0] => vec![0],
    Vec::<i32>::new() => Vec::<i32>::new(),
    vec![7, 0, 4, 3, 0, 2] => vec![7, 3, 4, 3, 0, 2],
    vec![7, 0, 4, 3, 0, 1] => vec![7, 3, 4, 3, 1, 1],
    vec![7, 0, 4, 3, 0, 0] => vec![7, 3, 4, 3, 0, 0],
    vec![7, 0, 1, 0, 0, 7] => vec![7, 7, 1, 7, 7, 7]);

    println!();

    printbat!(evenOdd,
    vec![1, 0, 1, 0, 0, 1, 1] => vec![0, 0, 0, 1, 1, 1, 1],
    vec![3, 3, 2] => vec![2, 3, 3],
    vec![2, 2, 2] => vec![2, 2, 2],
    vec![3, 2, 2] => vec![2, 2, 3],
    vec![1, 1, 0, 1, 0] => vec![0, 0, 1, 1, 1],
    vec![1] => vec![1],
    vec![1, 2] => vec![2, 1],
    vec![2, 1] => vec![2, 1],
    Vec::<i32>::new() => Vec::<i32>::new());

    println!();

    printbat!(fizzBuzz,
    1, 6 => vec!["1", "2", "Fizz", "4", "Buzz"],
    1, 8 => vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7"],
    1, 11 => vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz"],
    1, 16 => vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"],
    1, 4 => vec!["1", "2", "Fizz"],
    1, 2 => vec!["1"],
    50, 56 => vec!["Buzz", "Fizz", "52", "53", "Fizz", "Buzz"],
    15, 17 => vec!["FizzBuzz", "16"],
    30, 36 => vec!["FizzBuzz", "31", "32", "Fizz", "34", "Buzz"],
    1000, 1006 => vec!["Buzz", "1001", "Fizz", "1003", "1004", "FizzBuzz"],
    99, 102 => vec!["Fizz", "Buzz", "101"],
    14, 20 => vec!["14", "FizzBuzz", "16", "17", "Fizz", "19"]);

}
