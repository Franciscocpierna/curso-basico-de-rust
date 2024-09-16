/*
	s07-a03-Hash Maps		[8.3. Storing Keys with Associated Values in Hash Maps]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


// O 'prelude' é uma lista de coisas que o Rust automaticamente importa para todos os programas
// 'Vec' e 'String' estão no 'prelude'
// 'HashMap' não está no 'prelude'
// https://doc.rust-lang.org/std/prelude/index.html

use std::collections::HashMap;



fn main() {

	let mut scores = HashMap::new();		//  HashMap<K, V>	key,value pegos do código abaixo
																//  HashMap<String, i32>

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);


	let team_name = String::from("Blue");
	let score = scores.get(&team_name);	// get retorna  Option<&V>

	match score {
		None => println!("P1: get--> {team_name} não tem score", ),
		Some(i) => println!("P1: get--> {team_name} tem score: {i}"),
	}


	// Outra forma possível:
	// get() retorna  Option<&V>
	// copied() transforma Option<&i32> em Option<i32>
	// unwrap_or() transforma None em zero, Option<i32> em 'i32'
	let score = scores.get(&team_name).copied().unwrap_or(0);
	println!("P2: get(team_name) score--> {score}");

	// Não funciona com Option<&String> pois valor precisa suportar 'Copy'
	let teste = Option::Some( &String::from("qwerty") );
	//println!("P3: teste copiado:  {:?}", teste.copied() );



	
	// Iterando cada par key/value no hash map
	for (key, value) in &scores {
		println!("P4: iterando--> {key}: {value}");
	}



	// Para tipos que implementam o trait 'Copy' (ex: i32), os valores são copiados para o hash map
	// Valores sem 'Copy' como 'String' são *movidos* e o hash map passa a ser o dono (owner) deles
	
	let nome_cor = String::from("Red");
	let numero = 10;
	scores.insert(nome_cor, numero);

	//println!("P5: nome_cor {nome_cor}");		// String associado com 'nome_cor' já foi movido para o hash map
	println!("P6: inseriu--> numero {numero}");
	
	// Se fosse inserida uma referência '&nome_cor' no hash map o valor não seria movido para o hash map
	// Mas existem outros aspectos descritos em 'Validating References with Lifetimes' in Chapter 10
	
	
	
	
	// Para atualizar o hash map existem várias formas:

	// Substituir o valor velho pelo novo
	scores.insert(String::from("Blue"), 25);
	println!("P7: {:?}", scores);


	// Só incluir o novo valor se a chave ainda não existia
	// Método entry de hash map retorna uma enum 'Entry'
	// enum 'Entry' tem um método 'or_insert'
	let x = scores.entry(String::from("Yellow")).or_insert(50);
	let y = scores.entry(String::from("Black")).or_insert(999);
	println!("P8: {:?}", scores);


	// Combinar o valor novo com o valor velho
	let text = "hello world wonderful world";
	for word in text.split_whitespace() {
		let ref_entrada = scores.entry(word.to_string()).or_insert(0);
		// 'entry' retorna uma referência mutável para o valor da entrada, que pode ser alterado (mutável)
		// Se ref_entrada fosse 'mut i32' era só colocar '+= 1'
		// Mas ref_entrada é '&mut i32'
		// Precisa dizer para alterar o valor referenciado e não o próprio ref_entrada
		*ref_entrada += 1;
	}
	println!("P9: entry--> {:?}", scores);


	// Alterar valor existente de forma simples
	let valor = scores.get_mut("hello");
	match valor {
		None => (),
		Some(x) => *x += 100,
	}
	println!("P10: get_mut--> {:?}", scores);





	// Remover uma entrada
	scores.remove("Red");
	println!("P11: remove--> {:?}", scores);



}

