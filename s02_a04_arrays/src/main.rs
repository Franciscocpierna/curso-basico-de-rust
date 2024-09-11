/* 	Tipos de Dados: Tipos Compostos		[3.2. Data Types: Compound Types]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


fn main() {
	let aa = [1, 2, 3, 4, 5];
	let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho",
	  "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

	let _bb: [i32; 5] = [1, 2, 3, 4, 5];
	
	let cc = [3; 5];	
	let dd = [3, 5];

	println!("cc {:?}", cc);
	println!("dd {:?}", dd);

	// Indexa começando pelo elemento 0
	println!("Elemento 2 do array 'meses' é: {:?}", meses[2]);
	
	let errado = cc[11];	// Erro de compilação, Pânico se detectado na execução

	println!("Hello, world!, {:?}", errado);
}



