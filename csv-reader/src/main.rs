use std::env;
use std::fs;
use std::fs::read_to_string;
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    
    match command.as_str(){
        "read" => {
            let file_path =&args[2];
            let data = read_file(file_path);
            print_table(data);
        }
        "write" =>
         {
        let file_path = &args[2];
        write_to_file(file_path, &args[3..]);
    }   

        _ => println!("Invalid command"),
    }
}

fn read_file(file_path: &str) -> Vec<Vec<String>>{
    println!("In file {file_path}");

    let mut line_vector = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        line_vector.push(line.to_string());
    }

    let mut individual_vector = Vec::new();

    for item in line_vector.iter() {
        let x: Vec<String> = item.split(",").map(|f| f.to_string()).collect();
        individual_vector.push(x);
    }

    for item in individual_vector.iter() {
        println!("{:?}", item);
    }
    individual_vector
}

fn print_table(data: Vec<Vec<String>>) {
    let cols = data[0].len();
    print_separator(cols);
    for (i, row) in data.iter().enumerate() {
        for field in row.iter() {
            print!("| {:<14}", field);
        }
        println!("|");
        if i == 0 {
            print_separator(cols); // separator after header row
        }
    }
    print_separator(cols);
}

fn print_separator(cols: usize) {
    for _ in 0..cols {
        print!("+---------------");
    }
    println!("+");
}
fn write_to_file(file_path: &str, fields: &[String]) {
    let new_row = fields.join(",");
    let mut content = read_to_string(file_path).expect("Failed to read file");
    content.push_str("\n");
    content.push_str(&new_row);
    fs::write(file_path, content).expect("Failed to write file");
    println!("Row added successfully!");
}

fn filter_data(){


}


