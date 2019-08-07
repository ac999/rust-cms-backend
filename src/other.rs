use std::io::{Error, ErrorKind};
use std::path::Path;

pub fn file_exist(path: String) -> Result<String, Error> {
	let error = Error::new(
		ErrorKind::Other, "The specified path does not exist.");
	if Path::new(&path).exists(){
		Ok(path)
	} else {
		Err(error)
	}
}