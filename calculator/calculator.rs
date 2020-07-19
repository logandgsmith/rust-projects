use std::io;

fn main() {
    let mut input = String::new();
    //Preamble
    println!("Calculator v0.1");
    println!("Enter expressions like \"1 + 1\" ");
    println!("^C to Quit");

    loop {
        io::stdin().read_line(&mut input).expect("Couldn't read input");
        println!("You entered: {}", input);
    }
}