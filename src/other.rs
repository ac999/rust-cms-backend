use std::io::{Error, ErrorKind};
use std::path::Path;
use std::u16;

use regex::Regex;

pub fn file_exist(path: String) -> Result<String, Error> {
	let error = Error::new(
		ErrorKind::Other, "The specified path does not exist.");
	if Path::new(&path).exists(){
		Ok(path)
	} else {
		Err(error)
	}
}

pub fn string_to_u16(_str: String) -> u16 {
	u16::from_str_radix(&_str, 10)
	.expect("Error converting string to u16")
}

pub fn mail_check(_mail: String) -> bool {
	let re = Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)")
	.expect("Regex error");
	re.is_match(_mail.as_str())
}