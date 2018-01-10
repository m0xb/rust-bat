fn noNeg(list: Vec<i32>) -> Vec<i32> {
   return list.into_iter().filter(|&x| !x.is_negative()).collect()
}

fn no9(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().filter(|x| x % 10 != 9).collect()
}

fn noTeen(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().filter(|x| *x < 13 || *x > 19).collect()
}

//If I wanted this to return a Vec<String>, how would my code be different?
fn noLong(list: Vec<&str>) -> Vec<&str> {
    return list.into_iter().filter(|x| x.len() < 4).collect()
}

fn noZ(list: Vec<&str>) -> Vec<&str> {
    return list.into_iter().filter(|x| !x.contains("z")).collect()
}

fn no34(list: Vec<&str>) -> Vec<&str> {
    return list.into_iter().filter(|x| x.len() > 4 || x.len() < 3).collect()
}

fn noYY(list: Vec<&str>) -> Vec<String> {
    let y_list: Vec<String> = list.into_iter().map(|x| x.to_owned() + "y").collect();
    return y_list.into_iter().filter(|x| !x.contains("yy")).collect()
}

fn two2(mut list: Vec<i32>) -> Vec<i32> {
    list = list.iter().map(|x| x*2).collect();
    return list.into_iter().filter(|x| x % 10 != 2).collect();
}

fn square56(mut list: Vec<i32>) -> Vec<i32> {
    list = list.iter().map(|x| x*x + 10).collect();
    return list.into_iter().filter(|x| x % 10 != 5 && x % 10 != 6).collect()
}


fn main() {
    let noNeg_tests = vec![
        (vec![1, -2], vec![1]),
        (vec![-3, -3, 3, 3], vec![3, 3]),
        (vec![-1, -1, -1], vec![]),
        (vec![], vec![]),
        (vec![0, 1, 2], vec![0, 1, 2]),
        (vec![3, -10, 1, -1, 4, -400], vec![3, 1, 4]),
        (vec![-1, 3, 1, -1, -10, -100, -111, 5], vec![3, 1, 5])];
    for i in noNeg_tests {
        println!("noNeg: {:?} -> {:?}", i.1, noNeg(i.0));
    }

    println!();

    let no9_tests = vec![
        (vec![1, 2, 19], vec![1, 2]),
        (vec![9, 19, 29, 3], vec![3]),
        (vec![1, 2, 3], vec![1, 2, 3]),
        (vec![45, 99, 90, 28, 13, 999, 0], vec![45, 90, 28, 13, 0]),
        (vec![], vec![]),
        (vec![9], vec![]),
        (vec![0, 9, 0], vec![0, 0])];
    for i in no9_tests {
        println!("no9: {:?} -> {:?}", i.1, no9(i.0));
    }

    println!();

    let noTeen_tests = vec![
        (vec![12, 13, 19, 20], vec![12, 20]),
        (vec![1, 14, 1], vec![1, 1]),
        (vec![15], vec![]),
        (vec![-15], vec![-15]),
        (vec![], vec![]),
        (vec![0, 1, 2, -3], vec![0, 1, 2, -3]),
        (vec![15, 17, 19, 21, 19], vec![21]),
        (vec![-16, 2, 15, 3, 16, 25], vec![-16, 2, 3, 25])];
    for i in noTeen_tests {
        println!("noTeen: {:?} -> {:?}", i.1, noTeen(i.0));
    }

    println!();

    let noLong_tests = vec![
        (vec!["this", "not", "too", "long"], vec!["not", "too"]),
        (vec!["a", "bbb", "cccc"],vec!["a", "bbb"]),
        (vec!["cccc", "cccc", "cccc"], vec![]),
        (vec![], vec![]),
        (vec![""], vec![""]),
        (vec!["empty", "", "empty"], vec![""]),
        (vec!["a"], vec!["a"]),
        (vec!["aaaa", "bbb", "***", "333"], vec!["bbb", "***", "333"])];
    for i in noLong_tests {
        println!("noLong: {:?} -> {:?}", i.1, noLong(i.0));
    }

    println!();

    let noZ_tests = vec![
        (vec!["aaa", "bbb", "aza"], vec!["aaa", "bbb"]),
        (vec!["hziz", "hzello", "hi"], vec!["hi"]),
        (vec!["hello", "howz", "are", "youz"], vec!["hello", "are"]),
        (vec![], vec![]),
        (vec![""], vec![""]),
        (vec!["x", "y", "z"], vec!["x", "y"])];
    for i in noZ_tests {
        println!("noZ: {:?} -> {:?}", i.1, noZ(i.0));
    }

    println!();

    let no34_tests = vec![
        vec!["a", "bb", "ccc"],
        vec!["a", "bb", "ccc", "dddd"],
        vec!["ccc", "dddd", "apple"],
        vec!["this", "not", "too", "long"],
        vec!["a", "bbb", "cccc", "xx"],
        vec!["dddd", "ddd", "xxxxxxx"],
        vec![],
        vec![""],
        vec!["empty", "", "empty"],
        vec!["a"],
        vec!["aaaa", "bbb", "*****", "333"]];
    for i in no34_tests {
        println!("no34: {:?}", no34(i));
    }

    println!();

    let noYY_tests = vec![
        vec!["a", "b", "c"],
        vec!["a", "b", "cy"],
        vec!["xx", "ya", "zz"],
        vec!["xx", "yay", "zz"],
        vec!["yyx", "y", "zzz"],
        vec!["yyx", "y", "zzz"],
        vec!["hello", "there"],
        vec!["ya"],
        vec![],
        vec![""],
        vec!["xx", "yy", "zz"]];
    for i in noYY_tests {
        println!("noYY: {:?}", noYY(i));
    }

    println!();

    let two2_tests =  vec![
        vec![1, 2, 3],
        vec![2, 6, 11],
        vec![0],
        vec![],
        vec![1, 11, 111, 16],
        vec![2, 3, 5, 7, 11],
        vec![3, 1, 4, 1, 6, 99, 0]];
    for i in two2_tests {
        println!("two2: {:?}", two2(i));
    }

    println!();

    let square56_tests = vec![
        vec![3, 1, 4],
        vec![1],
        vec![2],
        vec![3],
        vec![4],
        vec![5],
        vec![6],
        vec![7],
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![3, -1, -4, 1, 4, 5, 9]];
    for i in square56_tests {
        println!("square56: {:?}", square56(i));
    }

}