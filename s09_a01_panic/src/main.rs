/*
	Erros Irrecuperáveis		[9.1. Unrecoverable Errors with panic!]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/





fn main() {

	let mut v = Vec::new();
	v.push(1);
	v.push(2);
	v.push(3);

	let y = 100;

	let x = v[y];
	//thread 'main' panicked at src/main.rs:25:14:
	//index out of bounds: the len is 3 but the index is 100
	//note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

	//Um backtrace é uma lista de todas as funções chamadas para chegar neste ponto do programa



	if y > v.len() {
		panic!("Entrei em pânico !!!");
	}
	//thread 'main' panicked at src/main.rs:35:9:
	//Entrei em pânico !!!
	//note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
	

	

	// Por default, quando um 'panic' acontece, todos os dados são liberados
	// Pode escolher 'abortar imediatamente', para reduzir memória e tempo de execução, em 'Cargo.toml':
	// [profile.release]
	// panic = 'abort'
	


}

