/*
	Tipos Genéricos		[10.1. Generic Data Types]

Baseado no livro:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Vale para qualquer 'T'
impl<T> Point<T> {
	fn x(&self) -> &T {
        &self.x
    }
}

// Vale somente para 'f32'
impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}




#[derive(Debug)]
struct PointDiferente<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> PointDiferente<X1, Y1> {
	fn mixup<X2, Y2>(self, other: PointDiferente<X2, Y2>) -> PointDiferente<X1, Y2> {
		PointDiferente {
            x: self.x,		// X1
            y: other.y,		// Y2
        }
    }
}





/*	Pode usar em enum também, na verdade já usamos

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/





fn main() {
    let inteiro = Point { x: 5, y: 10 };
    let flutuante = Point { x: 1.0, y: 4.0 };

	//let diferentex = Point { x: 5, y: 4.0 };
	let diferente = PointDiferente { x: 55, y: 44.0 };

	println!("inteiro {:?}", inteiro);
	println!("flutuante {:?}", flutuante);
	println!("diferente {:?}", diferente);

	// Métodos também podem usar genéricos
    println!("inteiro.x = {}", inteiro.x());

	// Métodos podem vales apenas para um 'T' específico
    //println!("distância da origem = {}", inteiro.distance_from_origin());
    println!("distância da origem = {}", flutuante.distance_from_origin());

	// Tipos podem ser diferentes na struct e no método
	let dif2 = PointDiferente { x: "Hello", y: 'c' };
    let dif3 = diferente.mixup(dif2);

    println!("dif3.x = {}, dif3.y = {}", dif3.x, dif3.y);

	//println!("diferente {:?}", diferente);	// Foi invalidado por mixup()
	//println!("diferente {:?}", dif2);			// Foi invalidado por mixup()


}




