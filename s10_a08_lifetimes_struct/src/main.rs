/*
	Lifetimes em Struct		[10.3. Validating References with Lifetimes]

Baseado no livro:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



// str como campo de struct
// str em 'nome' deve ter um tempo de vida pelo menos igual ao tempo de vida da struct 'Pessoa'
// Mesma coisa para 'cpf'
#[derive(Debug)]
struct Pessoa<'a> {
	nome: &'a str,
	cpf: &'a str,
}

impl<'a> Pessoa<'a> {
	fn get_cpf(&self) -> &str {
		self.cpf
	}

	fn longest(&self, x:&'a str) -> &str {
		if x.len() > self.nome.len() {
			x
	    } else {
			self.nome
	    }
	}
	
}





// A restrição de tempo de vida colocada na struct 'Pessoa' é respeitada com 'p1' mas não com 'p2'
fn main() {
	let meu_nome = String::from("Rômulo");
	let p1 = Pessoa {
		nome: &meu_nome,
		cpf: "12345678",
    };
	println!("p1: {:?}", p1);
		
	
	let p2: Pessoa;								// Tem o tempo de vida da função main()
	{
		let cpf_estatico = "999888777666";		// str literal é sempre 'static
		let cpf_bloco = String::from("999888777666");

		p2 = Pessoa {
			nome: meu_nome.as_str(),	// Tem o tempo de vida da função main()
			//cpf: &cpf_bloco,			// Tem o tempo de vida deste bloco !!!
			cpf: cpf_estatico,			// Tem o tempo de vida estático
		};
	}
	println!("p2: {:?}", p2);
	println!("cpf de p1: {}", p1.get_cpf() );		// p1 ainda é usado aqui
	println!("cpf de p2: {}", p2.get_cpf() );		// p2 ainda é usado aqui


	println!("retorno do add(): {}", add());

	println!("retorno do longest: {}", p1.longest("abc") );

}



// 'static significa que a referência é válida durante toda a execução do programa
// Por exemplo, todo string literal é 'static
fn add() -> &'static str {
	"added"
}	




// Um exemplo com Generics para tipo e tempo de vida
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
	x: &'a str,
	y: &'a str,
	ann: T,
) -> &'a str
where
	T: Display,			// T precisa implementar o trait Dispĺay
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}





/*
	Anotações de tempo de vida podem ser feitas automaticamente pelo compilador:
	(lifetime elision rules)

Regra 1)
O compilador atribui um parâmetro de tempo de vida 'a para cada parâmetro da função que for referência.

Regra 2)
Se existe apenas um parâmetro de tempo de vida na entrada, o mesmo é colocado na saída.
	
Regra 3)
Se existem vários parâmetros de tempo de vida na entrada, e um deles é &self ou &mut self,
é este o tempo de vida associado com a saída.

*/


