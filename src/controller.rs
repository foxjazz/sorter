/*use std::io::Write;
use std::io::Read;
use std::io;

pub mod controller {
    pub fn start() {
        loop {
            out("tm>");
            let s = readln();
            match s.as_str() {
                "feed" => {
                    feed();
                }
                "q" => {
                    return;
                }
                "help" => {
                    help();
                }
            }
        }
    }

    fn out(data: &str) {
        std::io::stdout().write(data.as_bytes()).unwrap();
        std::io::stdout().flush();
    }

    fn feed() {
        out("paste here:");
    }

    fn help() {
        out("Commands are: feed, help, q");
    }

    fn readln() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.pop();

        return line;
    }
}*/