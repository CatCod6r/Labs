pub struct Contact {
    pub name: String,
    pub phone: String,
}

pub fn find_phone_by_name(phonebook: Box<[Contact]>, name: &str) -> String {
    for contact in phonebook {
        if contact.name == name {
            return contact.phone;
        }
    }
    "Not found".to_string()
}
