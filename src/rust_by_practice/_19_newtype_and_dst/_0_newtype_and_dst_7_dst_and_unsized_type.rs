/* Make it work with const generics */
fn my_function<const n: usize>() -> [u32; n] {
    [123; n]
}

pub fn main() {
    let arr = my_function::<1>();
    println!("{:?}",arr);
}
