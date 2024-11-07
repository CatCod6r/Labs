use rand::Rng;

pub fn generate_keys(range: u32, charachters: String) -> Vec<char>{
    let mut ready_key = vec![];
    for _ in 0..range {
        let randnum: u8 = rand::thread_rng().gen_range(0..36);
        ready_key.push(charachters.chars().nth(randnum as usize).unwrap());
    }
    ready_key
}