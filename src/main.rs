use std::env;
use std::fs;
use std::io;

fn get_file_name () -> String {

    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input.");
    file_name
}

fn convert2csv(data : String) -> String {

    // read all attributes

    // store their data

    // output csv formatted string
}

fn main() {

    let filename = get_file_name();
    
    let json_content = fs::read_to_string(filename)
        .expect("Cannot read the file.");
    
    let csv_content = convert2csv(json_content);

    let len = filename.len()-4;
    let file_prefix = &filename[..len];
    let file_prefix = format!("{}.csv",file_prefix);

    fs::write(file_prefix, csv_content);

}
