/* Geometria do cruzamento
	Zero de cada via é o início do cruzamento

						largura V
				margem V|    |
						|    |
						|    |margem H
------------------------+----+--------
	ViaH  > > > 	   	|    |	    largura H
------------------------+----+--------
	perímetro H			|    |
						|    |
						|    |
						| ^  |perímetro V
						| ^  |
						| ^  |
						|ViaV|
						|    |


*/

use crate::comunicacao::{Comunicacao,MensagemDeVeiculo};


pub mod veiculos;
pub use veiculos::Carro;



pub const VIAH_MARGEM: f64 = 15.0;		//metros
pub const VIAV_MARGEM: f64 = 15.0;		//metros

pub const VIAH_LARGURA: f64 = 4.0;		//metros
pub const VIAV_LARGURA: f64 = 4.0;		//metros

pub const VIAH_PERIMETRO: f64 = 150.0;	//metros
pub const VIAV_PERIMETRO: f64 = 150.0;	//metros

pub const VIAH_TOTAL: f64 = VIAH_PERIMETRO + VIAV_LARGURA + VIAH_MARGEM;	// metros
pub const VIAV_TOTAL: f64 = VIAV_PERIMETRO + VIAH_LARGURA + VIAV_MARGEM;	// metros




// Cruzamento entre duas vias
// 'enum' tem semântica 'move', mas 'Via' é barato para fazer copy
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Via {
	ViaH,
	ViaV,
}

// Transito composto por carros nas vias
pub struct Transito {
	carros_via_h: Vec<Carro>,		// Descrição dos carros na via H
	carros_via_v: Vec<Carro>,		// Descrição dos carros na via V
	carros_criados: i32,			// Número de carros criados no total
}


impl Transito {

	// Cria um novo transito
	pub fn new() -> Self {
		Self {
			carros_via_h: Vec::new(),
			carros_via_v: Vec::new(),
			carros_criados: 0,
		}
	}

	// Retorna iterador sobre carros de uma via
	pub fn get_iterador( &self, via:Via) -> std::slice::Iter<Carro> {
		match via {
			Via::ViaH => self.carros_via_h.iter(),
			Via::ViaV => self.carros_via_v.iter(),
		}
	}
	

	// Detecta se ocorreu uma colisão
	pub fn ocorreu_colisao(&self) -> Option<&str> {

		// Detecta colisão ao longo da via H
		if self.carros_via_h.len() >= 2 {
			for i in 0..self.carros_via_h.len()-1 {
				let traseira_do_i = 
					self.carros_via_h.get(i).unwrap().pos_atual - self.carros_via_h.get(i).unwrap().comprimento;
				if traseira_do_i <= self.carros_via_h.get(i+1).unwrap().pos_atual {
					return Some("Colisão via H");
				}
			}	
		}

		// Detecta colisão ao longo da via V
		if self.carros_via_v.len() >= 2 {
			for i in 0..self.carros_via_v.len()-1 {
				let traseira_do_i = 
					self.carros_via_v.get(i).unwrap().pos_atual - self.carros_via_v.get(i).unwrap().comprimento;
				if traseira_do_i <= self.carros_via_v.get(i+1).unwrap().pos_atual {
					return Some("Colisão via V");
				}
			}
		}

		// Detecta colisão no cruzamento
		let mut cruzando_h = false;
		let mut cruzando_v = false;

		for carro in &self.carros_via_h {
			cruzando_h = cruzando_h || (
				carro.pos_atual > 0.0  &&
				carro.pos_atual < 0.0 + VIAV_LARGURA + carro.comprimento );
		}	

		for carro in &self.carros_via_v {
			cruzando_v = cruzando_v || (
				carro.pos_atual > 0.0  &&
				carro.pos_atual < 0.0 + VIAH_LARGURA + carro.comprimento );
		}	

		if cruzando_h && cruzando_v {
			return Some("Colisão dentro do cruzamento");
		}

		// Não tem colisão
		None
	}



