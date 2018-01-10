use std::cmp;
fn evenlySpaced(a: i32, b: i32, c: i32) -> bool {
    let max = cmp::max(a, cmp::max(b, c)) as f32;
    let min = cmp::min(a, cmp::min(b, c)) as f32;
    let x: Vec<f32> = vec![a as f32, b as f32 , c as f32];
    return x.contains(&((min + max) / 2.0))
}

fn evenlySpaced2(nums: Vec<i32>, start: usize) -> bool {
    if nums.len() <= 2 {
        return true
    }
    if start == nums.len() - 1 {
        return true
    }
    if (nums[start - 1] + nums[start + 1])/2 == nums[start] {
        return evenlySpaced2(nums, start + 1)
    }
    return false
}

fn main() {
    let evenlySpaced_tests = vec![
        (2, 4, 6),
        (4, 6, 2),
        (4, 6, 3),
        (6, 2, 4),
        (6, 2, 8),
        (2, 2, 2),
        (2, 2, 3),
        (9, 10, 11),
        (10, 9, 11),
        (10, 9, 9),
        (2, 4, 4),
        (2, 2, 4),
        (3, 6, 12),
        (12, 3, 6)];
    for i in evenlySpaced_tests {
        println!("evenlySpaced: {:?} -> {:?}", i, evenlySpaced(i.0 , i.1, i.2))
    }

    println!();

    let evenlySpaced2_tests = vec![
        (vec![1, 2, 4, 4, 5, 6], 1),
        (vec![2, 3, 4, 5, 6, 7], 1),
        (vec![1, 3, 5, 7, 9, 11], 1),
        (vec![], 1),
        (vec![1, 100], 1),
        (vec![-1, 0, 1, 2, 3, 4], 1),
        (vec![50, 40, 30, 20, 10], 1),
        (vec![0], 1),
        //This should pass.  However, I think I'd need a helper method to first sort the array into ascending/descending order.
        //Might be able to use backtracking.
        (vec![3, 1, 2, 4, 7, 5, 6], 1)];
    for i in evenlySpaced2_tests {
        println!("evenlySpaced2: {}", evenlySpaced2(i.0, i.1));
    }
}



