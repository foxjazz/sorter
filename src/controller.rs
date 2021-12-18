use std::io::Write;


mod sorter;

    pub fn start() {
        
         
        loop {
            out("sort>");
            let s = readln();
            match s.as_str() {
                "feed" => {
                    let str = feed();
                    sorter::sort(&str);

                },
                "q" => {
                    return;
                },
                "help" => {
                    help();
                },
                _ => (),
            }
        }
    }

    fn out(data: &str) {
        std::io::stdout().write(data.as_bytes()).unwrap();
        std::io::stdout().flush();
    }

    fn feed() -> String {
        out("paste here:"); 
        let mut obuf: String;
        let mut vec: Vec<String> = Vec::new();
       loop{
        let mut buf : [u8; 1];
        
        let r = readln();
        
        if r.contains("}"){
              obuf = vec.join(" ");
              vec.push(r);
              break;
        }
        vec.push(r);

       }
       
       obuf
    }

    fn help() {
        let data = "{
            first: string;
            second: boolean;   third: number;
         forth: Date;   fith: boolean;
            sixth: string;
            seventh: string;
            eight: string;
         }\r\n";
        out("Commands are: feed, help, q   (feed example: \r\n" );
        out("feed reqyires a starting { and an ending } giving this example: \r\n");
        out(data);
    }
    
    fn readln() -> String {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line.pop();
        return line;
    }