/* Ownership: Semântica Move		[4.1. What is Ownership?]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




fn main() {

	// Semântica 'copy'
	let x = 5;
	let y = x;			// Valor '5' é copiado para y
	println!("y = {y}");
	println!("x = {x}");

	// Semântica copy disponível quando o tipo implementa o 'trait' Copy 
	//	Inteiros
	//	Booleanos
	//	Ponto flutuante
	//	Caracteres
	//	Tuplas e Arrays apenas com tipos que suportam Copy
	//		(i32,bool) sim
	//		(i32,String) não



	// Semântica 'move'
	let s1 = String::from("hello");
	let s2 = s1;	// Valor 'Tipo String' é movido para s2
							// s1 não é mais válida !!!
	println!("s2 = {s2}");
	//println!("s1 = {s1}");	// s1 não é válida



	// Ainda é possível fazer um clone
	let s3 = s2.clone();
	println!("s3 = {s3}");
	println!("s2 = {s2}");

}





/*
Copy:	"Types whose values can be duplicated simply by copying bits."
		É uma cópia burra bit a bit
		É mais rápido
		Acionado pela atribuição '='
 		Tendo Copy automaticamente tem Clone

Clone: 	"A common trait for the ability to explicitly duplicate an object."
		É uma duplicação inteligente de todos os componentes e subcomponentes
		É mais lento que Copy
		Acionado pelo método 'clone()'
		Pode ter Clone mas não ter Copy
*/
