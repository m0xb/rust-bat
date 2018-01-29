use std::cmp::max;
fn groupSum(start: usize, nums: &Vec<i32>, target: i32) -> bool {
    if target == 0 {
        return true
    }
    if start >= nums.len() {
        return false
    }
    if groupSum(start + 1, nums, target - nums[start]) {
        return true}
    if groupSum(start + 1, nums, target) {
        return true}
    return false

}

fn groupSum6(start: usize, nums: &Vec<i32>, target: i32) -> bool {
    if target == 0 && start == nums.len() {
        return true
    }
    if start >= nums.len() {
        return false
    }
    if groupSum6(start + 1, nums, target - nums[start]) {
        return true
    }
    if nums[start] == 6  {
        if groupSum6(start + 1, nums, target - 6) {
            return true
        }
    } else if groupSum6(start + 1, nums, target) {
        return true
    }
    return false
}

fn groupNoAdj(start: usize, nums: &Vec<i32>, target: i32) -> bool {
    if target == 0 {
        return true
    }
    if start >= nums.len() {
        return false
    }
    if groupNoAdj(start + 2, nums, target - nums[start]) {
        return true
    }
    if groupNoAdj(start + 1, nums, target) {
        return true
    }
    return false
}

//Very similar to groupSum6
fn groupSum5(start: usize, nums: &Vec<i32>, target: i32) -> bool {
    if start == nums.len() && target == 0 {
        return true
    }
    if start >= nums.len() {
        return false
    }
    if nums[start] % 5 == 0 {
        if start + 2 <= nums.len() && nums[start + 1] == 1 {
            return groupSum5(start + 2, nums, target - nums[start])
        }
        return groupSum5(start + 1, nums, target - nums[start])

    }
    if groupSum5(start + 1, nums, target - nums[start]) {
        return true
    }
    if groupSum5(start + 1, nums, target) {
        return true
    }
    return false
}
//Use of a loop is allowed per the conditions of the bat.
fn groupSumClump(start: usize, nums: &Vec<i32>, target: i32) -> bool {
    if target == 0 && start == nums.len() {
        return true
    }
    if start >= nums.len() {
        return false
    }
    let mut sum = 0;
    let mut i = start;
    while i < nums.len() && nums[start] == nums[i] {
        sum += nums[i];
        i += 1;
    }
    //Note, i = start + 1, given the conditions of the while loop.
    if groupSumClump(i , nums, target - sum) {
        return true
    }
    //if groupSumClump(i, nums, target - nums[i - 1]) {
    //    return true
    //}
    if groupSumClump(i, nums, target) {
        return true
    }
    return false
}

fn splitArray(nums: &Vec<i32>) -> bool {
//This function needs to return true if l_sum == r_sum.  I need a way to bi-directionally sum the array and backtrack over possible positions
// for the split point between the two sums (index).
//I'll start with returning sA_Helper with index set to 0, and l_sum and r_sum set to to.  I'll make sA_Helper recursive, but not splitArray.
//sA_Helper returns a bool, so I can call it in the return statement.
    return sA_Helper(nums, 0, 0, 0)
}

fn sA_Helper(nums: &Vec<i32>, index: usize, l_sum: i32, r_sum: i32) -> bool {
    if index >= nums.len() {
        if l_sum == r_sum {
            return true
        }
        return false
    }
    //Increment l_sum by nums[index], check if it returns true via the base condition.
    if sA_Helper(nums, index + 1, l_sum + nums[index], r_sum) {
        return true
    }
    //Increment r_sum by nums[index], check if it returns true via the base condition.
    if sA_Helper(nums, index + 1, l_sum, r_sum + nums[index]) {
        return true
    }
    //
    return false
}

//I more or less plan on trying to duplicate my strategy with splitArray here, but with the added constraints.
//The condition for passing is that all elements can be successfully placed into two groups.  The first group must sum to a multiple of 10,
//the second group must sum to an odd number.
fn splitOdd10(nums: &Vec<i32>) -> bool {
    //Again, my helper function returns a bool, so we can just return it.
    return sO10_Helper(nums, 0, 0, 0)
}

fn sO10_Helper(nums: &Vec<i32>, ten_sum: i32, odd_sum: i32, index: usize) -> bool {
    //Our base (accessible only when index == nums.len(), so the whole array is considered) condition to return true
    // is going to be two criteria: ten_sum % 10 == 0, and odd_sum % 2 == 1
    if index >= nums.len() {
        if ten_sum % 10 == 0 && odd_sum % 2 == 1 {
            return true
        }
        return false
    }
    //Check for ten_sum
    if sO10_Helper(nums, ten_sum + nums[index], odd_sum, index + 1) {
        return true
    }
    if sO10_Helper(nums, ten_sum, odd_sum + nums[index], index + 1) {
        return true
    }
    return false
}

