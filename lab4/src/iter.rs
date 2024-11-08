pub fn max(multi_dim_matrix: [[i32; 3]; 3]) -> i32{
    let mut biggest_element = 0;
    for matrix in multi_dim_matrix{
        for element in matrix{
            if element > biggest_element{
                biggest_element = element
            }
        }
    }
    biggest_element
}