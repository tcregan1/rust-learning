use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    
    match command.as_str(){
        "read" => {
            let file_path =&args[2];
            read_file(file_path);
        }

        _ => println!("Invalid command"),
    }
}

fn read_file(file_path: &str)
{
    println!("In file {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    

}

fn write_to_file(){


}

fn filter_data(){


}


