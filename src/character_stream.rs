use crate::reader::*;


// TODO: change character buffer into a FIFO queue in constructor
pub struct CharStream {
	character_buffer: String,
	current_char_ptr: usize
}


impl CharStream {


	pub fn new(filename: &str) -> CharStream {
	
		let reader = Reader::new(filename);
	
		CharStream{
			character_buffer: reader.get_program_source()
								.expect("Error parsing source file."),
			current_char_ptr: 0
		}
	}
	
	// Returns true if more characters are available, false otherwise.
	pub fn more_available(&self) -> bool {
	
		let mut available = false;
		let charOption = self.character_buffer.chars().nth(self.current_char_ptr + 1);

		if charOption.is_some() {
			available = true;	
		}
		available
	}

	// Returns the next character without consuming it.
	// Returns None if no more characters are available. 
	pub fn peek_next_char(&self) -> Option<char> {
		let charOption = self.character_buffer.chars().nth(self.current_char_ptr);
		charOption
	}
	
	// Returns the kth character ahead in the stream without consuming it.
	// peek_ahead_char(0) returns the same character as peek_next_char().
	// Returns None if no more characters are available at the position.
	// The input k cannot be negative.
	
	pub fn peek_ahead_char(&mut self, k: usize) -> Option<char> {
	
		let maxCharIndex = self.character_buffer.len() - 1;

		let mut lookAheadIndex = self.current_char_ptr + k;
		
		if lookAheadIndex > maxCharIndex {
			lookAheadIndex = maxCharIndex;
		}
		self.current_char_ptr = lookAheadIndex; 
		let charOption = self.character_buffer.chars().nth(lookAheadIndex);
		
		charOption			
	}
	

	// Returns the next character and consumes it.
	// Returns None if no more characters are available.
	pub fn get_next_char(&mut self) -> Option<char> {
		let charOption = self.character_buffer.chars().nth(self.current_char_ptr);
		
		if charOption.is_some() {
			self.current_char_ptr +=1;	
		}

		charOption
	}
}
