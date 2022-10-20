use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut multiplecontents = vec![];
    for file in fs::read_dir("/Users/anhcowen/Downloads/en/xhtml/fw/01hh/").unwrap() {
        let filepath = String::from(file.unwrap().path().display().to_string());
        if filepath.contains("sect"){
            let mut fileopened = match File::open(&filepath) {
                Err(why) => panic!("couldn't open {}: {}", filepath, why),
                Ok(file) => file,
            };
            let mut contents = String::new();
            fileopened.read_to_string(&mut contents).unwrap();
            multiplecontents.push(contents);
        }  
    }
    let mut file;
    if std::path::Path::new("book.html").exists(){
        file = OpenOptions::new().append(true).open("book.html").expect(
            "cannot open file");
    }else{
    file = std::fs::File::create("book.html").expect(
        "cannot open file");}
    for line in multiplecontents{
        file.write_all(line.as_bytes()).expect("write failed");
    }
}