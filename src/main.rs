use finance::read_file_and_sum_total;
use finance::writter_file_sum;


fn main() {
    let total_result = read_file_and_sum_total("test_data/header_file.csv");
    match total_result {
        Ok(result) => {
            println!("Total value of header_file is: {}", result);
            writter_file_sum("test_data/total_header_file.csv", &result.to_string());
        },
        Err(_) => println!("Not found total"),
    }
}
