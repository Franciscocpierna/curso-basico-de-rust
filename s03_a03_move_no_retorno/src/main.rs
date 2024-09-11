/* Ownership: Funções		[4.1. What is Ownership?]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




fn main() {
	let s1 = devolve_ownership();		// Valor de retorno é movido para s1

	let mut s2 = String::from("hello");
	let s3 = recebe_e_devolve_ownership(s2);	// s2 é movido para a função, s2 fica inválido
															// O valor de retorno da função é movido para s3


	s2 = String::from("bbbbbbbbbbb ello");


	println!("s1:{} s2:{} s3:{}", &s1, &s2, &s3);

	println!("s1:{} s2:{} s3:{}", &s1, &s2, &s3);


}
// s1 fica inválido, sua propriedade sofre um drop
// s2 fica inválido, mas não é dono de nada
// s3 fica inválido, sua propriedade sofre um drop



fn devolve_ownership() -> String {
	let algo = String::from("aaaa");
	algo		// Conteúdo de 'algo' é retornado para a função chamadora, semântica 'move' pois é um 'String'
				// Variável 'algo' fica inválida
}



fn recebe_e_devolve_ownership(um_string: String) -> String {
	println!("{}", um_string);
	um_string	// Conteúdo de 'um_string' é retornado para a função chamadora, semântica 'move' pois é um 'String'
				// Variável 'um_string' fica inválida
}		



/* Em resumo:

	Each value in Rust has an owner.
	There can only be one owner at a time.
	When the owner goes out of scope, the value will be dropped.
*/	


