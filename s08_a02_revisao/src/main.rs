/*
	Revisão de Move, Copy e Clone			[Revisão de Move, Copy e Clone]

Baseado no livro:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


/*
	Copy:	"Types whose values can be duplicated simply by copying bits."
			Significa "Este tipo pode ser duplicado, basta copiar os bytes".
			É uma cópia burra bit a bit.
			É mais rápido.
			Acionado pela atribuição '='.
			Tendo Copy automaticamente tem Clone, pois Copy é mais restritivo.

	Clone: 	"A common trait for the ability to explicitly duplicate an object."
			Significa "Este tipo pode ser duplicado".
			É uma duplicação inteligente de todos os componentes e subcomponentes.
			É mais lento que Copy.
			Acionado pelo método 'clone()'.
			Pode ter Clone mas não ter Copy.

	Exemplo: struct com campo String
			Clone da struct vai criar uma nova struct e duplicar tudo, inclusive texto da String.
			Copy da struct apenas copiaria um pointer para a mesma String, a mesma String duas structs.
*/


#[derive(Debug)]
struct Ponto {
    x: f64,
    y: f64,
}


#[derive(Debug, Copy, Clone)]
//#[derive(Debug, Copy)]
struct PontoComCopy {
    x: f64,
    y: f64,
}


#[derive(Debug, Clone)]
//#[derive(Debug, Copy, Clone)]
struct VecPontosComCopy {
	z: Vec<PontoComCopy>,		// Vec não suporta Copy
	k: String,					// String não suporta Copy	
}





fn main() {

	let sx = Ponto{ x:1.1, y:2.2};
	let sy = sx;			// struct foi movida de 'x' para 'y', logo a referência de 'x' não é mais válida
	println!("{:?}", sy);
	//println!("{:?}", sx);	 	// error: use of moved value


	let sx = PontoComCopy{ x:3.3, y:4.4};
	let sy = sx;	// struct foi copiada de 'x' para 'y', logo a referência de 'x' ainda é válida
	println!("{:?}", sy);
	println!("{:?}", sx);


	// Conceito de reborrow (reempréstimo)
	let mut iii = 1000;
	let borrow = &mut iii;

	let reborrow = &mut *borrow;		// reempréstimo
	*reborrow += 500;
	//*borrow += 44;
	println!("reborrow {reborrow}");

	*borrow += 44;
	println!("borrow {}", borrow);
	//println!("borrow {}", *borrow);

	println!("iii {iii}");


}



