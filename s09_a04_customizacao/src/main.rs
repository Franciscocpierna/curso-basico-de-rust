/*
	Pânico ou não ?				[9.3. To panic! or Not to panic!]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


/*	Em resumo:
	- Pânico é mais aceitável em um protótipo ou um código para teste: panic, unwrap, expect, ...
		(desenvolvedor vai avaliar o que aconteceu)
	- Em código de produção o pânico deve ser evitado, especialmente em embedded systems
		(desligamento controlado do equipamento é melhor que uma falha descontrolada)
	- 'Result' deve ser usado quando erros de hardware são infrequentes porém não impossíveis
	- 'Result' e 'Option' obrigam o programador a pensar nos casos onde a situação não é ideal
*/



// Verificando a entrada em cada função

fn funcao1(x:isize) -> Option<isize> {
	if x < -100  ||  x > 100 {
		return None;
	} else {
		return Option::Some(x+100);		// Pode ser um cálculo complexo
	}
}


fn funcao2(x:isize) -> Option<isize> {
	if x < -100  ||  x > 100 {
		return None;
	} else {
		return Option::Some(x+200);		// Pode ser um cálculo complexo
	}
}


fn funcao3(x:isize) -> Option<isize> {
	if x < -100  ||  x > 100 {
		return None;
	} else {
		return Option::Some(x+300);		// Pode ser um cálculo complexo
	}
}





// Criando um tipo próprio para validar os dados

// Contém um inteiro entre -100 e +100, zero na criação
pub struct MeuInteiro {
	value: isize,
}

impl MeuInteiro {

	pub fn new() -> MeuInteiro {
        MeuInteiro{ value:0 }
    }

	pub fn set_value(&mut self, novo:isize) -> bool {
		if novo < -100  ||  novo > 100 {
			//  panic!("Valor deve estar entre -100 e +100, recebeu {}.", novo);
			return false;
		}
		self.value = novo;
		true
    }

	pub fn get_value(&self) -> isize {
		self.value
	}
}




fn funcao1_nova(x:&MeuInteiro) -> isize {
	x.get_value()+100			// Pode ser um cálculo complexo
}

fn funcao2_nova(x:&MeuInteiro) -> isize {
	x.get_value()+200			// Pode ser um cálculo complexo
}

fn funcao3_nova(x:&MeuInteiro) -> isize {
	x.get_value()+300			// Pode ser um cálculo complexo
}




fn main() {

	println!("Versão antiga: {:?}", funcao1(55));
	println!("Versão antiga: {:?}", funcao2(55));
	println!("Versão antiga: {:?}", funcao3(155));



	let mut entrada = MeuInteiro::new();
	if entrada.set_value(55) {
		println!("Versão nova: {:?}", funcao1_nova(&entrada));
		println!("Versão nova: {:?}", funcao2_nova(&entrada));
		println!("Versão nova: {:?}", funcao3_nova(&entrada));
	} else {
		println!("Número errado, faz alguma coisa");
	}

}




