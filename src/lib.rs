use std::result::Result::{Err, Ok};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// # Read file as string
/// receive path csv file in string and parse and print text with data parsed from file
pub fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(strLine) => {
                let arrItems: Vec<&str> = strLine.split(",").collect();
                let date = arrItems[0];
                let value = arrItems[1];
                let id = arrItems[2];
                let description = arrItems[3];
                println!(
                    "{} in {} has valued: {}, with {}",
                    id, date, value, description
                );
            }
            Err(_) => println!("Deu ruim"),
        }
    }

    Ok(())
}
