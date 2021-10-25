//mod character_stream;
//use character_stream::*;


// mod token;
// use token::*;

mod reader;
use reader::*;

		

fn main() {
	// let tt = TokenType::OPERATOR;
	// let token = Token::new("+".to_string(), tt, 2, 30);
	
	let filename = "../data/example1.x";

	let reader = Reader::new(filename.to_string());

	let program_lines = reader.vectorize_program_lines();

	println!("> {:?}", program_lines);
	// for (i, line) in program_lines.iter().enumerate() {
		// 	println!("Line {}> {}",i , Ok(program_lines)).to_string();
	// }

	// println!("text: {}", token.get_text());
	// println!("token type: {}", token.get_type().as_str());
	// println!("line numer: {}", token.get_line_number());
	// println!("char position: {}", token.get_char_pos());
}
