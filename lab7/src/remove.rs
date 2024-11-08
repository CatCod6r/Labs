pub fn remove<T: PartialEq>(mut array: &mut Vec<T>, item: T){
    if let Some(pos) = array.iter().position(|x| *x == item) {
        array.remove(pos);
    }
}
pub fn remove_elements<T: PartialEq>(vec: &mut Vec<T>, items: Vec<T>) {
    for item in items {
        remove(vec, item);
    }
}