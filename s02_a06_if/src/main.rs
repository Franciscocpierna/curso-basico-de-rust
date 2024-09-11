/* Controle de Fluxo	[3.5. Control Flow]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




// Linha de comentário, existem outros para gerar documentação

/*
	Bloco de comentário
*/


fn main() {
	let number = 3;

	// Condição deve ser do tipo bool, não precisa parênteses
    if number < 5 {
        println!("condição é verdadeira");
    } else {
        println!("condição é falsa");
	}


	// Cascata de ifs
    if number % 4 == 0 {
        println!("número é divisível por 4");
    } else if number % 3 == 0 {
        println!("número é divisível por 3");
    } else if number % 2 == 0 {
        println!("número é divisível por 2");
    } else {
        println!("número não é divisível por 4, 3, ou 2");
    }


	// Pode usar como expressão
	let outro_number = if number == 0  { 0 } else { 99 };
	println!("O valor do outro_number é: {outro_number}");


	println!("Hello, world!");
}
