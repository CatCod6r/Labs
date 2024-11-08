// for with index(no normal for loop in rust)
pub fn sum_for(args: [i32; 10]) -> i32 {
    let mut result = 0;
    for i in 0..args.len() {
        result += args[i];
    }
    result
}
// for..in
pub fn sum_for_in(args: [i32; 10]) -> i32 {
    let mut result = 0;
    for num in args {
        result += num;
    }
    result
}
// while
pub fn sum_while(args: [i32; 10]) -> i32 {
    let mut result = 0;
    let mut i = 0;
    while i < args.len() {
        result += args[i];
        i += 1;
    }
    result
}
// do..while (cant do that in Rust,
// so make realization with loop and break)
pub fn sum_do_while(args: [i32; 10]) -> i32 {
    let mut result = 0;
    let mut i = 0;
    loop {
        if i >= args.len() {
            break;
        }
        result += args[i];
        i += 1;
    }
    result
}
//No reduce method but have iter().fold() which works same way
pub fn reduce(args: [i32; 10]) -> i32{
    args.iter().fold(0, |acc, &x| acc + x)
}
