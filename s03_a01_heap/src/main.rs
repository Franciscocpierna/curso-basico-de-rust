/* Ownership: Memória Heap			[4.1. What is Ownership?]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



fn main() {
	
	// Cria a variável mutável s3, ela fica no Stack pois tem tamanho fixo
	//		let mut s3 = 
	// s3 aponta para uma variável 'Tipo String'
	// Como variável 'Tipo String' pode mudar de tamanho durante a execução ela é alocada no Heap
	//                                                                            *******
	//		let mut s3 = String::
	// O conteúdo inicial da variável 'Tipo String' virá de um 'String literal' (tamanho inicial)
	//		let mut s3 = String::from

	// 'String literal alo' é copiado para a memória da variável 'Tipo String'
	let mut s3 = String::from("alo");

	println!("Valor de s3 é {s3}");
	
	
	// Podemos mudar o conteúdo da variável 'Tipo String'
	// push_str() acrescenta um 'String literal' ao final do conteúdo da variável 'Tipo String'
	s3.push_str(", mundo");

	// Mais memória foi alocada automaticamente para a variável 'Tipo String'
	//                  *******
	// s3 não muda de tamanho pois ela apenas aponta para a variável 'Tipo String'
	println!("Valor de s3 agora é {}", s3);


	// s3 é dona (owns) a variável 'Tipo String'
	// Como faço para liberar a memória usada pela variável 'Tipo String' ???
	//                *******
	// A memória é liberada automaticamente quando termina o escopo do seu dono

	{
		let s4 = String::from("alo alo alo alo");
		println!("Valor de s4 é {}", s4);
    }			// Tanto a memória de 's4' como do 'Tipo String' do qual ela é dona são liberadas
    			// Função 'drop' que faz isto é chamada automaticamente pelo compilador
}

