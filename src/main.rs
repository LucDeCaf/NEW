use std::{env, fs};

#[derive(Debug, PartialEq)]
enum Token {
    BlockQuote,
    Whitespace,
    List { delim: String, ordered: bool },
    Heading(usize),
    Paragraph(String),
}

#[derive(Debug)]
enum HTMLElement {
    Document,
    Heading {
        level: usize,
    },
    OrderedList {
        delim: String,
        tight: bool,
        start: usize,
    },
    UnorderedList {
        delim: String,
        tight: bool,
    },
    ListItem,
    Paragraph,
    BlockQuote,
    Emphasis,
    Strong,
    CodeBlock {
        language: String,
    },
    Text(String),
}

#[derive(Debug)]
struct Node {
    element: HTMLElement,
    children: Option<Vec<Token>>,
    open: bool,
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

// fn deepest_open_node_mut<'a>(root: &'a mut Node) -> Option<&'a mut Node> {
//     if !root.open {
//         return None;
//     }

//     let mut traveller = root;

//     while let Some(traveller.children)

//     traveller
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    // ? Removed mandatory file input for debugging purposes
    // ? Instead using default file path of "./input.md"
    let filepath = match args.len() {
        2 => &args[1],
        _ => "./input.md",
    };

    let file_contents = fs::read_to_string(filepath).unwrap();

    let mut tokens: Vec<Token> = vec![];

    for line in file_contents.lines() {
        let mut line = line.trim().to_owned();

        // Whitespace
        if line.len() == 0 {
            tokens.push(Token::Whitespace);
            continue;
        }

        // Blockquote
        if line.starts_with(">") {
            tokens.push(Token::BlockQuote);
            line = remove_indicator(&line);
        }

        // List
        if line.starts_with("-") {
            tokens.push(Token::List {
                delim: line.chars().next().unwrap().to_string(),
                ordered: false,
            });
            line = remove_indicator(&line);
        }

        // Heading
        let hashless = line.trim_start_matches('#');
        let hash_count = line.len() - hashless.len();

        if hash_count > 0 && hash_count < 7 {
            tokens.push(Token::Heading(hash_count));
            line = hashless[1..].to_owned();
        }

        // Paragraph
        tokens.push(Token::Paragraph(line));
    }

    dbg!("Tokens: ", &tokens);

    // let mut document = Node {
    //     element: HTMLElement::Document,
    //     children: Some(vec![]),
    //     open: true,
    // };

    // for token in tokens {
    //     let last_open = deepest_open_node_mut(&mut document);

    // }
}
