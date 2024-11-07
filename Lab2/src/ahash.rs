use std::collections::HashMap;

pub fn find_phone_by_name<'a>(phonebook: &'a HashMap<&'a str, &'a str>, name: &'a str) -> Option<&'a str> {
    phonebook.get(name).copied()
}