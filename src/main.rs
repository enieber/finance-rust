use finance::read_file_line_by_line;

fn main() {
    let total_result = read_file_line_by_line("test_data/header_file.csv");
    match total_result {
        Ok(result) => println!("Total value of header_file is: {}", result),
        Err(_) => println!("Not found total"),
    }
}
