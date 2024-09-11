/* Tipos de Dados: Tipos Compostos		[3.2. Data Types: Compound Types]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




fn main() {
	let tup1: (i32, f64, bool) = (500, 6.4, true);

	let tup2 = (500, 6.4, true);

	println!("Minha tupla tem: {:?}    {:?}", tup1, tup2);


	// desestruturação (destructuring) quebra a tupla em suas partes
	let (x1, y1, z1) = tup2;
	println!("Minha tupla tem: {x1} {y1} {z1}");

	// Pode acessar os campos usando indexadores
	println!("Minha tupla tem: {:?} {:?} {:?}", tup1.0, tup1.1, tup1.2);

	// Uma tupla vazia é chamada unit, representa um valor vazio
    println!("Tupla vazia: {:?}", () );

    println!("Hello, world!");
}

/*
 println!("Nome do Produto {}", produto.get_nome());
    println!("Preco do Produto: {}", produto.get_preco());
    println!("Quantidade do Produto: {}", produto.quantidade)

    produto.set_nome("novo papel");
    produto.set_preco(14.99);
    produto.set_quantidade(15);
    println!("novo nome  {}", produto.get_nome());
    println!("novo Preco: {}", produto.get_preco());
    println!("nova Quantidade: {}", produto.quantidade);


*/