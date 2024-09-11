/*
	Usando Structs				[5.2. An Example Program Using Structs]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




// #[derive(Debug)] antes da struct torna 'Debug' disponível
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



fn main() {

	let scale = 2;

	let rect1 = Rectangle {
		width: dbg!(30 * scale),
		height: 50,
	};

    println!(
		"A área do retângulo é {} pixels quadrados.",
		area(&rect1)
    );


//	println!("rect1 é {}", rect1);			// Rectangle precisa ter a característica (trait) 'std::fmt::Display'

	println!("rect1 é {:?}", rect1);		// Rectangle precisa ter a característica (trait) 'std::fmt::Debug'
	println!("rect1 é {:#?}", rect1);		// Rectangle precisa ter a característica (trait) 'std::fmt::Debug'

	println!("\n\nchamando area2 ...\n");
	let r2 = area2(&rect1);
	println!("r2 vale {r2}");
}


fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}

 
fn area2(rectangle: &Rectangle) -> u32 {
	dbg!(&rectangle);							// Útil para depurar
	dbg!(rectangle.width * rectangle.height)	// Útil para depurar, aqui é o retorno da função
}




