use std::io::{self, Write};
use std::iter::Peekable;
use std::str::Chars;
use std::slice::Iter;
//use std::fs;
//
//expr = operand (operator operand)*
//operand = identifier | number 
//operator = "+|-|*|/"
//



#[derive(Debug)]
struct Token {
    _type: String,
    value: String,
}

struct Node<'a> {
    pub node: Token,
    pub children: &'a mut Vec<Node<'a>>,
}


fn parse_identifer(tokens: &mut Peekable<Chars>) -> Token {
    let tvalue:String = tokens.take_while(|&c| c != ' ' && c != '\n').collect();
    let t = Token { _type: "IDENTIFER".to_string(), value: tvalue };
    t
}

fn parse_number(tokens: &mut Peekable<Chars>) -> Token {
    let tvalue:String = tokens.take_while(|&c| c != ' ' && c != '\n').collect();
    let t = Token { _type: "NUMBER".to_string(), value: tvalue };
    t
}

fn parse_operator(tokens: &mut Peekable<Chars>) -> Token {
    let mut tvalue = "".to_string();
    let c = tokens.next().unwrap();
    tvalue.push(c);
    let t = Token { _type: "OPERATOR".to_string(), value: tvalue };
    t
}

fn accept_operand<'a>(tokens: &mut Iter<'a, Token>) -> Option<Node<'a>>{
    match tokens.next() {
        None => None,
        Some(t) => { 
            match t._type.trim() {
                "NUMBER" | "IDENTIFIER" => Some(Node { node:*t, children: &mut Vec::new() }),
                _ => None,
            }
        },
    }
}

fn accept_operator<'a>(tokens: &mut Iter<'a, Token>) ->Option<Node<'a>>{
    match tokens.next() {
        None => None,
        Some(t) => { 
            match t._type.trim() {
                "+" | "-"|"*"|"/" => Some(Node { node:*t, children: &mut Vec::new() }),
                _ => None,
            }
        },
    }
}

fn parse( allTokens: Vec<Token> ){
    let mut tokens = &mut allTokens.iter();
    let op1 = accept_operand(tokens);
    if tokens.len() > 1 {
        let mut root = accept_operator(tokens).unwrap();
        let op2 = accept_operand(tokens);
        root.children.push(op1.unwrap());
        root.children.push(op2.unwrap());
    }
}

fn proccess_line(line: String){
    let tokens = &mut line.chars().peekable();
    let mut tokenized: Vec<Token> = Vec::new();
    loop {
        let EOL = '\n';
        let c = *tokens.peek().unwrap_or_else(|| &EOL);
        match c {
            'a'...'z'|'A'...'Z' =>  tokenized.push(parse_identifer(tokens)),
            '0'...'9' => tokenized.push(parse_number(tokens)),
            ' ' =>  {tokens.next(); continue},
            '+'|'-'|'*'|'/' => tokenized.push(parse_operator(tokens)),
            '(' => { tokens.next(); tokenized.push(Token { _type: "RPAR".to_string(), value: "(".to_string()}) },
            ')' => { tokens.next(); tokenized.push(Token { _type: "LPAR".to_string(), value: ")".to_string()})},
            '\n' => break,
            _ => { println!("Not recognized token '{}'", c); break }
        }
    }

    println!("{:?}", tokenized);
    parse(tokenized);
}

fn main_loop() {

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!(">"); stdout.flush();
        let mut line = String::new();
        stdin.read_line(&mut line);
        proccess_line(line);
    }
}


fn main() {
//    let x: &str = "Esteban";
//    let f = do_work;
//    let f2: fn(i32) -> i32 = do_work;
//    f2(4);
//    f(3);
//    print_something(x);
//    let stdin = io::stdin();
//    let mut line = String::new();
//    let res = stdin.read_line(&mut line);
//    println!("{} {:?}", line, res);
//    let path = fs::canonicalize("./src/");
//    println!("{:?}", path.ok().unwrap().parent().unwrap());
    main_loop();
}


