/* Controle de Fluxo	[3.5. Control Flow]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


fn main() {

	let mut number = 3;
	

	println!("\n   Usando while");
	while number != 0 {
		println!("while {number}");
		number -= 1; 
	}
    

	println!("\n   Usando for");
	let aaa = [10, 20, 30, 40, 50];
	
	for elemento in aaa {
		println!("o valor é: {elemento}");
	}

	println!("\n   Usando Range");

	for number in 1..=4 {
        println!("com = {number}");
    }

	println!("\n   Usando Range Reverso");

	for number in (1..4).rev() {
        println!("reverso {number}");
    }

}
