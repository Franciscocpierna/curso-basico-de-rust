/*
	s07-a05		BTreeMap

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/


https://doc.rust-lang.org/std/collections/index.html

Use a HashMap when:
    You want to associate arbitrary keys with an arbitrary value.
    You want a cache.
    You want a map, with no extra functionality.

Use a BTreeMap when:
    You want a map sorted by its keys.
    You want to be able to get a range of entries on-demand.
    You’re interested in what the smallest or largest key-value pair is.
    You want to find the largest or smallest key that is smaller or larger than something.

*/


use std::collections::{HashMap,BTreeMap};


fn main() {
	let mut turma_hash = HashMap::new();
	let mut turma_btree = BTreeMap::new();

	let nomes = vec!["Ana","Beatriz","Claudio","Daniel","Ernesto","Flávia",
								"Geraldo","Rômulo","Remo","Saionara"];
	let notas = vec![0,1,2,3,4,5,6,7,8,9];
	for i in 0..10 {
		// Insere no HashMap
		turma_hash.insert(nomes[i], notas[i]);
		// Insere no BTreeMap
		turma_btree.insert(nomes[i], notas[i]);
	}


	// Pesquisa
	println!("\nPesquisa");
	let aluno = "Daniel";

	match turma_hash.get(&aluno) {
		Some(n) => println!("HashMap tem {} com nota {}", aluno, n),
		None => println!("HashMap não tem nota para {}", aluno),
	}

	match turma_btree.get(&aluno) {
		Some(n) => println!("BTreeMap tem {} com nota {}", aluno, n),
		None => println!("BTreeMap não tem nota para {}", aluno),
	}


	println!("\nIteração com HashMap");
	for (nome, nota) in &turma_hash {
		println!("iterando--> {} --> {}", nome, nota);
	}

	println!("\nIteração com BTreeMap");
	for (nome, nota) in &turma_btree {
		println!("BTreeMap iterando--> {} --> {}", nome, nota);
	}


	// Ambos tem:	remove()	contains_key()		len()

	// Só BTreeMap tem:
	println!("\nIteração com BTreeMap e um intervalo");

	let intervalo = turma_btree.range("R"..="S"); // "Saulo" não entra
	//let intervalo = turma_hash.range("R"..="S");

	for (nome, nota) in intervalo {
		println!("iterando com range() --> {} --> {}", nome, nota);
	}

	//for (nome, nota) in turma_btree.range("R"..="S") {
	//	println!("iterando--> {} --> {}", nome, nota);
	//}

}

