/* Blocos e Sombreamento		[3.1. Variables and Mutability]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


fn main() {
    println!("Inicio do programa");
    const X:i32 = 5;
    let y = 6;
    let mut z = 7;
	z = z + 1;
    
    println!("No início os valores são: X={X}, y={y}, z={z}");
 
    {						// bloco interno
	    const X:i32 = 555;
	    let y = 666;
	    let mut z = 777;
		z = z + 1;
        println!("Dentro do bloco interno os valores são: X={X}, y={y}, z={z}");
    }
    
    println!("Depois do bloco interno os valores são: X={X}, y={y}, z={z}");
}


