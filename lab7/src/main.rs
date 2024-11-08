use crate::difference::difference;
use crate::remove::{remove, remove_elements};
use crate::unique::unique;

mod remove;
mod unique;
mod difference;

fn main() {
    //Everything also works with String cuz of generic type

    // let mut array = vec!["Kiev", "Beijing", "Lima", "Saratov"];
    // remove(&mut array, "Kiev");
    // println!("{:?}", &array);

    //1
    let mut vector = vec![1,2,3,4,5,6,7,8,9];
    remove(&mut vector, 6);
    println!("{:?}", &vector);
    //2
    remove_elements(& mut vector, vec![1,2,3]);
    println!("{:?}", &vector);
    //3
    let mut vector_unique = vec![1,3,3,3,5,7,7,9];
    println!("{:?}", unique(&*vector_unique));
    //4
    let vector_for_diff = vec![1,2,3,4,5,7,8,9];
    println!("{:?}", difference(vector_for_diff,vec![1,2,3,5] ))
}

