/* Controle de Fluxo	[3.5. Control Flow]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




fn main() {

	let mut i = 0;

	loop {
		i += 1;
		if i % 2 == 0 {
			continue;
		}
		println!("i {i}");
		if i >= 10 {
			break;
		}
	}


	// loop como expressão
	let result = loop {
        i += 100;
        if i >= 100 {
            break i * 2;
        }
    };
	println!("\nresult {result}");


	// Labels em loops
	println!("\nLabels em loops");
	let mut contagem = 0;

	'meu_externo: loop {
		println!("contagem = {contagem}");
		let mut faltam = 100;
	
		loop {
			println!("faltam = {faltam}");
			if faltam == 97 {
				break;					// break do loop interno
			}

			println!("contagem = {contagem}");
			if contagem == 2 {
				break 'meu_externo;		// break do loop denominado
			}

			faltam -= 1;
		}

		println!("incrementa contagem");
		contagem += 1;
		}
		println!("Contagem final = {contagem}");



}
