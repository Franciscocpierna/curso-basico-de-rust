/*
crate root  (main.rs)
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
     └── serving	<<<<<<<<<<<<<<<<<<<<<<<<<-----
         ├── take_order()
         ├── serve_order()
         └── take_payment()
*/




fn take_order() -> String {
	String::from("take_order")
}


fn serve_order() -> String {
	String::from("serve_order")
}


fn take_payment() -> String {

	println!("Função do módulo 'front_of_house': {}", super::init_front_of_house() );	// super

	println!("Função do submódulo 'hosting': {}", super::hosting::add_to_waitlist() );

	String::from("take_payment")
}




