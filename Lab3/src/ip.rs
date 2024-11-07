pub fn ip(ip_addres: String){
    let mut char_array = ip_addres.split(".");
    let mut num_array: Vec<_> = vec![];
    for num in char_array{
        num_array.push(num.parse::<u32>().unwrap());
    }
    println!("{:?}", num_array);
    num_array[0] = num_array[0] << 8 << 8 << 8;
    num_array[1] = num_array[1] << 8 << 8;
    num_array[2] = num_array[2] << 8;
    let sum:u32 = num_array.iter().sum();
    println!("{}",sum)
}