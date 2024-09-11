/* 	Enum			[6.1. Defining an Enum]


Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


#[derive(Debug)]
enum Mensagem {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Mensagem {
	fn call(&self) {
		println!("Mensagem chamada é: {:?}", &self);
	}
}




fn main() {
	let m = Mensagem::Write(String::from("aloaloalo"));
    m.call();

}


