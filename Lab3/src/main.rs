mod random;
mod ip;
mod generate_keys;
mod methods;

use rand::Rng;

fn main() {
    println!("{}", random::rand_from_range(0,76));
    println!("{}", random::rand_to_max(56));
    println!("{:?}", generate_keys::generate_keys(16, "abcdefghijklmnopqrstuvwxyz0123456789".to_string()));
    ip::ip(String::from("10.0.0.1"));
    for (name, arity) in methods::methods() {
        println!("[{}, {}]", name, arity);
    }
}