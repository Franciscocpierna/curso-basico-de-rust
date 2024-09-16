/*
	Traits: Parte 3		[10.2. Traits: Defining Shared Behavior]

Baseado no livro:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


// Necessário para usar como restrição de genérico
use std::fmt::{Debug,Display};




// Um trait define funcionalidade que um tipo em particular oferece
pub trait Summary {
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


#[derive(Debug)]
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




// Pode retornar um (único) tipo desconhecido mas que implementa um certo trait 
fn returns_summarizable() -> impl Summary {
	Tweet {
		username: String::from("horse_ebooks"),
		content: String::from(
			"of course, as you probably already know, people",
		),
		reply: false,
		retweet: false,
	}
}





//#[derive(PartialOrd)]
//#[derive(PartialEq,PartialOrd)]
//#[derive(Display,PartialEq,PartialOrd)]
#[derive(Debug,PartialEq,PartialOrd)]
enum MeuEnum {
	VAR1,
	VAR2,
}


// Implementação de um método condicionada à implementação de traits
struct Pair<T> {
    x: T,
    y: T,
}

// Qualquer 'T' serve
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Precisa Display para o '{}'
// Precisa PartialOrd para o '>='
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


// Precisa Debug para o '{:?}'
// Precisa PartialOrd para o '>='
impl<T: Debug + PartialOrd> Pair<T> {
    fn cmp_debug(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}







fn main() {

	// Pode retornar um (único) tipo desconhecido mas que implementa um certo trait 
	let x = returns_summarizable();
	println!("x é:  {:?}", x.summarize() );


	// Implementação de um método condicionada à implementação de traits
	let p1 = Pair::new(11,22);
	p1.cmp_display();

	let p2 = Pair::new(33.33,44.44);
	p2.cmp_display();


	let p3 = Pair::new(MeuEnum::VAR1,MeuEnum::VAR2);
	//p3.cmp_display();
	p3.cmp_debug();	

}




