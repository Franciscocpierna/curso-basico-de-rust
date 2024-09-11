/*
	s05_a07_numeros_teclado


	Baseado em:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



use std::io;


fn main() {

	loop {
		println!("Digite <número> <operador> <número> ou somente enter para terminar");
		let mut linha = String::new();
		io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");
		linha = linha.trim().to_string();
		if linha.len() == 0 {
			break;
		} else {
			// Desconsidera vários espaços entre palavras
			let mut palavras = linha.split_whitespace();
			let op1_str = palavras.next().unwrap_or("");
			let operador = palavras.next().unwrap_or("");	
			let op2_str = palavras.next().unwrap_or("");
			println!("{} {} {}", operador, op1_str, op2_str);			

			let op1_res = op1_str.parse::<f64>();
			let op2_res = op2_str.parse::<f64>();
			if op1_res.is_err() {
				println!("Não entendi o operando: {}", op1_str);
			} else if op2_res.is_err() {
				println!("Não entendi o operando: {}", op2_str);
			} else {
				let op1 = op1_res.unwrap();
				let op2 = op2_res.unwrap();
				match operador {
					"+" => println!("{}  {}  {}  ==  {}", op1, operador, op2, op1+op2),
					"-" => println!("{}  {}  {}  ==  {}", op1, operador, op2, op1-op2),
					"*" => println!("{}  {}  {}  ==  {}", op1, operador, op2, op1*op2),
					"/" => println!("{}  {}  {}  ==  {}", op1, operador, op2, op1/op2),
					"%" => println!("{}  {}  {}  ==  {:.5}", op1, operador, op2, op1%op2),	// 111.1 % 10
					_ => println!("Não entendi o operador: {}", operador),
				}
			}	
		}
	}
}

