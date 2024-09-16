/*		Packages, Crates, Modules		[7.1. Packages and Crates]
										[7.2. Defining Modules to Control Scope and Privacy]
										[7.3. Paths for Referring to an Item in the Module Tree]


Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/
*/






/*
	Cargo Workspace:
		Conjunto de pacotes inter-relacionados (capítulo 14).

	Package:
		Pode conter vários 'crates binários' e opcionalmente apenas um 'crate biblioteca'.
		Descrito pelo arquivo Cargo.toml.

	Crate:
		Uma árvore de módulos que produzem um executável ou uma biblioteca.
		
	Módulo:
		Uma parte do programa cujos componentes (funções, structs, enums, constantes, etc)
		cooperam para o mesmo propósito (forte coesão).
		
	NESTE CURSO BÁSICO DE RUST TEMOS:
		Um package, composto por um crate binário, composto por vários módulos.

*/





/*
crate root  (main.rs)		<<<<<<<<<<<<<<<<<<<<<<<<<-----
 ├── main()
 ├── outra_funcao()
 │
 └── front_of_house
     ├── init_front_of_house()
     ├── chamadas_caminhos()
     │
	 ├── hosting
     │   ├── add_to_waitlist()
     │   └── seat_at_table()
     │
     └── serving
         ├── take_order()
         ├── serve_order()
         └── take_payment()
*/



mod front_of_house;			// Declaração do módulo 'front_of_house'
							// crate::front_of_house::coisas_do_front_of_house
							// Código pode estar:
							//		Inline: mod front_of_house {...}
    						//		src/front_of_house.rs
    						//		src/front_of_house/mod.rs


// Código dentro do módulo é privado por default
// pub mod front_of_house;





fn outra_funcao() -> String {
	String::from("outra_funcao")
}



fn main() {
	println!("Função deste módulo: {}", outra_funcao() );

	println!("Função do módulo 'front_of_house': {}", front_of_house::init_front_of_house() );	// is private !!!

	println!("Função do submódulo 'hosting': {}", front_of_house::hosting::add_to_waitlist() );	// is private !!!

}


