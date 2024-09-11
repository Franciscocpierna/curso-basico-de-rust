/* O Tipo Slice		[4.3. The Slice Type]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




fn main() {
	let my_string = String::from("alo mundo azul");
	
	// Parâmetro pode ser um slice
	let word = first_word(&my_string[0..6]);
	println!("{word}");
	let word = first_word(&my_string[..]);
	println!("{word}");
	// Parâmetro pode ser referência para String
	let word = first_word(&my_string);
	println!("{word}");


	let my_string_literal = "hello world";

	// Parâmetro pode ser slice de 'String Literal'
	let word = first_word(&my_string_literal[0..6]);
	println!("{word}");
	let word = first_word(&my_string_literal[..]);
	println!("{word}");
	
	// Parâmetro pode ser um 'String Literal', pois ele equivale a um '&str'
	let word = first_word(my_string_literal);
	println!("{word}");
}
		
		
// Parâmetro de first_word não precisa ser &String, pode ser &str
fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}
	
	
	
