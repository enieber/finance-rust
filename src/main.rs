use std::{
	error,
	fs::{read_to_string, write},
	path::Path,
	result,
};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;


/// # Read file as string
/// receive p in string and need implemenetation
fn read_file(p: &str) -> TResult<String> {
	read_to_string(p).map_err(|e| e.into())
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_read_file_with_success() {
    	let result = read_file("test_data/empty_file.csv");
    	assert!(result.is_ok());
    }

    #[test]
    fn test_read_file_with_success_empty_file() {
    	let result = read_file("test_data/empty_file.csv");

    	if let Ok(s) = result {
    		assert_eq!(s, "");
    	}
    }

	#[test]
    fn test_read_file_with_success_with_header() {
    	let result = read_file("test_data/header_file.csv");

    	if let Ok(s) = result {
 	  		assert_eq!(s, "Data,Valor,Identificador,Descrição\n");    	
 	  	}
    }
    
}
