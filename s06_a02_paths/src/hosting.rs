/*
crate root  (main.rs)
 ├── main()
 ├── outra_funcao()
 │
 ├── cooking
 │   ├── prepare_food()
 │   └── clean_up()
 │
 ├── hosting	<<<<<<<<<<<<<<<<<<<<<<<<<-----
 │   ├── add_to_waitlist()
 │   └── seat_at_table()
 │
 └── serving
     ├── take_order()
     ├── serve_order()
     ├── take_payment()
	 └── clean_up()
*/



// Cada campo da struct é privado por default, pode ser tornado público
pub struct Breakfast {
	pub toast: String,
	seasonal_fruit: String,
}

// Funções precisam ser exportadas individualmente
impl Breakfast{
	pub fn new() -> Self {
		Self {
			toast: String::from("torrada"),
			seasonal_fruit: String::from("maça"),
		}
	}
}


// Todas as variantes do enum ficam públicas
pub enum Appetizer {
	Soup,
	Salad,
}






pub fn add_to_waitlist() -> String {
	String::from("add_to_waitlist")
}


fn seat_at_table() -> String {
	String::from("seat_at_table")
}
