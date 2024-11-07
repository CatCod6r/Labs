// - Implement function `average` with signature
// `average(a: number, b: number): number`
// calculating average (arithmetic mean).
pub fn average(a: u32,b: u32) -> u32{
    (a + b)/2
}
// - Implement function `square` with signature
// `square(x: number): number` calculating square of x.
fn square (x: u32) -> u32{
    x*x
}
// - Implement function `cube` with signature
// `cube(x: number): number` calculating cube of x.
fn cube (x: u32) -> u32{
    x*x*x
}
// Call functions `square` and `cube` in loop, then pass their
// results to fun
// - Call `square` and `cube` in loop 0 to 9, pass results
//   to function `average` on each iteration.
//   Add calculation results to array and return this array
//   from function `calculate`.
// Call functions `square` and `cube` in loop, then pass their
// results to function `average`. Print what `average` returns.

pub fn calculate() -> Vec<u32> {
    let mut array = Vec::new();
    for i in 0..=9 {
        let x = average(square(i), cube(i));
        array.push(x);
    }
    array
}