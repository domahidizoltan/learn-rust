
// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!("lemonade panic!")
     }

    println!("Exercise Failed if printing out this line!");
}

pub fn main() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}
