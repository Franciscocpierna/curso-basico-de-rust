/*
	Propagação de Erros			[9.2. Recoverable Errors with Result]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/





use std::fs::File;
use std::io::{self, Read};


// Retorna io::Error como conteúdo do Result::Err pois vem de 'File::open' e 'read_to_string'
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),		// 'return' necessário
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => return Ok(username),
        Err(e) => return Err(e),  		// 'return' opcional aqui
    }
}



// Operador '?', propaga Err se for o caso, normalmente retorna o conteúdo de Ok
// Operador '?' pode ser usado apenas quando o tipo do retorno da função for compatível com Err(E) recebido por '?'
// Operador '?' pode converter o tipo de erro no retorno, mas requer 'Traits' (capítulo 10)
fn another_read_username_from_file() -> Result<String, io::Error> {
	let mut username_file = File::open("hello.txt")?;
	let mut username = String::new();
	let n_bytes = username_file.read_to_string(&mut username)?;
	Ok(username)
}



// Forma compacta
fn compacto_read_username_from_file() -> Result<String, io::Error> {
	let mut username = String::new();
	File::open("hello.txt")?.read_to_string(&mut username)?;
	Ok(username)
}
		

// Existe esta função pronta na biblioteca: 
// std::fs::read_to_string("hello.txt")
// crate::módulo::função
// https://doc.rust-lang.org/std/fs/fn.read_to_string.html

use std::fs;

fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}





// Pode usar também com 'Option'
// Encontra o último caracter na primeira linha de um texto
fn last_char_of_first_line(text: &str) -> Option<char> {
    text
		.lines()
		.next()?
		.chars()
		.last()
}




fn main() {

	let username = match read_username_from_file() {
		Ok(username) => username,
		Err(_) => "guest".to_string(),
    };
	println!("Username é '{}'", username);


	let username = match another_read_username_from_file() {
		Ok(username) => username,
		Err(_) => "guest".to_string(),
    };
	println!("Username é '{}'", username);


	let username = match compacto_read_username_from_file() {
		Ok(username) => username,
		Err(_) => "guest".to_string(),
    };
	println!("Username é '{}'", username);

	// Close automático quando 'File' sai do escopo

	let x = last_char_of_first_line("abcde\nqwerty\n");
	println!("caracter: {}", x.unwrap());
	let x = last_char_of_first_line("");
	println!("caracter: {}", x.unwrap());


}

