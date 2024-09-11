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
    username: String,		// Pode usar &str ?
    email: String,
    sign_in_count: u64,
}


fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("primeiro@example.com"),
        sign_in_count: 1,
    };
	println!("user1: {}", user1.email);

	user1.email = String::from("segundo@example.com");
	println!("user1: {}", user1.email);

	let _user2 = build_user(String::from("terceiro@example.com"), String::from("terceiro") );

	let user2 = build_user_com_slice("terceiro@example.com", "terceiro" );

	println!("user2: {}", user2.email);


	// Forma não abreviada de inicialização
    let user3 = User {
		active: user1.active,
		username: user1.username,	// o campo username:String de user1 é movido para user3, user1 fica inválido
		email: String::from("quarto@example.com"),
	    sign_in_count: user1.sign_in_count,
    };
	println!("user3: {}", user3.email);


	// Forma abreviada de inicialização
    let user4 = User {
		email: String::from("quinto@example.com"),
		..user1			// o campo username:String de user1 é movido para user3, user1 fica inválido
    };
	println!("user4: {}", user4.email);


}


// Forma abreviada de inicialização
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Forma abreviada de inicialização
fn build_user_com_slice(email: &str, username: &str) -> User {
    User {
        active: true,
        username: username.to_string(),		// Precisa to_string() ?
        email: email.to_string(),
        sign_in_count: 1,
    }
}

