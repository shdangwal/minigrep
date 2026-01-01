use std::env;
use std::fs;

fn main() {
    //TODO: look at usage of args_os() instead [https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html#the-args-function-and-invalid-unicode]
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Shoud have been able to read the file");

    println!("With text:\n{contents}");
}
