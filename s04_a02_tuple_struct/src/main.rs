/*
	Definindo e instanciando Structs: Parte 2		[5.1. Defining and Instantiating Structs]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




// tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);


#[derive(Debug)]
struct Point(i32, i32, i32);


// unit-like structs, não tem campo nenhum
#[derive(Debug)]
struct AlwaysEqual;



struct User {
	active: bool,
	username: String,
	email1: String,			// User é dona de todos os seus campos
	email2: &str,			// User não é dona de email2, requer 'lifetime' (capítulo 10)
	sign_in_count: u64,
}



fn main() {

	let black = Color(0, 0, 0);
	let origem = Point(1, 2, 3);

	println!("origem: {:?}", origem);

	let novo: Color;
	novo = origem;    	// Tipo diferente

	let unit_struct = AlwaysEqual;
	println!("unit_struct: {:?}", unit_struct);
}
	
	
	
