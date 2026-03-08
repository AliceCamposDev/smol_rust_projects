use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn load_vault_fn() {
    let mut file = fs::File::create("example.md").expect("Unable to create file");
    writeln!(file, "Hello, world!").expect("Unable to write to file");
    let contents = fs::read_to_string("example.txt").expect("Unable to read file");
    println!("File contents: {}", contents);

    let mut file2 = fs::File::create("example2.md").unwrap();
    file2.write_all("World, hello!".as_bytes()).unwrap();

    let mut f = File::open("intro.md").unwrap();

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();




    println!("File contents: {:?}", String::from_utf8_lossy(&buffer));
    let path = Path::new("./documents_to_test");
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let content = fs::read_to_string(&path).unwrap();
            println!("Arquivo: {:?}\n{}", path, content);
        }
    }
}