/*
crate root  (main.rs)
 ├── main()
 ├── outra_funcao()
 │
 └── front_of_house		<<<<<<<<<<<<<<<<<<<<<<<<<-----
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



//mod hosting;			// Declaração do submódulo 'hosting'
						// crate::front_of_house::hosting::coisas_do_hosting
						// Código pode estar:
						//		Inline: mod hosting {...}
						//		src/front_of_house/hosting.rs
						//		src/front_of_house/hosting/mod.rs



mod serving;			// Declaração do submódulo 'serving'
						// crate::front_of_house::serving::coisas_do_serving
						// Código pode estar:
						//		Inline: mod serving {...}
						//		src/front_of_house/serving.rs
						//		src/front_of_house/serving/mod.rs



// Código dentro do módulo é privado por default
pub mod hosting;



pub fn init_front_of_house() -> String {
//fn init_front_of_house() -> String {
	String::from("init_front_of_house")
}



// Chamadas através de caminhos
fn chamadas_caminhos() {

	// 'outra_funcao' não precisa ser publica pois está no módulo pai
	crate::outra_funcao();
	super::outra_funcao();


	// Caminho absoluto
	println!("Caminho absoluto: {}", crate::front_of_house::hosting::add_to_waitlist() );

    // Caminho relativo
	println!("Caminho relativo: {}", hosting::add_to_waitlist() );

}


