use crate::sums::{reduce, sum_do_while, sum_for, sum_for_in, sum_while};
use multiarray::*;
use crate::iter::max;
use crate::objects_iter::{BornDied, get_time_lived, Person};

mod sums;
mod iter;
mod objects_iter;

fn main() {
    let arr = [1,2,3,4,5,6,7,8,9,0];
    println!("{}", sum_for(arr));
    println!("{}", sum_for_in(arr));
    println!("{}", sum_while(arr));
    println!("{}", sum_do_while(arr));
    println!("{}", reduce(arr));

    let matrix: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    println!("{}", max(matrix));

    let people = [
        Person{name: "lenin".to_string(), born_die:BornDied{born: 1870, died: 1924}},
        Person{name: "mao".to_string(), born_die:BornDied{born: 1893, died: 1976}},
        Person{name: "gandhi".to_string(), born_die:BornDied{born: 1869, died: 1948}},
        Person{name: "hirohito".to_string(), born_die:BornDied{born: 1901, died: 1989}}
        ];
    println!("{:?}", get_time_lived(&people));
}