/* Annotate struct with lifetime:
1. `r` and `s` must have different lifetimes
2. lifetime of `s` is bigger than that of 'r'
*/
struct DoubleRef<'a,'b, T> {
    r: &'a T,
    s: &'b T
}
pub fn main() {
    println!("Success!")
}
