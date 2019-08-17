use petgraph::Graph;
use std::io;

#[derive(Debug, Clone)]
enum Command<'a> {
    Exit,
    Unknown,
    Help,
    Add(Vec<&'a str>),
    Connect
}

impl Command<'_> {
    pub fn print_help() {
        println!("-----");
        println!("Available Commands:");
        println!("add/a - add node");
        println!("help/h - displays this help");
        println!("exit/x - quits the program");
        println!("-----");
    }
    pub fn parse_line<'a, 'b>(input: &'a str) -> Command {
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
        return command;
    }
}

fn main() {
    println!("hello graph!");
    let mut g = Graph::<String, String>::new();

    Command::print_help();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read from stdin");
        let command = Command::parse_line(&input.as_str());

        match command {
            Command::Exit => break,
            Command::Help => Command::print_help(),
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
