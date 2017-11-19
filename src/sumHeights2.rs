fn sumHeights2(h_list: Vec<i32>, left: i32, right: i32) -> i32 {
    let mut heights = vec![];
    heights.push(left);
    for i in h_list {
        heights.push(i);
    }
    heights.push(right);
    let mut delta = 0;
    for i in 1..heights.len() {
        if heights[i] > heights[i-1] {
            delta += 2*(heights[i] - heights[i - 1]);
        } else {
            delta += heights[i] - heights[i-1];
        }
    }
    return delta;
}

fn main() {
    println!("{}", sumHeights2(vec![5, 3, 6, 7, 2], 0, 4));
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sumHeights2() {
        assert_eq!(16, sumHeights2(vec![5, 3, 6, 7, 2], 0, 4));
    }
}


