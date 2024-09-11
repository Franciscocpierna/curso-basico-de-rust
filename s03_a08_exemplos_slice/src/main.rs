/*
	s03_a08_exemplos_slice


	Baseado em:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



use std::io;


// Calcula tamanho mínimo, médio e máximo de uma lista de palavras
fn tamanho_palavras_v1() {
	println!("\n\ntamanho_palavras_v1()");

	// Vec<T> tem dados contíguos, pode acessar como array, pode variar o tamanho
	let mut lista_palavras = Vec::new();

	loop {
		println!("[V1] Digite uma palavra ou somente enter para terminar");
		let mut linha = String::new();
		io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");
		linha = linha.trim().to_string();
		//linha = linha.trim();		// trim() retorna slice
		if linha.len() == 0 {
			break;
		} else {
			println!("lido: {}", linha);	// println! não move (não invalida) 'linha'	
			lista_palavras.push( linha );
			//println!("lido: {}", linha);	// 'linha' foi movida (invalidada) por Vec::push()
		}
	}

	println!("Foram digitadas {} palavras", lista_palavras.len());
	for p in lista_palavras {
		println!("{}", p);
	}
}


// Calcula tamanho mínimo, médio e máximo de uma lista de palavras
// Pode ter várias palavras por linha
fn tamanho_palavras_v2() {
	println!("\n\ntamanho_palavras_v2()");

	let mut lista_palavras = Vec::new();
	//let mut lista_palavras: Vec<String> = Vec::new();		// Já deixa indicado o que quer

	loop {
		println!("[V2] Digite várias palavras ou somente enter para terminar");
		let mut linha = String::new();
		io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");
		linha = linha.trim().to_string();
		if linha.len() == 0 {
			break;
		} else {
			//let palavras = linha.split(" ");	// 'split' retorna iterador sobre slices de linha
			let palavras = linha.split_whitespace();		// Desconsidera vários espaços entre palavras

			for p in palavras {
				//lista_palavras.push( p.trim() );				// 'p' e 'palavras' destruídos antes de 'lista_palavras'
				//lista_palavras.push( p.trim().clone() );		// clone de slice é slice
				lista_palavras.push( p.trim().to_string() );	// cria String para colocar em 'lista_palavras'
			}
		}
	}
	println!("Foram digitadas {} palavras", lista_palavras.len());

	let mut minimo = 99999;
	let mut maximo = 0;
	let mut total = 0;
	//for p in lista_palavras {		// 'lista_palavras' movido por chamada implícita à '.into_iter()'
	for p in lista_palavras.iter() {
		let tam = p.chars().count();
		total += tam;
		if tam < minimo {
			minimo = tam;
		}
		if tam > maximo {
			maximo = tam;
		}
		println!("{}", p);
	}
	if lista_palavras.len() > 0 {
		println!("Minimo={}  médio={}  máximo={}", minimo, total/lista_palavras.len(), maximo);
	}
	
}



fn main() {
	tamanho_palavras_v1();
	tamanho_palavras_v2();
}



