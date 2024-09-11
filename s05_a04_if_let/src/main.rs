/*
	if let			[6.3. Concise Control Flow with if let]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(usize),
}


#[derive(Debug)]
enum Mensagem {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}




fn main() {

	let config_max = Some(3);
	
	// 'match' normal
	match config_max {
		Some(max) => println!("match, The maximum is configured to be {}", max),
		_ => (),
	}

	// Usando 'if let'
	if let Some(max) = config_max {
		println!("if_let, The maximum is configured to be {}", max);
	}



	let coin = Coin::Quarter(1999);
	let mut count = 0;

	// 'match' normal
    match coin {
        Coin::Quarter(ano) => println!("match, Ano do quarter {:?}!", ano),
        _ => count += 1,
    }

	// Usando 'if let' com um 'else'
	if let Coin::Quarter(ano) = coin {
		println!("if_let, Ano do quarter {:?}!", ano);
	} else {
		count += 1;
	}




	// Quando um só valor interessa
	let m1 = Mensagem::Write(String::from("aloalo"));
    let m2 = Mensagem::ChangeColor(11,22,33);
	if let Mensagem::Write(txt) = m1 {
		println!("O texto no WRITE m1 é {}", txt);
    }
	if let Mensagem::Write(txt) = m2 {
		println!("O texto no WRITE m2 é {}", txt);
    }


	// Semântica Copy e Move
	let m3 = Mensagem::Write(String::from("aloalo"));
	
	match m3 {
		Mensagem::Write(ref txt) => println!("Em m3 temos: {}", txt),
		//Mensagem::Write(txt) => println!("Em m3 temos: {}", txt),
		_ => (),
	}
	// Sem o 'ref', o String é movido para 'txt', invalidando 'm3', que não poderia ser usado abaixo
	println!("m3 é: {:?}", m3);



}