fn split53(nums: &Vec<i32>) -> bool {
    return s53_Helper(nums, 0, 0, 0)
}

fn s53_Helper(nums: &Vec<i32>, five_sum: i32, three_sum: i32, index: usize) -> bool {
    if index >= nums.len() {
        return five_sum == three_sum
    }
    if nums[index] % 3 == 0 {
        return s53_Helper(nums, five_sum, three_sum + nums[index], index + 1)
    }
    if nums[index] % 5 == 0 {
        return s53_Helper(nums, five_sum + nums[index], three_sum, index + 1)
    }
    return s53_Helper(nums, five_sum + nums[index], three_sum, index + 1) || s53_Helper(nums, five_sum, three_sum + nums[index], index + 1)

    /*if s53_Helper(nums, five_sum + nums[index], three_sum, index + 1) {
        return true
    }
    if s53_Helper(nums, five_sum, three_sum + nums[index], index + 1) {
        return true
    }
    return false
    */
}

#[macro_use]
mod bat_formatter;

fn main() {
    //Formatting exercise.
    let groupSum_tests = vec![
        (0, vec![2, 4, 8], 10, true),
        (0, vec![2, 4, 8], 14, true),
        (0, vec![2, 4, 8], 9, false),
        (0, vec![2, 4, 8], 8, true),
        (1, vec![2, 4, 8], 8, true),
        (1, vec![2, 4, 8], 2, false),
        (0, vec![1], 1, true),
        (0, vec![9], 1, false),
        (1, vec![9], 0, true),
        (0, vec![], 0, true),
        (0, vec![10, 2, 2, 5], 17, true),
        (0, vec![10, 2, 2, 5], 15, true),
        (0, vec![10, 2, 2, 5], 9, true)];
    let mut groupSum_rows = Vec::new();
    let mut column_widths = (3, 0, 0, 0);
    for i in groupSum_tests {
        let invocation = format!("groupSum({}, {:?}, {})", i.0, i.1, i.2);
        let expectation = i.3.to_string();
        let actual = groupSum(i.0, &i.1, i.2).to_string();
        let passed = i.3 == groupSum(i.0, &i.1, i.2);

        column_widths.1 = max(invocation.chars().count(), column_widths.1);
        column_widths.2 = max(expectation.chars().count(), column_widths.2);
        column_widths.3 = max(actual.chars().count(), column_widths.3);
        groupSum_rows.push((passed, invocation, expectation, actual));
    }
    for i in groupSum_rows {
        fn render_cell(s: &str, cwidth: usize) {
            print!("{}", s);
            let width_delta = cwidth - s.chars().count();
            let padding = " ".repeat(width_delta);
            print!("{}", padding);
        }
        let maybe_check = if i.0 {"✔"} else {""};
        if !i.0 {
            print!("\x1B[31m")
        }
        render_cell(maybe_check, column_widths.0);
        render_cell(&i.1, column_widths.1);
        print!(" ");
        render_cell(&i.2, column_widths.2);
        print!(" ");
        render_cell(&i.3, column_widths.3);
        if !i.0 {
            print!("\x1B[0m")
        }
        println!();
    }

    println!();

    let groupSum6_tests = vec![
        (0, vec![5, 6, 2], 8),
        (0, vec![5, 6, 2], 9),
        (0, vec![5, 6, 2], 7),
        (0, vec![1], 1),
        (0, vec![9], 1),
        (0, vec![], 0),
        (0, vec![3, 2, 4, 6], 8),
        (0, vec![6, 2, 4, 3], 8),
        (0, vec![5, 2 ,4, 6], 9),
        (0, vec![6, 2, 4, 5], 9),
        (0, vec![3, 2, 4, 6], 3),
        (0, vec![1, 6, 2 ,6, 4], 12),
        (0, vec![1, 6, 2 ,6, 4], 13),
        (0, vec![1, 6, 2 ,6, 4], 4),
        (0, vec![1, 6, 2 ,6, 4], 9),
        (0, vec![1, 6, 2 ,6, 5], 14),
        (0, vec![1, 6, 2 ,6, 5], 15),
        (0, vec![1, 6, 2 ,6, 5], 16),
        (0, vec![6], 0)];
    for i in groupSum6_tests {
        println!("groupSum6: {}", groupSum6(i.0, &i.1, i.2));
    }

    println!();

    let groupNoAdj_tests = vec![
        (0, vec![2, 5, 10, 4], 12),
        (0, vec![2, 5, 10, 4], 14),
        (0, vec![2, 5, 10, 4], 7),
        (0, vec![2, 5, 10, 4, 2], 7),
        (0, vec![2, 5, 10, 4], 9),
        (0, vec![10 ,2, 2, 3 ,3], 15),
        (0, vec![10, 2, 2, 3, 3], 7),
        (0, vec![], 0),
        (0, vec![1], 1),
        (0, vec![9], 1),
        (0, vec![9], 0),
        (0, vec![5, 10, 4, 1], 11)];
    for i in groupNoAdj_tests {
        println!("groupNoAdj: {}", groupNoAdj(i.0, &i.1, i.2))
    }

    println!();

    let groupSum5_tests = vec![
        (0, vec![2, 5, 10, 4], 19),
        (0, vec![2, 5, 10, 4], 17),
        (0, vec![2, 5, 10, 4], 12),
        (0, vec![2, 5, 4, 10], 12),
        (0, vec![3, 5, 1], 4),
        (0, vec![3, 5, 1], 5),
        (0, vec![1, 3, 5], 5),
        (0, vec![3, 5, 1], 9),
        (0, vec![2, 5, 10, 4], 7),
        (0, vec![2, 5, 10, 4], 15),
        (0, vec![2, 5, 10, 4], 11),
        (0, vec![1], 1),
        (0, vec![9], 1),
        (0, vec![9], 0),
        (0, vec![], 0),
        (0, vec![2, 5, 5], 7)];
    for i in groupSum5_tests {
        println!("groupSum5: {}, nums: {:?}, target: {}", groupSum5(i.0, &i.1, i.2), i.1, i.2);
    }

    println!();

    let groupSumClump_tests = vec![
        (0, vec![2, 4, 8], 10),
        (0, vec![1, 2, 4, 8, 1], 14),
        (0, vec![2, 4, 4, 8], 14),
        (0, vec![8, 2, 2, 1], 9),
        (0, vec![8, 2, 2, 1], 11),
        (0, vec![1], 1),
        (0, vec![9], 1)];
    for i in groupSumClump_tests {
        println!("groupSumClump: {}", groupSumClump(i.0, &i.1, i.2));
    }

    println!();

    let splitArray_tests = vec![
        vec![2, 2],
        vec![2, 3],
        vec![5, 2, 3],
        vec![5, 2, 2],
        vec![1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![],
        vec![1],
        vec![3, 5],
        vec![5, 3, 2],
        vec![2, 2, 10, 10, 1, 1],
        vec![1, 2, 2, 10, 10, 1, 1],
        vec![1, 2, 3, 10, 10, 1, 1]];
    for i in splitArray_tests {
        println!("splitArray: {}", splitArray(&i));
    }

    println!();

    let splitOdd10_tests = vec![
        vec![5, 5, 5],
        vec![5, 5, 6],
        vec![5, 5, 6, 1],
        vec![6, 5, 5, 1],
        vec![6, 5, 5, 1, 10],
        vec![6, 5, 5, 5, 1],
        vec![1],
        vec![],
        vec![10, 7, 5, 5],
        vec![10, 0, 5, 5],
        vec![10, 7, 5, 5, 2],
        vec![10, 7, 5, 5, 1]];
    for i in splitOdd10_tests {
        println!("splitOdd10: {}", splitOdd10(&i));
    }

    println!();

    let split53_tests = vec![
    vec![1, 1],
    vec![1, 1, 1],
    vec![2, 4, 2],
    vec![2, 2, 2, 1],
    vec![3, 3, 5, 1],
    vec![3, 5, 8],
    vec![2, 4, 6],
    vec![3, 5 ,6, 10, 3, 3]];
    for i in split53_tests {
        println!("split53: {}", split53(&i));
    }

    printbat!(groupSumClump,
        0, &vec![2, 4, 8], 10 => true,
        0, &vec![1, 2, 4, 8, 1], 14 => true,
        0, &vec![2, 4, 4, 8], 14 => false,
        0, &vec![8, 2, 2, 1], 9 => true,
        0, &vec![8, 2, 2, 1], 11 => false,
        0, &vec![1], 1 => true,
        0, &vec![9], 1 => false);


}


