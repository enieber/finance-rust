use std::result::Result::{Err, Ok};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// # Read file as string
/// receive path csv file in string and parse and calculate valus
pub fn read_file_and_sum_total(filepath: &str) -> Result<f32, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut total_value = 0.0;
    let lines = reader.lines();
    let first_line_data = lines[1];
    let line_object = split_line_to_vec(&line_object);
    println!("test {}", line_object[0]);
    let last_line = lines.last();
    match last_line {
        Some(some_line) => {
            match some_line {
                Ok(line_ok) => {
                    let line_item = split_line_to_vec(&line_ok);
                    println!("last lin is: {}", line_item[0]);
                },
                Err(_) => {}
            }
        },
        None => {}
    }
  //  for line in reader.lines() {
    //    total_value += sum_total_by_line(line);
    //}
    Ok(total_value)
}

fn split_line_to_vec(str_line: &String) -> Vec<&str> {
    let list = str_line
        .split(",")
        .collect();
    list
}

/// ## sum_total_by_line function sum item value in position 1
/// example: date, value, id, description
fn sum_total_by_line(line: Result<String, std::io::Error>) -> f32 {
    let mut total_value = 0.0;
    match line {
            Ok(str_line) => {
                let arr_items: Vec<&str> = split_line_to_vec(&str_line);
                let value = arr_items[1];
                let result_value: Result<f32, _> = value.parse();
                match result_value {
                    Ok(value_number) => total_value += value_number,
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    total_value
}
