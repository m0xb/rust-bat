fn doubling(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().map(|x| 2 * x).collect()
}

fn square(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().map(|x| x * x).collect()
}

//This seems like it could be done with one line of code, though I'm not sure how.
fn addStar(list: Vec<&str>) -> Vec<String> {
    //&str, from what I can tell, doesn't implement the Add trait the same way String does (where you can add a String and a &str).
    //Below is a kludged together solution to get around this made by looking at compile errors.
    let s_list: Vec<String> = list.into_iter().map(|x| x.to_owned()).collect();
    let star_list = s_list.into_iter().map(|x| x + &"*").collect();
    return star_list
}

fn copies3(list: Vec<&str>) -> Vec<String> {
    let s_list: Vec<String> = list.into_iter().map(|x| x.to_owned()).collect();
    let copy_list = s_list.into_iter().map(|x| x.clone() + &x + &x).collect();
    return copy_list
}

fn moreY(list: Vec<&str>) -> Vec<String> {
    let s_list: Vec<String> = list.into_iter().map(|x| x.to_owned()).collect();
    let y_list = s_list.into_iter().map(|x| "y".to_string() + &x.clone() + &"y").collect();
    return y_list
}

fn math1(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().map(|x| (x + 1)*10).collect();
}

fn rightDigit(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().map(|x| x % 10).collect();
}

fn lower(list: Vec<&str>) -> Vec<String> {
    let s_list: Vec<String> = list.into_iter().map(|x| x.to_owned()).collect();
    return s_list.into_iter().map(|x| x.to_lowercase()).collect();
}

fn noX(list: Vec<&str>) -> Vec<String> {
    let s_list: Vec<String> = list.into_iter().map(|x| x.to_owned()).collect();
    return s_list.into_iter().map(|x| x.replace("x", "")).collect()
}




fn main() {
    let doubling_tests = vec![
    vec![1, 2, 3],
    vec![6, 8, 6, 8, -1],
    vec![],
    vec![5],
    vec![5, 10],
    vec![8, -5, 7 ,3, 109],
    vec![6, -3 , 12, 23, 4, 1 ,19, 11, 2, 3, 2],
    vec![3, 1, 4, 1, 5, 9]];
    for i in doubling_tests {
        println!("doubling: {:?}", doubling(i));
    }

    println!();

    let square_tests = vec![
    vec![1, 2, 3],
    vec![6, 8, -6, -8, 1],
    vec![],
    vec![12],
    vec![5, 10],
    vec![6, -3, 12, 23, 4, 1, 19, 11, 2, 3, 2]];
    for i in square_tests {
        println!("square: {:?}", square(i));
    }

    println!();

    let addStar_tests = vec![
        vec!["a", "bb", "ccc"],
        vec!["hello", "there"],
        vec!["*"],
        vec![],
        vec!["kittens", "and", "chocolate", "and"],
        vec!["empty", "string", "*"]];
    for i in addStar_tests {
        println!("addStar: {:?}", addStar(i));
    }

    println!();

    let copies3_tests = vec![
    vec!["a", "bb", "ccc"],
    vec!["24", "a", ""],
    vec!["hello", "there"],
    vec!["no"],
    vec![],
    vec!["this", "and", "that", "and"]];
    for i in copies3_tests {
        println!("copies3: {:?}", copies3(i));
    }

    let moreY_tests = vec![
        vec!["a", "b", "c"],
        vec!["hello", "there"],
        vec!["yay"],
        vec!["", "a", "xx"],
        vec![],
        vec!["xx", "yy", "zz"]];
    for i in moreY_tests {
        println!("moreY: {:?}", moreY(i));
    }

    println!();

    let math1_tests = vec![
        vec![1, 2, 3],
        vec![6, 8, 6, 8, 1],
        vec![10],
        vec![],
        vec![5, 10],
        vec![-1, -2, -3, -2, -1],
        vec![6, -3, 12, 23, 4, 1, 19, 11, 2, 3, 2]];
    for i in math1_tests {
        println!("math1: {:?}", math1(i));
    }

    println!();

    let rightDigit_tests = vec![
        vec![1, 22, 93],
        vec![16, 8, 886, 8, 1],
        vec![10, 0],
        vec![],
        vec![5, 10],
        vec![5, 50, 500, 5000, 5000],
        vec![6, 23, 12, 23, 4, 1, 19, 1119, 2, 3, 2]];
    for i in rightDigit_tests {
        println!("rightDigit: {:?}", rightDigit(i));
    }

    println!();

    let lower_tests = vec![
        vec!["Hello", "Hi"],
        vec!["AAA", "BBB", "ccc"],
        vec!["KitteN", "ChocolaTE"],
        vec![],
        vec!["EMPTY", ""],
        vec!["aaX", "bYb", "Ycc", "ZZZ"]];
    for i in lower_tests {
        println!("lower: {:?}", lower(i));
    }

    println!();

    let noX_tests = vec![
        vec!["ax", "bb", "cx"],
        vec!["xxax", "xbxbx", "xxcx"],
        vec!["x"],
        vec![""],
        vec![],
        vec!["the", "taxi"],
        vec!["the", "xxtaxixxx"],
        vec!["this", "is", "the", "x"]];
    for i in noX_tests {
        println!("noX: {:?}", noX(i));
    }


}