#[derive(Debug)]
pub struct User {
    name: String,
    city: String,
}
pub fn create_user(name: &str, city: &str) -> User {
    User {
        name: name.to_string(),
        city: city.to_string(),
    }
}
