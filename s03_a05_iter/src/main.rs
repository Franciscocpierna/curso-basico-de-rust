/* O Tipo Slice		[4.3. The Slice Type]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



fn main() {
	let s = String::from("alo mundo azul");

	let word = first_word(&s);

	println!("the first word is: {}", word);
}


// first_word retorna um 'string slice' &str
fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]		// String inteiro
}


