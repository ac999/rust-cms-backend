use std::io::{Error, ErrorKind};
use std::path::Path;
use std::u16;

pub fn file_exist(path: String) -> Result<String, Error> {
	let error = Error::new(
		ErrorKind::Other, "The specified path does not exist.");
	if Path::new(&path).exists(){
		Ok(path)
	} else {
		Err(error)
	}
}

pub fn string_to_u16(_str: String) -> u16{
	from_str_radix(_str, 10)
	.expect("Error converting string to u16")
}