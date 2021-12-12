mod sorter;
// use controller;
fn main() {

    test();
    println!("Hello, world!");
}
fn test(){
 let data = "{
   first: string;
   second: boolean;   third: number;
   forth: Date;   fith: boolean;
   sixth: string;
   seventh: string;
   eight: string;
}".to_string();
let result = sorter::sort(&data);
    println!( "{}", result.as_str()  );
}
/*pub fn start(){
 controller::start();
}
*/