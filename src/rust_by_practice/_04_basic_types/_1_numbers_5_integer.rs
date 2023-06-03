
// Fix errors and panics to make it work
pub fn main() {
    let v1 = 251_u8 + 4;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}",v1,v2);
 }
 