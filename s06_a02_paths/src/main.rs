/*	s06-a02-Paths				[7.4. Bringing Paths Into Scope with the use Keyword]
								[7.5. Separating Modules into Different Files]


Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




/*
crate root  (main.rs)	<<<<<<<<<<<<<<<<<<<<<<<<<-----
 ├── main()
 ├── outra_funcao()
 │
 ├── cooking
 │   ├── prepare_food()
 │   └── clean_up()
 │
 ├── hosting
 │   ├── add_to_waitlist()
 │   └── seat_at_table()
 │
 └── serving
     ├── take_order()
     ├── serve_order()
     ├── take_payment()
	 └── clean_up()
*/


// Precisa alterar o 'Cargo.toml' colocando o pacote 'rand = "0.8.5"' nas dependencias
// Cargo faz download do pacote e suas dependências a partir de 'crates.io'
// A comunidade Rust disponibiliza vários pacotes em 'https://crates.io/'
use rand::Rng;


// 'std' é sempre incluído automaticamente, não precisa alterar 'Cargo.toml'
use std::collections::HashMap;


// Possível importar todos os itens do módulo para este contexto com o 'glob operator'
// Não é uma boa idéia
//use std::collections::*;


// Possível juntar vários em uma mesma linha
//use std::cmp::Ordering;
//use std::io;
//use std;
use std::{cmp::Ordering, io, self};






mod cooking;

mod hosting;

mod serving;




fn outra_funcao() -> String {
	String::from("outra_funcao")
}



fn main() {
	println!("Função deste módulo: {}", outra_funcao() );

	println!("cooking::clean_up:  {}", cooking::clean_up() );
	println!("serving::clean_up:  {}", serving::clean_up() );


	println!("Número aleatório: {}", rand::thread_rng().gen_range(1..=100) );


}
