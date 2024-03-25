use std::{env, fs};

#[derive(Debug, PartialEq)]
enum Block {
    Document(Vec<Block>),
    BlockQuote(Vec<Block>),
    List(Vec<Block>),
    Paragraph(String),
}

fn remove_indicator(line: &str) -> String {
    if let Some(ch) = line.chars().nth(1) {
        if ch.is_whitespace() {
            line[2..].to_owned()
        } else {
            line[1..].to_owned()
        }
    } else {
        String::new()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("invalid argument count");
    }

    let file_contents = fs::read_to_string(&args[1]).unwrap();

    let mut document = Block::Document(vec![]);

    for line in file_contents.lines() {
        let line = line.trim();

        if line.starts_with(">") {
            
        }
    }

    dbg!(&document);
}
