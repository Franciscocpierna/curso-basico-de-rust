/* Blocos e Sombreamento		[3.1. Variables and Mutability]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


fn main() {
    println!("Inicio do programa");
    let x = 5;
    println!("O valor de x é: {x}");
    let x = x + 1;
    println!("O valor de x é: {x}");

    {
        let x = x * 2;
        println!("O valor de x no bloco interno é: {x}");
    }
	println!("O valor de x depois do bloco interno é: {x}");
  

    let spaces = "   ";
    let spaces = spaces.len();		// let cria nova variável com novo tipo
    println!("O valor de spaces é: {spaces}");

	let mut spaces2 = "   ";
    println!("O valor de spaces2 é: {spaces2}");
	spaces2 = "qwerty";						// mesma variável com o mesmo tipo
    println!("O valor de spaces2 é: {spaces2}");

	//spaces2 = 5; //spaces2.len();		// valor é mutável, o tipo não é mutável
    
}


