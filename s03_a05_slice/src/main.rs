/* O Tipo Slice		[4.3. The Slice Type]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




fn main() {

	// Slices são referências para uma sequência continua de elementos
	// Um slice é um tipo de referência
	
	let s = String::from("hello world");

    let s1 = &s[0..5];		// Tipo &str ou 'string slice'
    let s2 = &s[6..11];
    let s3 = &s[..2];
	let s4 = &s[3..];
  
    let slit = "Hello, world!";		// slit é do tipo &str ou 'string slice'
    
    println!("s1:{}, s2:{}, s3:{}, s4:{}, slit:{}", s1,s2,s3,s4,slit);
}

