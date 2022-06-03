use std::fs;
use std::io;
use std::env;
use std::collections::HashSet;

use serde_json::{json, Deserializer, Value};


fn get_file_name () -> String {

    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input.");
    file_name
}

// fn convert2csv(data : String) -> String {

//     // get all attribute headers
//     let headers = parse_headers(data);

//     // serialize their data
//     // for every json record, run parse_record

//     // output csv formatted string

// }

fn main() {

    let filename = env::args().nth(1).unwrap();
    
    let mut json_content = {
        let temp = fs::read_to_string(&filename).unwrap();
        serde_json::from_str::<Value>(&temp).unwrap()
    };
    
    
    // get headers 
    // let mut headers =  HashSet::new();

    for i in json_content.as_array() {
        //  println!("{:#?}",i);
        for j in i {
                // println!("{:#?}",j);
                for key in j.as_array().unwrap() {
                println!("{:#?}",key);
                println!("ok")
                // headers.insert(key);
            }
        }
    }
    
    

    let len = filename.len()-4;
    let file_prefix = &filename[..len];
    let file_prefix = format!("{}csv",file_prefix);

    println!("{}", file_prefix);
    // fs::write(file_prefix, csv_content);

}
