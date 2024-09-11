/* Funções			[3.3. Functions]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



fn outra_funcao() {
	println!("Outra função.");
}

fn outra_funcao_com_parametro(x: i32) {
	println!("Outra função recebeu {x}.");
}

fn print_labeled_measurement(valor: f64, unidade: char) {
	println!("A medida é: {valor}{unidade}.");
}



fn soma(x:i32, y:i32) -> i32 {
	x+y		// Sem o ponto e vírgula
			// ; o que acontece ???
}


fn somaret(x:i32, y:i32) -> i32 {
	return x+y;
}
			

fn main() {
    println!("Hello, world!");
	outra_funcao();
	outra_funcao_com_parametro(55);
	print_labeled_measurement(123.4,'m');
	
	let x = 999;
	//print_labeled_measurement(123.0,'m');				// Use literal f64
	print_labeled_measurement(x as f64,'m');			// Converter manualmente



	let xy = soma(3,4);
	println!("Somas deram {} e {}", xy, somaret(3,4) );



	// Vale para qualquer bloco
	let mut y = {
        let x = 3;
        x + 1
    };
	println!("Sem ponto e vírgula {:?}", y);

	y = {			// Poderia fazer y mutável ???
		let x = 3;
        x + 19999
    };
	println!("Com ponto e vírgula {:?}", y);



}

