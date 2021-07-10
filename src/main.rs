mod algorithms;

use algorithms::encryption;


fn main() {
    let to_test = "foo bar";
    let output = &encryption::caesar(to_test, 5);
    println!("{}", output);
}

// todo: 
// add a basic encryption algorithm
// maybe a basic xor?
// probably should have seperate files
