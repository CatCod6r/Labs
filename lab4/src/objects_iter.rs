use std::collections::HashMap;

pub struct BornDied {
    pub born: u16,
    pub died: u16
}
pub struct Person{
    pub name: String,
    pub born_die: BornDied
}
pub fn get_time_lived(people: &[Person]) -> HashMap<&String, u16> {
    let mut people_to_ret = HashMap::new();
    for person in people{
        people_to_ret.insert(&person.name, person.born_die.died - person.born_die.born);
    }
    people_to_ret
}