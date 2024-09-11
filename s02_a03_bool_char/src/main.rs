/* Tipos de Dados: Tipos Escalares		[3.2. Data Types: Scalar Types]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


fn main() {

	let t = true;
	let f: bool = false;

	let x = t && f;
	let y = t || !f;
	let z = 12 > 13;

	let cc = 'z';
	let _c = 'z';			// sublinha elimina o warning

	let z: char = 'ℤ';			// char type is four bytes and represents a Unicode Scalar Value

	println!("bool: {x}, char: {cc}");
}

