/*	Ownership: Memória Heap			[4.1. What is Ownership?]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



fn main() {

	// Cria a variável imutável s1, ela fica no Stack pois tem tamanho fixo
	//					let s1 = 
	// O tipo de s1 é string slice ou uma referência para string

	// s1 aponta para um 'String literal'
	// 'String literal' é imutável, fica fixo no código
	//					"primeiro string literal"

	// s1 até poderia ser mutável e apontar para outra coisa

	let s1 = "primeiro string literal";

	{	// s2 não é válida nesta linha
		let s2 = "segundo string literal";	// s2 é válida a partir de agora
		println!("Valor de s1 é {s1}");
		println!("Valor de s2 é {s2}");
    }												// escopo termina, s2 não é mais válida

	println!("Valor de s1 é {s1}");
//	println!("Valor de s2 é {s2}");					// s2 não é válido


	println!("Hello, world!");
}


