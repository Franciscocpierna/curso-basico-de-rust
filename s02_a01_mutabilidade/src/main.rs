/* Variáveis e Mutabilidade		[3.1. Variables and Mutability]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


fn main() {
	println!("Inicio do programa");
	let mut x = 5;
	println!("O valor de x é: {x}");

	x = 6;					// pode ???

	let x = 666;		// pode???
	println!("O valor de x agora é: {x}");
	
	
	let mut y = 5;
	println!("O valor de y é: {y}");
	y = 6;		// pode ???
	println!("O valor de y agora é: {y}");
}

