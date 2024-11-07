struct Obj {
    name: String,
}

pub fn objects() {
    let OBJ: Obj = Obj { name: String::from("Name") };
    let mut obj2 = Obj { name: String::from("Name") };

    obj2.name = String::from("Nazar");
    obj2 = Obj { name: String::from("Nazar") };

    println!("obj1 name: {}", OBJ.name);
    println!("obj2 name: {}", obj2.name);
}
