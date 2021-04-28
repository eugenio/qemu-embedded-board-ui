use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde_lexpr::from_str;
pub fn get_file_content() -> String {
		// Create a `Path` from an `&'static str`
	let path = Path::new("./../../");



	// `join` merges a path with a byte container using the OS specific
	// separator, and returns the new path
	let new_path = path.join("resources/prova.kicad_sch");

	// The `display` method returns a `Show`able structure
	let display = new_path.display();
	
	// Convert the path into a string slice
	let mut file = match File::open(&new_path) {
		Err(why) => panic!("couldn't open {}: {}", display, why),
		Ok(file) => file,
	};		

	// Read the file contents into a string, returns `io::Result<usize>`
	let mut s = String::new();

	match file.read_to_string(&mut s) {


		Err(why) => panic!("couldn't read {}: {}", display, why),
		Ok(s) => s,
	};

	let result: String = from_str(&s).unwrap();

	result
}
