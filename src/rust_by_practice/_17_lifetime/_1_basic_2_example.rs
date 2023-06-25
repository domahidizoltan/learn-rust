
/* Annotate `r` and `x` as above, and explain why this code fails to compile, in the lifetime aspect. */

pub fn main() {  
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
    
    {
        let r;                // ---------+-- 'a
        let x = 5;
                              //          |
        {                     //          |
            //let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
}
