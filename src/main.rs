mod alternate;

use crate::alternate::alternate;
// first algorithm in Rust 
// i found it very weird to use reference to manipulate parameters of the function
// because they are living in the stack and destroyed when the function call terminate 
// still struggling with ownership
fn main() {
    let result = alternate(1, "true", "false");
    println!("{:?}", result);
    
}