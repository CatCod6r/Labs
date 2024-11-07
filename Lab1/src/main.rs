use std::collections::HashMap;

fn main() {
    println!("inc function with number :{}", inc(5));
    let num_obj = Num{num: 5};
    println!("inc function where Num is object :{}", inc2(num_obj));
    //Creating vec with different values with enum because rust dowsnt have that feature :(
    let diff_vals = vec![
        DifferentVals::Bool(true),
        DifferentVals::Int(54),
        DifferentVals::Text("Marcus Avrelius".to_string()),
        DifferentVals::Int(8),
        DifferentVals::Bool(true),
        DifferentVals::Text("Meditations".to_string())
    ];

    for entry in get_entries(diff_vals){
        println!("{}", entry.0);
        println!("{}", entry.1);
    }
}
fn inc(num: u8) -> u8{
    num - 1
}
fn inc2(num_struct: Num) -> u8{
    num_struct.num - 1
}
fn get_entries(vec: Vec<DifferentVals>) -> HashMap<&'static str, u32> {
    let mut entries = HashMap::new();
    for entry in vec{
        match entry {
            DifferentVals::Int(..) => {
               match entries.get(&"Number") {
                   Some(count) => { entries.insert("Number", count + 1); }
                   None => { entries.insert("Number", 1); }
               }
            }
            DifferentVals::Text(..) => {
               match entries.get(&"String") {
                   Some(count) => { entries.insert("String", count + 1); }
                   None => { entries.insert("String", 1); }
               }
            }
            DifferentVals::Bool(..) => {
               match entries.get(&"Boolean") {
                   Some(count) => { entries.insert("Boolean", count + 1); }
                   None => { entries.insert("Boolean", 1); }
               }
            }
            _ => println!("None of the values matched")
        }
    }
    entries
}
struct Num{
    num: u8
}
enum DifferentVals {
    Int(i32),
    Text(String),
    Bool(bool)
}