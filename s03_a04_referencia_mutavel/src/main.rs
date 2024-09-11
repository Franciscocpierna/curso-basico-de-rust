/* 	Referências e empréstimos		[4.2. References and Borrowing]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/





fn main() {
	let s = String::from("hello");
	change1(&s);
	
	let mut x = String::from("hello");
	change2(&mut x);
}


//some_string é imutável
fn change1(some_string: &String) {
	some_string.push_str(", world");	// imutável
}


//some_string é mutável
fn change2(some_string: &mut String) {
	some_string.push_str(", world");
	println!("{}", some_string);
}


// Várias referências imutáveis simultâneas para o mesmo valor é aceito
// Mas a cada momento pode haver APENAS UMA referência mutável para um valor
// A existência de uma referência mutável impede a existência de outras referências de qualquer tipo
fn nao_funciona() {
	let mut y = String::from("hello");

	// ry1 deve sair do escopo antes de ry2 ser criada
	let ry1 = &y;
	let ry2 = &y;
	println!("ry1 referencia {}", ry1);
	println!("ry2 referencia {}", ry2);

	let ry3 = &mut y;		// empréstimo de ry1 e ry2 já caducaram
	println!("ry3 referencia {}", ry3);
	
}


