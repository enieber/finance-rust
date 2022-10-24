use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Lines, Write};
use std::path::Path;

pub fn read_file_to_line(file_path: &str) -> Result<Lines<BufReader<File>>, String> {
    let file = File::open(file_path);
    match file {
        Ok(file_open) => {
            let file_buffer = BufReader::new(file_open);
            return Ok(file_buffer.lines());
        }
        Err(err) => {
            return Err(format!("Error to read file: {}", err));
        }
    }
}

fn create_file_or_open(file_path: &str) -> Result<File, Error> {
    if Path::new(file_path).exists() {
        let file = OpenOptions::new().write(true).append(true).open(file_path);
        return file;
    } else {
        let file = File::create(file_path);
        return file;
    }
}

pub fn write_line(file_path: &str, line: &str) -> Result<String, String> {
    let file = create_file_or_open(file_path);

    match file {
        Ok(mut file_buffer) => {
            let has_write = writeln!(file_buffer, "{}", &line);
            match has_write {
                Ok(_ok) => {
                    return Ok(format!("Line write with success"));
                }
                Err(err) => {
                    return Err(format!("Error to write file: {}", err));
                }
            }
        }
        Err(err) => {
            return Err(format!("Error to open file: {}", err));
        }
    }
}

pub struct Data {
    date: String,
    value: String,
    id: String,
    description: String,
}

impl ToString for Data {
    fn to_string(&self) -> String {
        return format!(
            "The {} has value {} in {} with description: {}",
            &self.id, &self.value, &self.date, &self.description
        );
    }
}

fn convert_to_data(value_data: Vec<&str>) -> Data {
    let data = Data {
        date: value_data[0].to_string(),
        value: value_data[1].to_string(),
        id: value_data[2].to_string(),
        description: value_data[3].to_string(),
    };
    return data;
}

pub fn split_line(line: &str) -> Data {
    let value_data: Vec<&str> = line.split(",").collect();
    let data = convert_to_data(value_data);
    return data;
}
