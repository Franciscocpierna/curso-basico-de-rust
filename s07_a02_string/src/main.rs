/*
	Strings			[8.2. Storing UTF-8 Encoded Text with Strings]


Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




fn main() {

	// Criando strings

	let s = String::new();
	println!("P1: s >>>{s}");
	
	let s = String::from("initial contents");
	println!("P2: s >>>{s}");

	let hello = String::from("Olá");			// strings usam UTF-8
	println!("P3: hello >>>{hello}");
	let hello = String::from("안녕하세요");
	println!("P4: hello >>>{hello}");

	let data = "conteúdo inicial";					// &str
	let s = data.to_string();
    println!("P5: s >>>{s}");

	let s = "outro conteúdo inicial".to_string();	// Diretamente com o string literal
	println!("P6: s >>>{s}");
  
   
	// Atualizando strings

	let mut s = String::from("foo");
	s.push_str("bar");
	println!("P7: s >>>{s}");


	let mut s1 = String::from("foo");
	let s2 = "bar";
	s1.push_str(s2);
	println!("P8: s1 >>>{s1}");
	println!("P9: s2 >>>{s2}");		// s2 continua disponível, push_str não virou dona de s2
	
	s1.push('l');
	println!("P10: s1 >>>{s1}");	


	// Concatenando strings

	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2;
	println!("P11: s3 >>>{s3}");
	//println!("P12: s1 >>>{s1}");				// o conteúdo de 's1' foi movido para s3 e não pode mais ser usada
	
	let mut s1 = String::from("Hello, ");
	s1.push_str(&s2);					// Altera o próprio 's1', 's2' foi apenas emprestado
	println!("P13: s1 >>>{s1}");
	
		
	let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{s1}-{s2}-{s3}");		// Melhor maneira, não tira a propriedade de 's1'
	println!("P14: s4 >>>{s4}");	


	// Indexando strings: não pode
	let s5 = String::from("hello");
	//let h = s5[0];


	// Slices de strings: Não é uma boa idéia quando podem haver caracteres não ASCII
	let hello = "Здравствуйте";
	let s6 = &hello[0..4];
	println!("P15: s6 >>>{s6}");


	// Iterando sobre strings
	// Obs: strings contém caracteres UTF-8, cujo tamanho varia de 1 até 4 bytes
	for c in "Здравствуйте".chars() {
		println!("P16: {c}");
	}

}

