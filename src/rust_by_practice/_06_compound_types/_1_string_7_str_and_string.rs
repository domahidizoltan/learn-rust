
// Fix error with at least two solutions
pub fn main() {
    let s = "hello, world";
    greetings(s.to_string());
}

fn greetings(s: String) {
    println!("{}", s)
}
