/*
	Definindo e instanciando Structs: Parte 1		[5.1. Defining and Instantiating Structs]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {

	//Note that the entire instance must be mutable
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("primeiro@example.com"),
        sign_in_count: 1,
    };
	println!("Email user1: {}", user1.email);

    user1.email = String::from("segundo@example.com");
	user1.sign_in_count += 1;
	println!("Email user1: {}", user1.email);


	let user2 = build_user(String::from("terceiro@gmail.com"), String::from("terceiro") );
	println!("Email user2: {}", user2.email);

}


//  Retorna uma instância de User
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}




