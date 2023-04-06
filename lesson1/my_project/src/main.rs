use std::io;

/// Create comment. 
/// What is this crate doing?

fn main() {

    //! # Main Function
    //! ```
    //! fn main() 
    //! ```
    //! 
    //! entry point of the program, reads user inputs and outputs to stdout
    //! 
    let mut input = String::new();

    // print a message to the user
    println!("Say Something! ");

    /*  
        Check response and respond accordingly
     */
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said: {}", input);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
