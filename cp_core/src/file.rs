
use std::fs;
use std::io;

/// Read the entire contents of a text file into a String.
///
/// Example:
/// let s = import_txt("/path/to/file.txt")?;
pub fn import_txt<P: AsRef<std::path::Path>>(path: P) -> io::Result<String> {
	fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
	use super::import_txt;

	#[test]
	fn test_import_txt_not_found() {
		let res = import_txt("this_file_should_not_exist_12345.txt");
		assert!(res.is_err());
	}
}
