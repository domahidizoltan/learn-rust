
// Question: how many heap allocations are happening here?
// Your answer: 
pub fn main() {  
    // Create a String type based on `&str`
    // The type of string literals is `&str`
   let s: String = String::from("hello, world!"); // 1 here

   // Create a slice point to String `s`
   let slice: &str = &s;

   // Create a String type based on the recently created slice
   let s: String = slice.to_string(); // 2 here

   assert_eq!(s, "hello, world!");

   println!("Success!");
}
