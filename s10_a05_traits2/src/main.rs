/*
	Traits: Parte 2		[10.2. Traits: Defining Shared Behavior]

Baseado no livro:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


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



// Função aceita como parâmetro qualquer tipo que implementa o trait Summary
pub fn notify1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Sintaxe alternativa
pub fn notify2<T: Summary>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}
	
	

// 'item1' e 'item2' são implementações de 'Summary'
// 'item1' e 'item2' podem ser de tipos diferentes, desde que implementem o trait 'Summary'
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
	// ...
}


// 'item1' e 'item2' são referências para o mesmo tipo 'T', e o tipo 'T' implementa Summary
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
	// ...
}	





// Função aceita como parâmetro qualquer tipo que implementa os traits 'Summary' e 'Display'
fn notify_duplo1(item: &(impl Summary + Debug)) {
    println!("notify_duplo1:   {}  {:?}", item.summarize(), item);
}

// Sintaxe alternativa
fn notify_duplo2<T: Summary + Debug>(item: &T) {
    println!("notify_duplo2:   {}  {:?}", item.summarize(), item);
}





// Sintaxe alternativa para muitos traits 
fn super_notify<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
	0
}	

fn mesmo_super_notify<T, U>(t: &T, u: &U) -> i32
where
	T: Display + Clone,
	U: Clone + Debug,
{
	0
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

	notify1(&tweet);
	notify2(&tweet);


	let s = String::from("qwerty");
	// String não implementa Summary
	//notify1(&s);

	// '#[derive(Debug)]' para satisfazer o requisito
	notify_duplo1(&tweet);


}



