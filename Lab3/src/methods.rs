use std::collections::HashMap;

struct Function {
    func: Box<dyn Fn()>,
    arg_count: usize,
}
fn m1(x: i32) {
    println!("{}", x);
}

fn m2(x: i32, y: i32) {
    println!("{} {}", x, y);
}

fn m3(x: i32, y: i32, z: i32) {
    println!("{} {} {}", x, y, z);
}

pub fn methods() -> Vec<(String, usize)>{
    let mut iface: HashMap<&str, Function> = HashMap::new();
    iface.insert(
        "m1",
        Function {
            func: Box::new(|| m1(0)),
            arg_count: 1,
        },
    );
    iface.insert(
        "m2",
        Function {
            func: Box::new(|| m2(0, 0)),
            arg_count: 2,
        },
    );
    iface.insert(
        "m3",
        Function {
            func: Box::new(|| m3(0, 0, 0)),
            arg_count: 3,
        },
    );
    let mut result: Vec<(String, usize)> = Vec::new();
    for (name, info) in iface {
        result.push((name.to_string(), info.arg_count));
    }
    result
}