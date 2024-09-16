/*
crate root  (main.rs)
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
 └── serving	<<<<<<<<<<<<<<<<<<<<<<<<<-----
     ├── take_order()
     ├── serve_order()
     ├── take_payment()
	 └── clean_up()
*/




fn take_order() -> String {
	String::from("take_order")
}


fn serve_order() -> String {
	String::from("serve_order")
}


fn take_payment() -> String {
	String::from("take_payment")
}


pub fn clean_up() -> String {
	String::from("clean_up")
}	


