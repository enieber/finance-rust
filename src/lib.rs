use std::result::Result::{Err, Ok};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// # Read file as string
/// receive path csv file in string and parse and print text with data parsed from file
pub fn read_file_line_by_line(filepath: &str) -> Result<f32, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut total_value = 0.0;

    for line in reader.lines() {
        match line {
            Ok(str_line) => {
                let arr_items: Vec<&str> = str_line.split(",").collect();
                let value = arr_items[1];
                let result_value: Result<f32, _> = value.parse();
                match result_value {
                    Ok(value_number) => total_value += value_number,
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
    Ok(total_value)
}
