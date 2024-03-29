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
    children: Vec<usize>,
    open: bool,
}

#[derive(Debug)]
struct Graph {
    stack: Vec<Node>,
    root: Option<usize>,
}

impl Graph {
    fn new() -> Self {
        Self {
            stack: vec![],
            root: None,
        }
    }

    fn set_root(&mut self, root: Option<usize>) {
        self.root = root;
    }

    fn add_node(&mut self, node: Node) -> bool {
        let stack_len = self.stack.len();

        // Handle empty stack or no root
        if stack_len == 0 || self.root == None {
            self.root = Some(0);
            self.stack.push(node);
            return true;
        }

        let last_open_index = self.last_open_index();
        let last_open_node = self.get_node_mut(last_open_index);

        if let Some(last_node) = last_open_node {
            last_node.children.push(stack_len);
            self.stack.push(node);

            return true;
        }

        false
    }

    fn get_node(&self, index: Option<usize>) -> Option<&Node> {
        if let Some(index) = index {
            self.stack.get(index)
        } else {
            None
        }
    }

    fn get_node_mut(&mut self, index: Option<usize>) -> Option<&mut Node> {
        if let Some(index) = index {
            self.stack.get_mut(index)
        } else {
            None
        }
    }

    fn last_open_index(&mut self) -> Option<usize> {
        // Check if stack is empty
        if self.stack.is_empty() {
            return None;
        }

        // Check if root is closed
        if let Some(root) = self.get_node(self.root) {
            if !root.open {
                return None;
            }
        }

        let mut current_index = self.root;

        loop {
            let current_node = self.get_node(current_index).unwrap();

            let last_child_index = current_node.children.last().map(ToOwned::to_owned);
            let last_child = self.get_node(last_child_index);

            let child = if let Some(c) = last_child {
                c
            } else {
                break;
            };

            if !child.open {
                break;
            }

            current_index = last_child_index;
        }

        current_index
    }
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

    let mut graph = Graph::new();

    for token in tokens {}

    dbg!("Graph: ", &graph);
}
