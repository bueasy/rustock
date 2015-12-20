use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
#[path="../stock/mod.rs"]
mod stock;
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
    let str_v : Vec<&str> = str.lines().collect();
    let mut map =  HashMap::new();
    for st in &str_v{
        if(st.len()>0&&!st.starts_with('#')){
            let v: Vec<&str> = st.split_whitespace().collect();
            //println!("date:{},openPrice={}", v[0], v[1]);
            let open_price_v = v[1].parse::<f64>().ok().expect("wrong openPrice!");
            let high_price_v = v[2].parse::<f64>().ok().expect("wrong highPrice!");
            let low_price_v = v[1].parse::<f64>().ok().expect("wrong lowPrice!");
            let close_price_v = v[1].parse::<f64>().ok().expect("wrong closePrice!");
            let knock_qty_v = v[1].parse::<u64>().ok().expect("wrong knockQty!");
            let exch_id_v="1".to_string();
            let stk_id_v="399300".to_string();
            let  stk=stock::Stock{exch_id:exch_id_v,stk_id:stk_id_v,
                    stk_name:"沪深300".to_string(),
                    open_price:open_price_v,close_price:close_price_v,
                    high_price:high_price_v,low_price:low_price_v,knock_qty:knock_qty_v};
            map.insert(exch_id_v.push_str(stk_id_v),stk);
        }

    }

}
