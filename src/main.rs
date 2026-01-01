use std::env;

fn main() {
    //TODO: look at usage of args_os() instead [https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html#the-args-function-and-invalid-unicode]
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
