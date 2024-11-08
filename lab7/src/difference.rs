pub fn difference<T: PartialEq>(vec: Vec<T>, vec_to_diff: Vec<T>) -> Vec<T>{
    let mut return_val = vec![];
    for element in vec{
        if let None = vec_to_diff.iter().position(|x| *x == element){
            return_val.push(element);
        }
    }
    return_val
}