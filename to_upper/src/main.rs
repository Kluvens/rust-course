fn main() {
    // try and get the first argument
    // using some iterator methods
    let arg = std::env::args().nth(1).expect("No argument is given");

    // the compiler suggests to "borrow here"
    // but we haven't learnt how to borrow :(
    // we have a String type, and want to get a &str
    // Try find a function that can help us using
    // the docs https://doc.rust-lang.org/stable/std/string/struct.String.html
    let upp = uppercase(&arg);
    
    println!("arg = {}", arg);
    println!("upp = {}", upp);
}

fn uppercase(src: &str) -> String {
    src.to_uppercase()
}
