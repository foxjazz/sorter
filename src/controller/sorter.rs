

pub fn sort(dataw: &String) -> String {
        
        if dataw.chars().nth(0) != Some('{') {
                panic!("Error");
        }
        //let &mut dataw = data;
        let mut data = dataw.to_string();
        data.retain(|c| c != '\r' && c != '\n' && c != '{' && c != '}');
      
       let mut lines = data.split_inclusive(';').collect::<Vec<_>>();
        lines.sort();
        lines.insert(0, "{");
        lines.push("}");
       let s =  lines.join("\r\n");
       println!("{}", s);
        s
}


