/* Variáveis e Mutabilidade		[3.1. Variables and Mutability]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


//const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;

//const UMA_HORA_EM_SEGUNDOS = 1 * 60 * 60;				// pode ???

const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 600;		// pode sombrear ???


fn main() {
    const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;	// ESCOPO INTERNO
    println!("Inicio do programa");
    let mut x = 5;
    println!("O valor de x é: {x}");

    x = UMA_HORA_EM_SEGUNDOS;
    println!("O valor de x agora é: {x}");

    
}

