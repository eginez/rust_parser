use std::io::{self, Write};
use std::fs;


fn proccess_line(line: String){
    for c in line.chars() {
        match c {
            'a'...'z'|'A'...'Z' => println!("letter"),
            '0'...'9' => println!("number"),
            ' ' => println!("space"),
            '+'|'-'|'*'|'/' => println!("operator"),
            '\n' => println!("EOL"),
            _ => println!("wtf"),
        }
    }
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


