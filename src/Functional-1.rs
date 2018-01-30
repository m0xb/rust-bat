fn doubling(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().map(|x| 2 * x).collect()
}

fn square(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().map(|x| x * x).collect()
}

fn addStar(list: Vec<&str>) -> Vec<String> {
    return list.into_iter().map(|x| x.to_owned() + "*").collect();
}

fn copies3(list: Vec<&str>) -> Vec<String> {
    list.into_iter().map(|x| x.to_owned() + x + x).collect()
}

fn moreY(list: Vec<&str>) -> Vec<String> {
    list.into_iter().map(|x| "y".to_string() + x + "y").collect()
}

fn math1(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().map(|x| (x + 1)*10).collect();
}

fn rightDigit(list: Vec<i32>) -> Vec<i32> {
    return list.iter().map(|x| x % 10).collect();
}

fn lower(list: Vec<&str>) -> Vec<String> {
     list.iter().map(|x| x.to_lowercase()).collect()
}

fn noX(list: Vec<&str>) -> Vec<String> {
    list.into_iter().map(|x| x.replace("x", "")).collect()
}

#[macro_use]
mod bat_formatter;
fn main() {
    printbat!(doubling,
    vec![1, 2, 3] => vec![2, 4, 6],
    vec![6, 8, 6, 8, -1] => vec![12, 16, 12, 16, -2],
    vec![] => vec![],
    vec![5] => vec![10],
    vec![5, 10] => vec![10, 20],
    vec![8, -5, 7, 3, 109] => vec![16, -10, 14, 6, 218],
    vec![6, -3, 12, 23, 4, 1, 19, 11, 2, 3, 2] => vec![12, -6, 24, 46, 8, 2, 38, 22, 4, 6, 4],
    vec![3, 1, 4, 1, 5, 9] => vec![6, 2, 8, 2, 10, 18]);

    println!();

    printbat!(square,
    vec![1, 2, 3] => vec![1, 4, 9],
    vec![6, 8, -6, -8, 1] => vec![36, 64, 36, 64, 1],
    vec![] => vec![],
    vec![12] => vec![144],
    vec![5, 10] => vec![25, 100],
    vec![6, -3, 12, 23, 4, 1, 19, 11, 2, 3, 2] => vec![36, 9, 144, 529, 16, 1, 361, 121, 4, 9, 4]);

    println!();

    printbat!(addStar,
    vec!["a", "bb", "ccc"] => vec!["a*", "bb*", "ccc*"],
    vec!["hello", "there"] => vec!["hello*", "there*"],
    vec!["*"] => vec!["**"],
    vec![] => vec![],
    vec!["kittens", "and", "chocolate", "and"] => vec!["kittens*", "and*", "chocolate*", "and*"],
    vec!["empty", "string", ""] => vec!["empty*", "string*", "*"]);

    println!();

    printbat!(copies3,
    vec!["a", "bb", "ccc"] => vec!["aaa", "bbbbbb", "ccccccccc"],
    vec!["24", "a", ""] => vec!["242424", "aaa", ""],
    vec!["hello", "there"] => vec!["hellohellohello", "theretherethere"],
    vec!["no"] => vec!["nonono"],
    vec![] => vec![],
    vec!["this", "and", "that", "and"] => vec!["thisthisthis", "andandand", "thatthatthat", "andandand"]);

    println!();

    printbat!(moreY,
    vec!["a", "b", "c"] => vec!["yay", "yby", "ycy"],
    vec!["hello", "there"] => vec!["yhelloy", "ytherey"],
    vec!["yay"] => vec!["yyayy"],
    vec!["", "a", "xx"] => vec!["yy", "yay", "yxxy"],
    vec![] => vec![],
    vec!["xx", "yy", "zz"] => vec!["yxxy", "yyyy", "yzzy"]);

    println!();

    printbat!(math1,
    vec![1, 2, 3] => vec![20, 30, 40],
    vec![6, 8, 6, 8, 1] => vec![70, 90, 70, 90, 20],
    vec![10] => vec![110],
    vec![] => vec![],
    vec![5, 10] => vec![60, 110],
    vec![-1, -2, -3, -2, -1] => vec![0, -10, -20, -10, 0],
    vec![6, -3, 12, 23, 4, 1, 19, 11, 2, 3, 2] => vec![70, -20, 130, 240, 50, 20, 200, 120, 30, 40, 30]);

    println!();

    printbat!(rightDigit,
    vec![1, 22, 93] => vec![1, 2, 3],
    vec![16, 8, 886, 8, 1] => vec![6, 8, 6, 8, 1],
    vec![10, 0] => vec![0, 0],
    vec![] => vec![],
    vec![5, 10] => vec![5, 0],
    vec![5, 50, 500, 5000, 5000] => vec![5, 0, 0, 0, 0],
    vec![6, 23, 12, 23, 4, 1, 19, 1119, 2, 3, 2] => vec![6, 3, 2, 3, 4, 1, 9, 9, 2, 3, 2]);

    println!();

    printbat!(lower,
    vec!["Hello", "Hi"] => vec!["hello", "hi"],
    vec!["AAA", "BBB", "ccc"] => vec!["aaa", "bbb", "ccc"],
    vec!["KitteN", "ChocolaTE"] => vec!["kitten", "chocolate"],
    vec![] => vec![],
    vec!["EMPTY", ""] => vec!["empty", ""],
    vec!["aaX", "bYb", "Ycc", "ZZZ"] => vec!["aax", "byb", "ycc", "zzz"]);

    println!();

    printbat!(noX,
    vec!["ax", "bb", "cx"] => vec!["a", "bb", "c"],
    vec!["xxax", "xbxbx", "xxcx"] => vec!["a", "bb", "c"],
    vec!["x"] => vec![""],
    vec![""] => vec![""],
    vec![] => vec![],
    vec!["the", "taxi"] => vec!["the", "tai"],
    vec!["the", "xxtxaxixxx"] => vec!["the", "tai"],
    vec!["this", "is", "the", "x"] => vec!["this", "is", "the", ""]);

//    let doubling_tests = vec![
//    vec![1, 2, 3],
//    vec![6, 8, 6, 8, -1],
//    vec![],
//    vec![5],
//    vec![5, 10],
//    vec![8, -5, 7 ,3, 109],
//    vec![6, -3 , 12, 23, 4, 1 ,19, 11, 2, 3, 2],
//    vec![3, 1, 4, 1, 5, 9]];
//    for i in doubling_tests {
//        println!("doubling: {:?}", doubling(i));
//    }
//
//    println!();
//
//    let square_tests = vec![
//    vec![1, 2, 3],
//    vec![6, 8, -6, -8, 1],
//    vec![],
//    vec![12],
//    vec![5, 10],
//    vec![6, -3, 12, 23, 4, 1, 19, 11, 2, 3, 2]];
//    for i in square_tests {
//        println!("square: {:?}", square(i));
//    }
//
//    println!();
//
//    let addStar_tests = vec![
//        vec!["a", "bb", "ccc"],
//        vec!["hello", "there"],
//        vec!["*"],
//        vec![],
//        vec!["kittens", "and", "chocolate", "and"],
//        vec!["empty", "string", "*"]];
//    for i in addStar_tests {
//        println!("addStar: {:?}", addStar(i));
//    }
//
//    println!();
//
//    let copies3_tests = vec![
//    vec!["a", "bb", "ccc"],
//    vec!["24", "a", ""],
//    vec!["hello", "there"],
//    vec!["no"],
//    vec![],
//    vec!["this", "and", "that", "and"]];
//    for i in copies3_tests {
//        println!("copies3: {:?}", copies3(i));
//    }
//
//    println!();
//
//    let moreY_tests = vec![
//        vec!["a", "b", "c"],
//        vec!["hello", "there"],
//        vec!["yay"],
//        vec!["", "a", "xx"],
//        vec![],
//        vec!["xx", "yy", "zz"]];
//    for i in moreY_tests {
//        println!("moreY: {:?}", moreY(i));
//    }
//
//    println!();
//
//    let math1_tests = vec![
//        vec![1, 2, 3],
//        vec![6, 8, 6, 8, 1],
//        vec![10],
//        vec![],
//        vec![5, 10],
//        vec![-1, -2, -3, -2, -1],
//        vec![6, -3, 12, 23, 4, 1, 19, 11, 2, 3, 2]];
//    for i in math1_tests {
//        println!("math1: {:?}", math1(i));
//    }
//
//    println!();
//
//    let rightDigit_tests = vec![
//        vec![1, 22, 93],
//        vec![16, 8, 886, 8, 1],
//        vec![10, 0],
//        vec![],
//        vec![5, 10],
//        vec![5, 50, 500, 5000, 5000],
//        vec![6, 23, 12, 23, 4, 1, 19, 1119, 2, 3, 2]];
//    for i in rightDigit_tests {
//        println!("rightDigit: {:?}", rightDigit(i));
//    }
//
//    println!();
//
//    let lower_tests = vec![
//        vec!["Hello", "Hi"],
//        vec!["AAA", "BBB", "ccc"],
//        vec!["KitteN", "ChocolaTE"],
//        vec![],
//        vec!["EMPTY", ""],
//        vec!["aaX", "bYb", "Ycc", "ZZZ"]];
//    for i in lower_tests {
//        println!("lower: {:?}", lower(i));
//    }
//
//    println!();
//
//    let noX_tests = vec![
//        vec!["ax", "bb", "cx"],
//        vec!["xxax", "xbxbx", "xxcx"],
//        vec!["x"],
//        vec![""],
//        vec![],
//        vec!["the", "taxi"],
//        vec!["the", "xxtaxixxx"],
//        vec!["this", "is", "the", "x"]];
//    for i in noX_tests {
//        println!("noX: {:?}", noX(i));
//    }


}