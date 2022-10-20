use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::io::Write;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
struct Options {
    page: usize,
    display: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Page {
    display: String,
    options: Vec<Options>,
}
fn main() {
    let (story, choice) = Main_Menu();
    let contents = fs::read_to_string(/*Path::new(*/ &story[choice].path())
        .expect("Should have been able to read the file");
    read_book(contents, story[choice].path().display().to_string());
}

fn Main_Menu() -> (Vec<DirEntry>, usize) {
    let path = env::current_dir().unwrap(); //.parent()
    let path = path.join("resources");
    let mut options = vec![];
    println!("Pick a story using the correct number:");
    let mut i: usize = 0;
    for file in fs::read_dir(path).unwrap() {
        let file = file.unwrap();
        if file
            .file_name()
            .into_string()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            != ""
        {
            println!(
                "{i}.\t{}",
                &file
                    .file_name()
                    .into_string()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap()
            );
            options.push(file);
            i += 1;
        }
    }
    let choice = int_Input_Check(0, options.len() - 1);
    let story = options;
    (story, choice)
}

fn int_Input_Check(valid_range_start: isize, valid_range_end: usize) -> usize {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut input: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input! Please type a number!");
            int_Input_Check(valid_range_start, valid_range_end)
        }
    };
    if valid_range_start != -1 {
        let valid_range = valid_range_start as usize..=valid_range_end;
        if !valid_range.contains(&input) {
            println!("Invalid Input! Please type a valid input (Numbers {valid_range_start}-{valid_range_end})!");
            input = int_Input_Check(valid_range_start, valid_range_end);
        }
    }
    input
}

fn read_book(data: String, mut booktitle: String) {
    let book: HashMap<String, Page> = serde_json::from_str(&data).expect("Unable to parse");
    let mut pagenum: &usize = &1;
    let mut currentpage = &book[&pagenum.to_string()];
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    dbg!(now.to_string());
    booktitle.push_str("-");
    booktitle.push_str(&now.to_string());
    dbg!(&booktitle);
    while !currentpage.options.is_empty() {
        println!("{}", &currentpage.display);
        if currentpage.options.len() == 1 {
            pagenum = &currentpage.options[0].page;
        } else {
            for (i, child) in currentpage.options.iter().enumerate() {
                let other = String::from("To continue,");
                println!(
                    "{} Choose Option {i}",
                    child.display.as_ref().unwrap_or(&other)
                );
            }
            let position = int_Input_Check(0, currentpage.options.len() - 1);
            pagenum = &currentpage.options[position].page;
        }
        currentpage = &book[&pagenum.to_string()];
        if currentpage.options.is_empty() {
            println!("{}", &currentpage.display);
        }
    }
}
