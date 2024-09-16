/*
crate root  (main.rs)
 ├── main()
 ├── outra_funcao()
 │
 ├── cooking	<<<<<<<<<<<<<<<<<<<<<<<<<-----
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



// Dá acesso às coisas públicas do módulo 'hosting'
use crate::hosting;

// Dá acesso especifico à 'Breakfast'
use crate::hosting::Breakfast;


// pub use crate::hosting::Breakfast;
// É possível re-exportar um tipo
// Pode facilitar para quem usa as funções deste módulo
// Quem usa este módulo teria agora  crate::cooking::Breakfast





// Cuidado com coisas que tem o mesmo nome mas ficam em módulos diferentes

//use std::fmt::Result;
//use std::io::Result;
// x: Result; 		qual deles ???


use std::fmt;
use std::io;
// x: fmt::Result;
// y: io::Result;



// Pode trocar o nome dos tipos no momento do 'use' usando 'as'
// use std::fmt::Result;
// use std::io::Result as IoResult;
// Mas em geral não é uma boa ideia









//pub fn prepare_food() -> String {
fn prepare_food() -> String {
	String::from("prepare_food")
}


// Chamadas através de caminhos
pub fn clean_up() -> String {
	
	// 'outra_funcao' não precisa ser publica pois está no módulo pai
	crate::outra_funcao();
	super::outra_funcao();
	
	// Caminho absoluto
	println!("Caminho absoluto: {}", crate::hosting::add_to_waitlist() );
	
	// Caminho relativo
	println!("Caminho relativo: {}", super::hosting::add_to_waitlist() );

	


	// Usando use
	let bb = Breakfast::new();

	println!("Usando use: {}", hosting::add_to_waitlist() );
	//println!("Usando use: {}", add_to_waitlist() );


	String::from("clean_up")


	
}


