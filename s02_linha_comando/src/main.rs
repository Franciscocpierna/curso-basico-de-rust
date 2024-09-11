/*
	Exemplo: Argumentos na Linha de Comando

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/


	Pega os argumentos da linha de comando. Por exemplo:

	$ cargo run -- arg1 arg2 arg3 etc

*/




use std::env;

fn main() {

	println!("\nTotal de elementos em env::args é {}", env::args().len());
	
	println!("\nPercorre usando iterador:");
	let mut i = 0;
	for x in env::args() {
		println!("Argumento [{}] == {}", i, x);
		i += 1;
	}


	println!("\nPercorre usando iterador com indices:");
	for (i,x) in env::args().enumerate() {
		println!("Com enumerate fica Argumento [{}] == {}", i, x);
	}


	println!("\nColoca tudo em um vector");
	let argumentos: Vec<String> = env::args().collect();

	println!("\nPercorre usando o vector:");
	for i in 0..argumentos.len() {
		println!("Com vector fica Argumento [{}] == {}", i, argumentos[i]);
	}
	println!("Total de {} elementos no vector", argumentos.len());

}



