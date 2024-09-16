/*
	Vectors			[8.1. Storing Lists of Values with Vectors]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



fn main() {

	// Como criar um Vector de i32

	let _vi1: Vec<i32> = Vec::new();

	let mut vi2 = Vec::new();
	vi2.push(90);
	vi2.push(91);
	vi2.push(92);
	vi2.push(93);

	let _vi3 = vec![90, 91, 92, 93, 94];

	// Como criar um Vector de &str

	let _vss1: Vec<&str> = Vec::new();

	let mut vss2 = Vec::new();
	vss2.push("aaaa");
	vss2.push("bbbb");
	vss2.push("cccc");
	vss2.push("dddd");

	let _vss3 = vec!["aaaa", "bbbb", "cccc", "dddd"];

	// Como criar um Vector de String

	let _vs1: Vec<String> = Vec::new();

	let mut vs2 = Vec::new();
	vs2.push( String::from("AAAA") );
	vs2.push( String::from("BBBB") );
	vs2.push( String::from("CCCC") );
	vs2.push( String::from("DDDD") );
	
	let _vs3 = vec![String::from("AAAA"), String::from("BBBB"), String::from("CCCC"), String::from("DDDD")];

	
	// Como acessar um elemento do Vector

	let x = vi2[2];
	let y = vss2[2];
	//let z = vs2[2];					// Somente para tipos com semântica COPY ou como referência/empréstimo
	println!("x {x}   y {y}   z não");

	let xx = &vi2[2];
	let yy = &vss2[2];		// Referência para referẽncia é resolvido automaticamente
	let zz = &vs2[2];
	println!("xx {xx}   yy {yy}   zz {zz}");

	let xxx = vi2.get(2);
	let yyy = vss2.get(2);
	let zzz = vs2.get(2);

	match xxx {
		Some(valor) => println!("xxx {valor}"),
		None => println!("Não existe xxx"),
	}
	match yyy {
		Some(valor) => println!("yyy {valor}"),
		None => println!("Não existe yyy"),
	}
	match zzz {
		Some(valor) => println!("zzz {valor}"),
		None => println!("Não existe zzz"),
	}

	// Se a posição não existir ?
	// let nao_existe = &vs2[100];				Erro de execução
	// let nao_existe = vs2.get(100);			Retorna None




	// Enorme quantidade de métodos em 'https://doc.rust-lang.org/std/vec/struct.Vec.html'
	println!("Vector vs2 tem {} elementos", vs2.len());

	let ultimo = vs2.last();
	match ultimo {
		None => println!("ultimo está vazio"),
		Some(x) => println!("O último elemento eh {x}"),
	}
	

	// Borrow checker fiscaliza o Vector como um todo
	let mut v4 = vec![80, 81, 82, 83, 84];
	let first = &v4[0];
	// Não pode alterar 'v4' pois ele foi emprestado para 'first'
	//v4.push(66);			
	println!("The first element is: {first}");
	// Aqui já pode alterar 'v4' pois ele foi "devolvido" na linha anterior
	v4.push(66);



	// Como iterar sobre os elementos de um Vector
	println!("Iterar sem alterar");
	for i in &vs2 {	// i assume cada elemento de 'vs2'
								// i é do tipo '&String', referência para String
								// Estamos pegando o Vector de 'vs2' emprestado, 'vs2' continua dono (owner) do Vector
		println!("i {i}");
	}
	println!("Iterar alterando");
	for i in &mut vs2 {// i assume cada elemento de 'vs2'
									// i é do tipo '&mut String', referência para String mutável
									// Estamos pegando o Vector de 'vs2' emprestado, 'vs2' continua dono (owner) do Vector
		i.push_str("ZZ");	// Altera o elemento
		println!("i {i}");
	}
	println!("Iterar alterando no caso de inteiros");
	println!("v4 antes de +9000   {:?}", v4);
	for i in &mut v4 {
		*i += 9000;				// Dereference operator '*' no Capítulo 15
								// println! lida com '&' e '*' automaticamente
	}
	println!("v4 depois de +9000   {:?}", v4);


	// Vector de Enum
	#[derive(Debug)]
	enum Celula {
	    MeuInteiro(i32),
	    MeuFloat(f64),
	    MeuTexto(String),
	}

	let mut linha = vec![
		Celula::MeuInteiro(3),
	    Celula::MeuFloat(10.12),
		Celula::MeuTexto(String::from("blue")),
	];

	println!("\nElemento 1 do Vector linha é:   {:?}", linha[1]);


	// Ordenação
	println!("\nantes da ordenação--> {:?}", v4);
	v4.sort();
	v4.sort_unstable();		// Mais rápido, mas não preserva a ordem dos iguais
	println!("depois da ordenação--> {:?}", v4);

	println!("\nantes da ordenação--> {:?}", linha);
	// linha ???
	//linha.sort_unstable();		// Mais rápido, mas não preserva a ordem dos iguais
	
	println!("depois da ordenação--> {:?}", linha);


	// Retirar elemento do fim do Vector
	println!("Retirar elemento do fim do Vector");
	let x = linha.pop();
	match x {
		None => println!("Vector estava vazio."),
		Some(i) => println!("Elemento retirado do Vector é:   {:?}", i),
	}
	println!("Vector ficou com {:?} elementos", linha.len());

	// Retirar um elemento qualquer preservando a ordem
	println!("Retirar elemento do meio preservando a ordem");
	println!("antes--> {:?}", v4);
	v4.remove(2);
	println!("depois do remove--> {:?}", v4);


	// Retirar um elemento qualquer sem preservar a ordem
	println!("antes--> {:?}", v4);
	v4.swap_remove(2);
	println!("depois do remove--> {:?}", v4);


	// Caso elementos sejam removidos do início do 'Vec', 'VecDeque' é mais eficiente que 'Vec'
	// https://doc.rust-lang.org/std/collections/struct.VecDeque.html


}

