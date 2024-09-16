/*
	Programando um Jogo		[2. Programming a Guessing Game]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/
*/





// O 'prelude' disponibiliza um conjunto de itens da biblioteca padrão automaticamente (ex: String)
// Para itens fora do 'prelude' é necessário declarar com 'use'
use std::cmp::Ordering;
use std::io;


use rand::Rng;			// Este não é da biblioteca padrão, requer [dependencies] rand = "0.8.5"


fn main() {

	println!("Guess the number!");
	
	// rand::thread_rng() retorna uma struct ThreadRgn que descreve um gerador de números aleatórios
	// gen_range() é um método da struct ThreadRgn que gera números aleatórios
	let secret_number = rand::thread_rng().
							gen_range(1..=100);
	
	loop {
		println!("Please input your guess.");
	
		let mut guess = String::new();
	
		// Módulo std::io tem muita coisa
		// https://doc.rust-lang.org/std/io/index.html
		// io::stdin() é uma função que retorna uma estrutura Stdin, um novo handle para a entrada padrão
		// io::stdin().read_line() é um método que lê uma linha e anexa a um String, retorna o enum Result
		// Result pode ser Ok(T) ou Err(E)
		// expect() é método de Result que retorna o valor do Ok ou pânico no caso de Err 
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

	
		// guess é String
		// trim() é método de String, remove espaços do início e do fim
		// parse() é método de String, transforma o string em outro tipo, neste caso 'u32' (compilador pega isto)
		// parse() retorna o enum Result
		// 'continue' inicia nova iteração do 'loop'
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("You guessed: {guess}");
	

		// guess é u32
		// cmp() é método de vários tipo que compara o self com outro valor
		// cmp() retorna o enum Ordering, com suas variantes 'Less', 'Greater' e 'Equal'
		// 'break' interrompe o 'loop'
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}

}

