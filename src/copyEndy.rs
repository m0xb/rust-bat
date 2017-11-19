pub fn copyEndy(list: Vec<i32>) -> Vec<i32> {
    let mut endy = vec![];
    for i in list {
        if (i >= 0 && i <= 10) || (i >= 90 && i <= 100) {
            endy.push(i);
        }
    }
    return endy;
}


fn main() {
    println!("{:?}", copyEndy(vec![10, 7, 90, 99, 21, 14, 48, 99, 9, 5]));
}



#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_copyEndy() {
        assert_eq!(vec![10, 7, 90, 99, 99, 9, 5], copyEndy(vec![10, 7, 90, 99, 21, 14, 48, 99, 9, 5]));
        assert_eq!(vec![5, 4, 3, 2, 1, 0], copyEndy(vec![-5, 5, -4, 4, -3, 3, -2, 2, -1, 1, 0]));
        assert_eq!(vec![], copyEndy(vec![]));
        assert_eq!(vec![], copyEndy(vec![14, 15, -16, -17]));
    }
}

        

