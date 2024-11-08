pub fn unique<T: PartialEq>(array: &[T]) -> Vec<&T>{
    let mut return_arr = vec![];
    for element in array{
        if let None = return_arr.iter().position(|x| *x == element) {
            return_arr.push(element);
        }
    }
    return_arr
}