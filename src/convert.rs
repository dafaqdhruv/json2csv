use std::fs;
use std::io::Write;
use std::collections::HashSet;

use serde_json::Value;


pub fn convert2csv(filename : String) {
    let json_content = {
        let temp = fs::read_to_string(&filename).unwrap();
        serde_json::from_str::<Value>(&temp).unwrap()
    };
    
    
    // get attribute headers
    let mut headers = Vec::new();
    
    let mut temp = HashSet::new();
    for i in json_content.as_array() {
        for j in i {
            for key in j.as_object().unwrap().keys() {
                temp.insert(key.to_string());
            }
        }
    }
    for i in temp {
        headers.push(i.to_string());
    }
            

    let len = filename.len()-4;
    let file_prefix = &filename[..len];
    let file_prefix = format!("{}csv",file_prefix);

    fs::File::create(&file_prefix)
        .expect("Cannot create file.");

    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_prefix)
        .unwrap();

    // dbg
    let mut temp = String::new();
    let len = headers.len();
    for i in &headers[..len-1] {
        temp.push_str(i);
        temp.push_str(",");        
    }
    temp.push_str(&headers[len-1].to_string());
    temp.push_str("\n");
    output_file.write_all(temp.as_bytes()).expect("cannot write headers");
    

    for i in json_content.as_array() {
        for j in i {
            // dbg!(json_record_to_csv(&headers, &j));
            output_file.write_all(json_record_to_csv(&headers, &j).unwrap().as_bytes())
                .expect("Error writing to file.");
        }
    }
}


// converts a single json object to csv 
pub fn json_record_to_csv( headers: &Vec<String>, obj: &Value)  -> Result<String, bool>{
    
    let mut output = String::new();

    for i in headers {
        let temp = obj.get(&i);
        let result = match temp {
            Some(x) => match x.as_str() {
                Some(s) => String::from(s),
                None => x.to_string(),
            },
            None => String::from(""),
        };
        output.push_str(&result);
        output.push_str(",")
        // add comma
    }
    // remove last comma
    let len = output.len()-1;
    output = String::from(&output[..len]);
    output.push_str("\n");
    

    Ok(output)
}