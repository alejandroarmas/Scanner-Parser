use std::fs::File;
use std::io;
use std::io::prelude::*;


struct Lines<R> {
    reader: io::BufReader<R>,
    buffer: String
}

impl <R: Read> Lines<R> {

	/*
	Code Source: https://stevedonovan.github.io/rust-gentle-intro/3-filesystem.html#another-look-at-reading-files
	*/
	
	fn new(file: R) -> Lines<R> {
		Lines{reader: io::BufReader::new(file), buffer: String::new()}
    }
	
    fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>>{
		self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
			Ok(nbytes) => if nbytes == 0 {
				None // no more lines!
            } else {
                let line = self.buffer.trim_end();
                Some(Ok(line))
            },
            Err(e) => Some(Err(e))
        }
    }
}


pub struct Reader {
	program_file_name: String,
	source_code_lines: String
}


impl Reader {
	
	pub fn new(file_name: &str) -> Reader {
		Reader{program_file_name : file_name.to_string(), source_code_lines: String::default()}
	}

	pub fn get_program_source(mut self) -> io::Result<String> {

		self.source_code_lines.clear();

		let file = File::open(&self.program_file_name)?;
	
		let mut lines = Lines::new(file);

		while let Some(line) = lines.next() {
			let line = line?;
			self.source_code_lines.push_str(&line.to_string());
			self.source_code_lines.push_str("\n");
			// println!("{}", line);
		}
	
		Ok(self.source_code_lines)
	}

}


