use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct Loader<'a> {
	file: &'a str
}

impl<'a> Loader<'a> {
	pub fn load_file(self) -> Result<(), io::Error> {
		let f = try!(File::open(self.file));
		let reader = BufReader::new(f);

		for line in reader.lines() {
		    let line = try!(line);
		    println!("{}", line);
		}

		Ok(())
	}
}
