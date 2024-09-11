/* Referências e Empréstimos		[4.2. References and Borrowing]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




fn main() {
    let palavra = String::from("abacaxi");

	// Move: passa a propriedade do 'String'
	let len1 = calcula_tamanho_move( palavra.clone() );
    println!("O tamanho1 de '{}' é {}.", palavra, len1);

	// Referencia: não passa a propriedade do 'String'
	let len2 = calcula_tamanho_referencia(&palavra);
    println!("O tamanho2 de '{}' é {}.", palavra, len2);

	// É a mesma coisa que na linguagem C ?
	let x = 11;
	soma_900(&x);
	soma_900(&22);
}



// A propriedade do 'String' é recebida pela função
fn calcula_tamanho_move(s: String) -> usize {
	s.len()
}
// s fica inválido
// s tinha a propriedade do 'String', drop do 'String'



// Um empréstimo do 'String' é recebido pela função, e não a propriedade do 'String'
fn calcula_tamanho_referencia(s: &String) -> usize {
	s.len()
}
// s fica inválido
// s não tinha a propriedade do 'String', nenhum drop acontece



// Referência (empréstimo) não é a mesma coisa que endereço '&' em C
fn soma_900( ref_int: &i32) {
	let c_a = *ref_int + 900;
	let s_a = ref_int + 900;
	println!("com asterisco {}      sem asterisco {}", c_a, s_a);
}



