
// Fix the errors
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0.0 as isize,
    One = 1.0 as isize,
    Two = 2.0 as isize,
}


pub fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as isize, Number1::One as isize);
    assert_eq!(Number1::One as isize, Number2::One as isize);

    println!("Success!");
} 
