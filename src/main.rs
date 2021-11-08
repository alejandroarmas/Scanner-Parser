//mod character_stream;
//use character_stream::*;


mod token;
use token::*;

mod reader;
use reader::*;

mod lexer;
use lexer::*;

mod parser;
use parser::*;


mod character_stream;
use character_stream::*;

		

fn main() {

	let filename = "../data/example1.x";

	let mut lx = Lexer::new(filename);

	let tokens = lx.scan();

	for t in tokens {
		println!(" Lexume: {:^10} | Type: {:^14} | Position:({:4}, {:2})", t.get_text(), t.get_type().as_str(), t.get_char_pos(), t.get_line_number());
	}
	

}
