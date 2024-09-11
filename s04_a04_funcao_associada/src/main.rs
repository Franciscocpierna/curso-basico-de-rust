/*
	Métodos			[5.3. Method Syntax]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
	// Função associada, retorna um novo Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

	// Método pois tem &self
    fn area(&self) -> u32 {				// self: &Self
    									// poderia ser '&mut self' ou 'self'
        self.width * self.height
    }
}


impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}




fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

	let sq = Rectangle::square(3);


    println!(
        "A área do retângulo rect1 é {} pixels quadrados.", rect1.area() );

    println!(
        "A área do retângulo sq é {} pixels quadrados.", sq.area() );
}


