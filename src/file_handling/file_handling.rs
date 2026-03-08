use std::{fs};
use std::io::Write;


pub fn file_handling_fn() {
    let mut file = fs::File::create("example.txt").expect("Unable to create file");
    writeln!(file, "Hello, world!").expect("Unable to write to file");
    let contents = fs::read_to_string("example.txt").expect("Unable to read file");
    println!("File contents: {}", contents);

    let mut file2 = fs::File::create("exemple2.txt").unwrap();
    file2.write_all("World, hello!".as_bytes()).unwrap();
    let contents2 = fs::read_to_string("exemple2.txt").expect("Unable to read file");
    println!("File contents: {}", contents2);


}