/*
	Conta Palavras em um Arquivo de Texto - versão 2


Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


use std::process;
use std::env;

use std::fs::File;
use std::io::{BufReader,BufRead};

use std::collections::HashMap;



struct Teste {
	#[allow(dead_code)]			// Só afeta a estrutura
	caixa_alta: String,
	conta: u32,
}
impl Teste {
	fn new(caixa: &String, conta: u32) -> Teste {
		Teste {
			caixa_alta: caixa.clone().to_uppercase(),
			conta,
		}
	}
}


fn main() {

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("Uso: s07_a06_lendo_arquivo_txt <nome do arquivo>");
		process::exit(1);
	}

	// Abre o arquivo txt para leitura
	let mut reader: BufReader<File>;
	println!("Abrindo {} ...", args[1]);
	if let Ok(file) = File::open(&args[1]) {
		reader = BufReader::new(file);
	} else {
		println!("Arquivo não encontrado: {}", args[1]);
		process::exit(1);
	}

	// Cria uma tabela hash
	let mut contagem: HashMap<String, Teste> = HashMap::new();


	// Le o arquivo e insere na tabela hash
	loop {
		let mut linha_lida = String::new();
		let n_bytes = reader.read_line(&mut linha_lida).expect("Não conseguiu ler o arquivo");

		if n_bytes == 0 {
			// Terminou o arquivo
			break;
		} else {
			// Leu mais uma linha
			print!("{}", linha_lida);

			// Desconsidera vários espaços entre palavras
			let palavras = linha_lida.split_whitespace();
			for p in palavras {
				let mut palavra = String::from(p);

				let ultimo_char = palavra.chars().last().unwrap();
				// palavra.ends_with('.')
				match ultimo_char {
					'.' | ',' | ';' | ':' | '?' | '!' => palavra.pop().unwrap(),
					outras => outras,
				};
				println!("{}", palavra);
				if palavra.is_empty() {
					continue;
				}

				// Insere na tabela versão 1
				match contagem.get_mut(&palavra) {
					Some(teste) => teste.conta += 1,
					None => { contagem.insert(palavra.clone(), Teste::new(&palavra,1)); }
				};


				// Insere na tabela versão 2 (operação mais cara)
				match contagem.get(&palavra) {
					Some(teste) =>
						contagem.insert(palavra.clone(), Teste::new(&palavra,teste.conta + 1)),
					None =>
						contagem.insert(palavra.clone(), Teste::new(&palavra,1)),
				};


				// Insere na tabela versão 3 (operação mais cara)
				if let Some(teste) = contagem.get(&palavra) {
					contagem.insert(palavra.clone(), Teste::new(&palavra,teste.conta + 1));
				} else {
					contagem.insert(palavra.clone(), Teste::new(&palavra,1));
				};


				// Insere na tabela versão 4
				// entry() -> Acessa a entrada do HashMap associada com a chave fornecida
				// para "in-place manipulation".
				contagem
					.entry(palavra.clone())
					.or_insert(Teste::new(&palavra,0))
					.conta += 1;


				// Insere na tabela versão 5
				// entry() -> Acessa a entrada do HashMap associada com a chave fornecida 
				// para "in-place manipulation".
				contagem
					.entry(palavra.clone())
					.and_modify(|teste| teste.conta += 1)
					.or_insert(Teste::new(&palavra,1));
			}

		}	
	}

	// Mostra o conteúdo da tabela na tela
	let mut todos1 = Vec::new();
	for (key, teste) in contagem.iter() {
		todos1.push( (key,teste.conta));
	}
	todos1.sort_by(|a, b| b.1.cmp(&a.1));
	for x in todos1.iter() {
		println!("{} {}", x.1, x.0);
	}

	
	// Versão alternativa
	let mut todos2:Vec<(&String,&u32)> =
				contagem
					.iter()
					.map(|teste| (teste.0,&teste.1.conta))
					.collect();
	todos2.sort_by(|a, b| b.1.cmp(a.1));
	for x in todos2.iter() {
		println!("{} {}", x.1, x.0);
	}


}



