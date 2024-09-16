/*
	Erros Recuperáveis			[9.2. Recoverable Errors with Result]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




use std::fs::File;
use std::io::ErrorKind;


fn main() {

    let greeting_file_result = File::open("hello.txt");
	// File::open retorna Result<T, E>
	// T é std::fs::File					pode-se ler o arquivo com 'file.read_to_string' por exemplo
	// E é std::io::Error



	let greeting_file1 = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problema abrindo o arquivo: {:?}", error),
    };




/*	// Sem pânico
	if greeting_file_result.is_err() {
		println!("Problema abrindo o arquivo, mensagem: {}", greeting_file_result.unwrap_err());
	} else {
		let greeting_file2 = greeting_file_result.unwrap();   puxar para cima
		// lê o arquivo ...
	}
*/



/*
	let greeting_file3 = match greeting_file_result {
		Ok(file) => file,
		Err(error) => match error.kind() {

			ErrorKind::NotFound => match File::create("hello.txt") {
						Ok(fc) => fc,
						Err(e) => panic!("Problema criando o arquivo: {:?}", e),
			},

			other_error => {
						panic!("Problema abrindo o arquivo: {:?}", other_error);
			}

		},
	};

*/


	// 'unwrap' gera pânico no caso de erro
	let greeting_file4 = File::open("hello.txt").unwrap();


	// 'expect' gera pânico no caso de erro, com mensagem específica
	let greeting_file5 = File::open("hello.txt")
			.expect("hello.txt deveria ser incluída neste projeto");


}



