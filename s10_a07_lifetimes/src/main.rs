/*
	Lifetimes				[10.3. Validating References with Lifetimes]

Baseado no livro:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



// Toda referência em Rust tem um tempo de vida, o tempo no qual ela é válida
// Na maioria dos casos o compilador controla sozinho o tempo de vida dos valores
// As vezes são necessárias anotações do programador para ajudar o compilador


fn main() {

	// Conceito de tempo de vida
	let y = 9;
	let mut r;
	{
		let x = 5;		// Tempo de vida de 'x' limitado a este bloco
		r = &x;
	}
//	println!("r: {}", r);		// Aqui o 'x' não existe mais

	r = &y;
	println!("r: {}", r);		// Aqui o 'y' ainda existe


	// Anotações de tempo de vida em funções
	let string1 = String::from("abcdef");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);	// Ambos são válidos até fim do programa
    println!("The longest string is {}", result);


	// Borrow checker vê 'resultado' com tempo de vida apenas no bloco, mas está ok
	let string1 = String::from("longolongolongo");
	{
		let string2 = String::from("xyz");
		let resultado = longest(string1.as_str(), string2.as_str()); // Pior caso é string2, mas está ok
		println!("resultado dentro do bloco é: {}", resultado);			// 'resultado' é usado apenas dentro do bloco
	}


	// Borrow checker usa o menor tempo de vida, 'resultado' pode ter string2, então erro
	let string1 = String::from("longolongolongo");
	let mut resultado = "?";
	{
		let string2 = String::from("xyz");
		resultado = longest(string1.as_str(), string2.as_str());	// Pior caso é string2
	}
	//println!("resultado fora do bloco é {}", resultado);			// Aqui string2 não é mais válido
																	// então resultado pode ter sido invalidado
    


	// Funciona pois o retorno tem o tempo de vida de 'string1' e é válido até o fim da 'main'
    let result = longest2(string1.as_str(), string2);
    println!("The longest22 string is {}", result);

}



// Em funções a sintaxe de lifetime busca conectar os tempos de vida dos vários parâmetros
// com o tempo de vida do retorno da função



// 's1' e 's2' podem ter tempos de vida diferentes
// compilador não sabe qual deles será retornado
// compilador não sabe qual o tempo de vida de 'result'
// fn longest(s1: &str, s2: &str) -> &str {

// Dizer para o compilador: o tempo de vida do retorno será o menor tempo de vida entre argumentos
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
	if s1.len() > s2.len() {
		s1
	} else {
		s2
	}
}


// Diz para o compilador: o tempo de vida do retorno será o tempo de vida de 's1'
//fn longest2(s1: &str, s2: &str) -> &str {
fn longest2<'a>(s1: &'a str, s2: &str) -> &'a str {
	s1
}


/*
// Não funciona pois o String em result será destruído ao final da função
// Logo a referência retornada por ela seria inválida
//fn longest3(x: &str, y: &str) -> &str {
fn longest3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/


// Não requer anotação de tempo de vida
// Funciona pois o String criado é 'MOVE' como retorno da função
fn longest4(x: &str, y: &str) -> String {
	let result = String::from("novo string criado");
	result
}



