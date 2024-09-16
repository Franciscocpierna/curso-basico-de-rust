/*
crate root  (main.rs)
 ├── main()
 ├── outra_funcao()
 │
 └── front_of_house
     ├── init_front_of_house()
     ├── chamadas_caminhos()
     │
	 ├── hosting	<<<<<<<<<<<<<<<<<<<<<<<<<-----
     │   ├── add_to_waitlist()
     │   └── seat_at_table()
     │
     └── serving
         ├── take_order()
         ├── serve_order()
         └── take_payment()
*/




// Cada campo da struct é privado por default, pode ser tornado público individualmente
pub struct Breakfast {
	pub toast: String,
	seasonal_fruit: String,
}


// Todas as variantes do enum ficam públicas automaticamente
pub enum Appetizer {
	Soup,
	Salad,
}





pub fn add_to_waitlist() -> String {
//fn add_to_waitlist() -> String {
	String::from("add_to_waitlist")
}


fn seat_at_table() -> String {
	String::from("seat_at_table")
}

