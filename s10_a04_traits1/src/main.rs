/*
	Traits: Parte 1		[10.2. Traits: Defining Shared Behavior]

Baseado no livro:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


// Um trait define funcionalidade que um ou mais tipos de dados oferece
trait Summary {
	fn summarize(&self) -> String;
}


pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}


impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}




pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}





// É possível implementar novos traits em tipos velhos
// Mas apenas quando ou o trait ou o type é local ao crate da implementação (Summary é local)
impl Summary for String {
	fn summarize(&self) -> String {
		format!("Tamanho do String: {}", self.len())
	}
}



// Um trait pode ter comportamento default
pub trait Summary2 {
    fn summarize(&self) -> String {
        String::from("(comportamento default...)")
    }
}


// Pode-se usar o comportamento default do trait
struct NaoFazNada {	
}

impl Summary2 for NaoFazNada {
}



// É possível gerar conflitos de nomes de métodos
// Exemplo: o tipo 'NaoFazNada' implementa os traits 'Summary' e 'Summary2'
// Cada um desses traits tem um método 'summarize' diferente
// Qual é que vale ???
impl Summary for NaoFazNada {
	fn summarize(&self) -> String {
		format!("Conflito !!!")
	}
}






fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("Sumário do novo tweet:\n\t\t\t {}", tweet.summarize());


	let s = String::from("qwerty");
	println!("Sumário de v:\n\t\t {}", s.summarize());


	let nfn = NaoFazNada{};
	// Sintaxe alternativa permite eliminar a ambiguidade
	//println!("Sumário de nfn:\n\t\t {}", nfn.summarize());
	
	println!("Sumário de nfn:\n\t\t {}", Summary2::summarize(&nfn));

}





