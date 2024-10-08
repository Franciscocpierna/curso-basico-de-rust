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

mod veiculos;

use veiculos::Carro;


const _VIAH_MARGEM: f64 = 15.0;		//metros
const _VIAV_MARGEM: f64 = 15.0;		//metros

const VIAH_LARGURA: f64 = 4.0;		//metros
const VIAV_LARGURA: f64 = 4.0;		//metros

const VIAH_PERIMETRO: f64 = 150.0;	//metros
const VIAV_PERIMETRO: f64 = 150.0;	//metros

const VIA_MAXIMO_CARROS: usize = 4;	// Número máximo de carros criados por via


// Cruzamento entre duas vias
// 'enum' tem semântica 'move', mas 'Via' é barato e facilita poder clonar o valor às vezes
#[derive(Debug,Clone)]
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



	// Define a velocidade com a qual o veiculo ingressa no perímetro
	fn define_velocidade_chegada(&self, via:&Via) -> f64 { 
		match via {
			Via::ViaH => {
				if self.carros_via_h.len() == 0 {
					return veiculos::VELOCIDADE_CRUZEIRO;
				} else {
					let ultimo_carro = self.carros_via_h.last().unwrap();
					let distancia = VIAH_PERIMETRO + ultimo_carro.pos_atual - ultimo_carro.comprimento;

					if distancia < 0.5 {	// Considera via parada, não chega
						return 0.0;
					} else if distancia < 4.0 {		// Considera via congestionada, chega como pelotão
						return veiculos::VELOCIDADE_CRUZEIRO.min(ultimo_carro.vel_atual);
					} else {
						veiculos::VELOCIDADE_CRUZEIRO	// Considera via livre
					}
				}	
			}
			Via::ViaV => {
				if self.carros_via_v.len() == 0 {
					return veiculos::VELOCIDADE_CRUZEIRO;
				} else {
					let ultimo_carro = self.carros_via_v.last().unwrap();
					let distancia = VIAV_PERIMETRO + ultimo_carro.pos_atual - ultimo_carro.comprimento;

					if distancia < 0.5 {	// Considera via parada, não chega
						return 0.0;
					} else if distancia < 4.0 {		// Considera via congestionada, chega como pelotão
						return veiculos::VELOCIDADE_CRUZEIRO.min(ultimo_carro.vel_atual);
					} else {
						veiculos::VELOCIDADE_CRUZEIRO	// Considera via livre
					}
				}	
			}
		}
	}	



	// Chega um novo carro no transito
	pub fn chega_carro( &mut self, via:Via) -> bool {

		let vel = self.define_velocidade_chegada(&via);
		let acel: f64;

		if vel == 0.0 {
			return false;
		}

		let mut nova_placa = String::from("CCC");
		nova_placa.push_str( &format!("{:04}",self.carros_criados) );
		self.carros_criados += 1;

		let novo_carro = Carro::new(nova_placa, via.clone(), 0.0);

		match via {					// Posso usar aqui pois foi usado um clone antes
			Via::ViaH => {
				self.carros_via_h.push(novo_carro);
			}
			Via::ViaV => {
				self.carros_via_v.push(novo_carro);
			}
		}

		true
	}



	// Avança o estado de todos os carros por tickms milissegundos
	pub fn tick( &mut self, tickms:f64) {
		println!("transito.tick");

		// Atualiza todos os carros da via H
		for carro in &mut self.carros_via_h {
			carro.tick(tickms);
		}

		// Atualiza todos os carros da via V
		for carro in &mut self.carros_via_v {
			carro.tick(tickms);
		}

		// Carro mais antigo na via H saiu do sistema ?
		// Obs: Seria melhor usar VeqDeque no lugar de Vec neste caso
		// https://doc.rust-lang.org/std/collections/struct.VecDeque.html#
		if self.carros_via_h.len() > 0  {
			let mais_antigo_h = self.carros_via_h.get(0).unwrap();
			if mais_antigo_h.pos_atual > 0.0 + mais_antigo_h.comprimento + VIAV_LARGURA {
				println!("@{} saiu da via H", mais_antigo_h.placa);
				self.carros_via_h.remove(0);
			}
		}

		// Carro mais antigo na via V saiu do sistema ?
		// Obs: Seria melhor usar VeqDeque no lugar de Vec neste caso
		// https://doc.rust-lang.org/std/collections/struct.VecDeque.html#
		if self.carros_via_v.len() > 0  {
			let mais_antigo_v = self.carros_via_v.get(0).unwrap();
			if mais_antigo_v.pos_atual > 0.0 + mais_antigo_v.comprimento + VIAH_LARGURA {
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






