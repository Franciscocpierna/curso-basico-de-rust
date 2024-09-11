/*
	Option			[6.2. The match Control Flow Construct]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



/*	Definido na standard library:
	<T> is a generic type parameter

	enum Option<T> {
		None,
		Some(T),
	}	
*/



// match: Os padrões indicados devem cobrir todas as possibilidades

fn somar_um(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}

fn somar_option(x: Option<i32>, y: Option<i32>) -> Option<i32> {
	match (x,y) {
		(Some(i),Some(j)) => Some(i+j),
		(Some(i), None) => None,
		(None, Some(j)) => None,
		(None, None) => None,
	}
}


fn main() {
	let numero5 = Some(5);		// Option<i32>
	let nao_numero: Option<i32> = None;

	let some_char = Some('e');	// Option<char>

	let x: i8 = 5;
	let y: Option<i8> = Some(5);

//////////////////////	let sum = x + y;
	// no implementation for `i8 + Option<i8>`
	// y pode ser Some, mas pode ser None, precisa tratar dos dois casos


	println!("numero5:  {:?}", numero5);
	println!("somar_um c/ numero5:  {:?}", somar_um(numero5));
	println!("...");

	println!("nao_numero:  {:?}", nao_numero);
	println!("somar_um c/ nao_numero:  {:?}", nao_numero);
	println!("...");

	println!("somar_option:  {:?}", somar_option(numero5,numero5));




}
