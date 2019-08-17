use petgraph::Graph;
use std::io;
use std::borrow::Cow;

#[derive(Debug, Clone)]
enum Command<'a> {
    Exit,
    Unknown,
    Help,
    Add(Vec<&'a str>),
    Connect
}

fn print_help() {
    println!("-----");
    println!("Available Commands:");
    println!("add/a - add node");
    println!("help/h - displays this help");
    println!("exit/x - quits the program");
    println!("-----");
}


fn parse_line<'a, 'b>(input: &'a str) -> Cow<'b, Command> {
    let mut iter = input.split_whitespace();
    let command_str = iter.next().unwrap_or("");
    let args:Vec<&str> = iter.collect();
    let command = match command_str {
        "add" | "a" => Command::Add(args.to_owned()),
        "connect" | "c" => Command::Connect,
        "exit" | "x" => Command::Exit,
        "h" => Command::Help,
        _ => {
            println!("unknown command: {}", command_str);
            Command::Unknown
        }
    };
    return Cow::Owned(command);
}

fn main() {
    println!("hello graph!");
    let mut g = Graph::<String, String>::new();

    print_help();
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("read from stdin");
        let command = parse_line(&input.as_str()).into_owned();

        match command {
            Command::Exit => break,
            Command::Help => print_help(),
            Command::Add(args) => {
                for s in args {
                    println!("adding {}", s);
                    g.add_node(s.to_string());
                }
            },
            Command::Connect => (),
            Command::Unknown => ()
        }
    }
}