	// Define a velocidade com a qual o veiculo ingressa no perímetro	!!!
	fn define_velocidade_chegada(&self, via:&Via) -> f64 { 
		match via {
			Via::ViaH => {
				if self.carros_via_h.len() == 0 {
					return veiculos::VELOCIDADE_CRUZEIRO;					// Sozinho na via
				} else {
					let ultimo_carro = self.carros_via_h.last().unwrap();
					let distancia = VIAH_PERIMETRO + ultimo_carro.pos_atual - ultimo_carro.comprimento;
					if distancia < 20.0 {	// Considera via parada, não chega
						return 0.0;
					}
					// Qual velocidade de chegada para levar 2s até o da frente ?
					let velocidade = distancia / 2.0;
					if velocidade < veiculos::VELOCIDADE_CRUZEIRO {
						return velocidade;
					} else {
						return veiculos::VELOCIDADE_CRUZEIRO;
					}
				}
			}
			Via::ViaV  => {
				if self.carros_via_v.len() == 0 {
					return veiculos::VELOCIDADE_CRUZEIRO;
				} else {
					let ultimo_carro = self.carros_via_v.last().unwrap();
					let distancia = VIAV_PERIMETRO + ultimo_carro.pos_atual - ultimo_carro.comprimento;
					if distancia < 20.0 {	// Considera via parada, não chega
						return 0.0;
					}
					let raiz = ultimo_carro.vel_atual.powi(2) -
									2.0*ultimo_carro.acel_min *
									(VIAV_PERIMETRO + ultimo_carro.pos_atual - ultimo_carro.comprimento - 0.5);
					if raiz <= 0.0 {
						return 0.0;
					}	
					let vel_dist = raiz.sqrt();
					if vel_dist < veiculos::VELOCIDADE_CRUZEIRO {
						return vel_dist;
					} else {
						return veiculos::VELOCIDADE_CRUZEIRO;
					}
				}
			}
		}
	}	


	// Chega um novo carro no transito
	pub fn chega_carro( &mut self, via:Via, comunicacao:&mut Comunicacao) -> Result<(),String> {

		let vel = self.define_velocidade_chegada(&via);

		if vel == 0.0 {
			return Err("Via congestionada".to_string());
		}

		let mut nova_placa = String::from("CCC");
		nova_placa.push_str( &format!("{:04}",self.carros_criados) );
		self.carros_criados += 1;

		let novo_carro = Carro::new(nova_placa.clone(), via, 0.0);

		comunicacao.send_por_veiculo(MensagemDeVeiculo::Chegada {
			placa: nova_placa,
			via: via,
			acel_max: novo_carro.acel_max,
			acel_min: novo_carro.acel_min,
			vel_max: novo_carro.vel_max,
			comprimento: novo_carro.comprimento
			 } );


		match via {
			Via::ViaH => {
				self.carros_via_h.push(novo_carro);
			}
			Via::ViaV => {
				self.carros_via_v.push(novo_carro);
			}
		}

		Ok(())
	}



	// Avança o estado de todos os carros por tickms milissegundos
	pub fn tick( &mut self, tickms:f64, comunicacao:&mut Comunicacao) {
		println!("transito.tick");

		// Atualiza todos os carros da via H
		for carro in &mut self.carros_via_h {
			carro.tick(tickms, comunicacao);
		}

		// Atualiza todos os carros da via V
		for carro in &mut self.carros_via_v {
			carro.tick(tickms, comunicacao);
		}

		// Carro mais antigo na via H saiu do sistema ?
		// Obs: Seria melhor usar VeqDeque no lugar de Vec neste caso
		// https://doc.rust-lang.org/std/collections/struct.VecDeque.html#
		if self.carros_via_h.len() > 0  {
			let mais_antigo_h = self.carros_via_h.get(0).unwrap();
			if mais_antigo_h.pos_atual > mais_antigo_h.comprimento + VIAV_LARGURA + VIAH_MARGEM {
				println!("@{} saiu da via H", mais_antigo_h.placa);
				self.carros_via_h.remove(0);
			}
		}

		// Carro mais antigo na via V saiu do sistema ?
		// Obs: Seria melhor usar VeqDeque no lugar de Vec neste caso
		// https://doc.rust-lang.org/std/collections/struct.VecDeque.html#
		if self.carros_via_v.len() > 0  {
			let mais_antigo_v = self.carros_via_v.get(0).unwrap();
			if mais_antigo_v.pos_atual > mais_antigo_v.comprimento + VIAH_LARGURA + VIAV_MARGEM {
				println!("@{} saiu da via V", mais_antigo_v.placa);
				self.carros_via_v.remove(0);
			}
		}

	}


	// Mostra estado das vias
	pub fn mostra_vias(&self) {
		println!("___Carros na via H___");

		for carro in &self.carros_via_h {
			carro.mostra();
		}

		println!("___Carros na via V___");
		for carro in &self.carros_via_v {
			carro.mostra();
		}

	}


	// Verifica se algum carro no sistema
	pub fn vazio(&self) -> bool {
		self.carros_via_h.len()  == 0  &&  self.carros_via_v.len() == 0
	}





} 






