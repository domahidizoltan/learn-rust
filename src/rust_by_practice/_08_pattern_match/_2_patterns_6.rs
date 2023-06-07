
// FIX the error with least changing
// DON'T remove any code line
pub fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
       value => value.push_str(" world!") 
    }
}
