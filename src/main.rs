use finance::{read_file_to_line, split_line, write_line};

fn main() {
    let file = read_file_to_line("data/NU_19247529_01JAN2022_31JAN2022.csv");
    match file {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line_str) => {
                        let data = split_line(&line_str);
                        let result = write_line("data/save.txt", &data.to_string());
                        match result {
                            Ok(_sucess) => {
                                println!("line: ok");
                            }
                            Err(err) => {
                                println!("line error: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        println!("Fail to read line: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            println!("Error to read file: {}", err);
        }
    }
}
