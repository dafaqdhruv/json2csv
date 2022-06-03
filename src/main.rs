use std::env;

mod convert;

fn main() {

    let filename = env::args().nth(1).unwrap();
    
    convert::convert2csv(filename);

}


// fn get_file_name () -> String {
//     let mut file_name = String::new();
//     io::stdin()
//         .read_line(&mut file_name)
//         .expect("Failed to read input.");
//     file_name
// }