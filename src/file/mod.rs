use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
pub fn load_file(path:&str){
    println!("the file path is	{}", path);
    let path = Path::new(path);
    let display = path.display();
    let mut file = match File::open(&path) {
      Err(why) => panic!("couldn't open {}: {}", display,
                                                 Error::description(&why)),
      Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
       Err(why) => panic!("couldn't read {}: {}", display,
                                                  Error::description(&why)),
       Ok(_) => {get_stock_info(&mut s)},
    }



}

pub fn get_stock_info(str:&mut String){
    let str_v : Vec<&str> = str.split('\n').collect();

    for st in &str_v{
        if(st.len()>0){
            let v: Vec<&str> = st.split_whitespace().collect();
            println!("date:{},openPrice={}", v[0], v[1]);
        }

    }

}
