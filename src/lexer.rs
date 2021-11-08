
use crate::token::*;
use crate::reader::*;
use crate::character_stream::*;



/*

Token Types: 

TokenType::NONE => "None",
TokenType::INTCONSTANT => "IntConstant",      ->
TokenType::FLOATCONSTANT => "FloatConstant",  -> double, float
TokenType::OPERATOR => "Operator",            -> ( ) { } = == < > <= >= != + - * / ; ,
TokenType::KEYWORD => "Keyword",              -> unsigned char short int long float double while if return void main
TokenType::VARIABLE => "Variable",            ->
TokenType::FUNCTION => "Function",            ->
TokenType::INVALID => "Invalid"

Keywords of language X: unsigned, char, short, int, long, float, double, while, if, return, void, and main.

Operators of language X are: (, ), {, }, =, ==, <, >, <=, >=, !=, +, -, *, /, ; and , 

Each nested block should be indented another level.

_____________________________________________

Input: 

float Foo(int val);

void main(){
    float Value;
    
	Value = Foo(7);
}

float Foo(int val){
    float TestFloat = 1.4;
    
    while(val > 0){
        TestFloat = TestFloat * TestFloat;
        val = val - 1;   
    }
    return TestFloat;
}

______________________________________________


*/

pub mod LexerHelper {

    pub static DECIMAL_CONSTANT: char = '.';

    use crate::TokenType;

    pub struct DistinguishTokenType {
        types: Vec<String>
    }
    
    impl DistinguishTokenType {
    
        pub fn new() -> DistinguishTokenType {
            let key_words = vec![
                String::from("unsigned"),
                String::from("char")    ,
                String::from("short")   ,
                String::from("int")     , 
                String::from("long")    ,
                String::from("float")   , 
                String::from("double")  , 
                String::from("while")   ,
                String::from("if")      ,  
                String::from("return")  , 
                String::from("void")    , 
                String::from("main") 
            ];
    
            DistinguishTokenType{types : key_words}
        }
    

        pub fn get_next_type(&self, currentLexeme: &std::string::String, nextChar: Option<char>) -> TokenType {
            
            let tokenType : TokenType;
    
            if self.types.contains(&currentLexeme) {
                tokenType = TokenType::KEYWORD;
            }
            else {
                if nextChar.unwrap() == '(' {
                    tokenType = TokenType::FUNCTION;
                }
                else {
                    tokenType = TokenType::VARIABLE;
                }
    
            }
    
            tokenType
        }
        
    }

}


pub struct Lexer {
	token_vector : Vec<Token>,
    rowNumber    : usize,
    columnNumber : usize,
    cstream      : CharStream
}

impl Lexer {
    
    pub fn new(filename: &str) -> Lexer {
        
        Lexer {
            token_vector: Vec::new(),
            columnNumber: 1,
            rowNumber:    1,
            cstream: CharStream::new(filename)
        }
    }

    pub fn scan(&mut self) -> &Vec<Token> {

        let tokenHelper   = LexerHelper::DistinguishTokenType::new();

        let mut currentLexeme = String::from("");
        let mut tokenType;
        
		while self.cstream.more_available() {

			match self.cstream.peek_next_char() {
                Some(T) => {
					if T.is_alphabetic() {
                        
                        let mut nextIsAlphaNumeric = true;
                        let mut nextIsWhiteSpace   = false;
                        
						while !nextIsWhiteSpace && nextIsAlphaNumeric {
                            
                            currentLexeme.push(self.cstream.get_next_char().expect("Next Character Failed"));
							self.columnNumber += 1;
                            
                            nextIsWhiteSpace   = self.cstream.peek_next_char().is_some() 
                                                    && self.cstream.peek_next_char().unwrap().is_whitespace();
                            nextIsAlphaNumeric = self.cstream.peek_next_char().is_some() 
                                                    && self.cstream.peek_next_char().unwrap().is_alphanumeric();
						}
                        
						
						tokenType = tokenHelper.get_next_type(&currentLexeme, self.cstream.peek_next_char());
                        
						let token = Token::new(currentLexeme.to_string(), tokenType, self.rowNumber, self.columnNumber - currentLexeme.len());
						self.token_vector.push(token);
						currentLexeme.clear();
					}
                    
					else if T.is_numeric() {
                        let mut nextIsNumeric    = true;
                        let mut nextIsWhiteSpace = false;
                        let mut nextIsDecimal    = false;
                    
						while !nextIsWhiteSpace && nextIsNumeric || nextIsDecimal {
                            
                            currentLexeme.push(self.cstream.get_next_char().unwrap());
							self.columnNumber += 1;

                            nextIsDecimal    = self.cstream.peek_next_char().unwrap() == LexerHelper::DECIMAL_CONSTANT;
                            nextIsNumeric    = self.cstream.peek_next_char().is_some() 
                                                    && self.cstream.peek_next_char().unwrap().is_numeric();
                            nextIsWhiteSpace = self.cstream.peek_next_char().is_some() 
                                                    && self.cstream.peek_next_char().unwrap().is_whitespace();
						}

						if currentLexeme.contains(LexerHelper::DECIMAL_CONSTANT) {
                            tokenType = TokenType::FLOATCONSTANT;                            
						}
                        
						else {  
							tokenType = TokenType::INTCONSTANT;
						}

                        let token = Token::new(currentLexeme.to_string(), tokenType, self.rowNumber, self.columnNumber - currentLexeme.len());
                        self.token_vector.push(token);

						currentLexeme.clear();
					}
                    
					else if T.is_whitespace() {
						if T == '\n' {
                            self.rowNumber += 1;
						}
                        
						self.cstream.get_next_char();
						self.columnNumber += 1;
					}
                    
					else {
						tokenType = TokenType::OPERATOR;
						let token = Token::new(T.to_string(), tokenType, self.rowNumber, self.columnNumber - currentLexeme.len());
						self.token_vector.push(token);
						self.columnNumber += 1;
						self.cstream.get_next_char();
                        
					}
				}
				None => self.columnNumber += 1,
			}
		}
		
        &self.token_vector
	}
}





