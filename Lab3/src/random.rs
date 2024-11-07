use rand::Rng;

pub fn rand_from_range(min: u32, max:u32) -> u32{
    rand::thread_rng().gen_range(min..max)
}
pub fn rand_to_max(max: u32) -> u32{
    rand::thread_rng().gen_range(0..max)
}