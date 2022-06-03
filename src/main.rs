use std::fs;
use std::io;
mod convert;

fn get_file_name () -> String {

    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input.");
    file_name
}

fn convert2csv(data : String) -> String {

    // get all attribute headers
    let headers = parse_headers(data);

    // serialize their data
    // for every json record, run parse_record

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
