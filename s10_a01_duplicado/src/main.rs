/*
	Código Duplicado 					[10. Generic Types, Traits, and Lifetimes]


Baseado em:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



// Parâmetro é um slice de valores i32
fn largest(list: &[i32]) -> &i32 {
	let mut largest = &list[0];

	for item in list {
		if item > largest {
		//if *item > *largest {
			largest = item;
        }
    }

	largest
}




fn main() {

	// Uma função evita a duplicação de código
	let vector1_numeros = vec![34, 50, 25, 100, 65];
	let maior1 = largest(&vector1_numeros);
	println!("P1: O maior número é {}", maior1);

	let vector2_numeros = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	let maior2 = largest(&vector2_numeros);
	println!("P2: O maior número é {}", maior2);


	// Array tem similaridade com vector
	let array3_numeros = [1, 2, 3, 4, 5];
	let maior3 = largest(&array3_numeros);
	println!("P3: O maior número é {}", maior3);


	// A função não funciona com char, aqui vai entrar o Genérico mais adiante
	let array4_char = ['a', 'b', 'c', 'd', 'e'];
	let maior4 = largest(&array4_char);
	println!("P4: O maior caracter é {}", maior4);
    
    
	// A função não funciona com char, aqui vai entrar o Genérico mais adiante
	let vector5_char = vec!['a', 'b', 'c', 'd', 'e'];
	let maior5 = largest(&vector5_char);
	println!("P5: O maior caracter é {}", maior5);


}	




