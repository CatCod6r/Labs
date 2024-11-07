pub fn rangeodd(start_num: i32, end_num: i32) -> Vec<i32>{
    if start_num > end_num{
        return vec![];
    }
    let mut vec: Vec<i32> = vec![];
    for x in start_num..end_num {
        if (x % 2 != 0){
            vec.push(x);
        }
    }
    vec
}