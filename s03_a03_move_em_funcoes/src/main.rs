/* Ownership: Funções		[4.1. What is Ownership?]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




fn main() {
	let s = String::from("hello");

	recebe_ownership(s  );	// Propriedade do 'String' entregue para a função
										// s não é mais válido
	println!("{}", s );


	let x = 5;
	recebe_copia(x);		// x apenas copiado para a função
										// x continua válido
	println!("{}", x);
}	// s não é dono do String, então ele não é liberado da memória neste ponto


fn recebe_ownership(um_string: String) {
	println!("{}", um_string);
}
	// um_string sai fora de escopo
	// um_string fica inválido
	// ele é dono do 'String'
	// a memória do 'String' é liberada (drop)

		

fn recebe_copia(um_inteiro: i32) {
	println!("{}", um_inteiro);
}
	// um_inteiro sai fora de escopo
	// um_inteiro fica inválido
	// ele não é dono de ninguém, nenhum drop acontece


