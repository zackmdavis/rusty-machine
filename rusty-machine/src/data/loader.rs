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

	// For now we will simply throw errors if we have missing data.
	// We will only handle Real data types. Anything else will throw errors.
	//
	// Reading should go like this:
	// Read a line - 
	//
	// Count cols, add data to the relevent DataCols
	// 
}

/// A data table consisting of varying column types and headers.
pub struct DataTable {
	rows: usize,
	cols: usize,
	data_cols: Vec<DataCols<f64>>,
}

enum DataType {
	Reals,
	Categorical,
}
/// A data column with a consistent data type. 
pub struct DataCols<T> {
	data_type : DataType,
	data: Vec<T>,
	// If categorical - some kind of set for each category

}