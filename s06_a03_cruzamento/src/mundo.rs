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
	num_carros_criados_h: usize,	// Número de carros criados para a via H
	num_carros_sairam_h: usize,		// Número de carros ativos na via H
	carros_via_h: [Carro;4],		// Descrição dos carros criados até agora na via H
	num_carros_criados_v: usize,	// Número de carros criados para a via V
	num_carros_sairam_v: usize,		// Número de carros ativos na via V
	carros_via_v: [Carro;4],		// Descrição dos carros criados até agora na via V
}

impl Transito {

	// Cria um novo transito
	pub fn new() -> Self {
		Self {
			num_carros_criados_h: 0,
			num_carros_sairam_h: 0,
			carros_via_h: [ Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
							Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
							Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
							Carro::new(String::from("AAA0000"), Via::ViaH, 0.0)],
			num_carros_criados_v: 0,
			num_carros_sairam_v: 0,
			carros_via_v: [ Carro::new(String::from("AAA0000"), Via::ViaV, 0.0),
							Carro::new(String::from("AAA0000"), Via::ViaV, 0.0),
							Carro::new(String::from("AAA0000"), Via::ViaV, 0.0),
							Carro::new(String::from("AAA0000"), Via::ViaV, 0.0)],
		}	
	}

	// Detecta se ocorreu uma colisão
	pub fn ocorreu_colisao(&self) -> Option<&str> {
		let mut i: usize = self.num_carros_sairam_h + 1;
		while i < self.num_carros_criados_h {
			if self.carros_via_h[i-1].pos_atual - self.carros_via_h[i-1].comprimento <= self.carros_via_h[i].pos_atual {
				return Some("Colisão via H");
			}	
			i += 1;
		}

		i = self.num_carros_sairam_v + 1;
		while i < self.num_carros_criados_v {
			if self.carros_via_v[i-1].pos_atual - self.carros_via_v[i-1].comprimento <= self.carros_via_v[i].pos_atual {
				return Some("Colisão via V");
			}
			i += 1;
		}

		// Detecta colisão no cruzamento
		let mut cruzando_h = false;
		let mut cruzando_v = false;
		i = self.num_carros_sairam_h;
		while i < self.num_carros_criados_h {
			cruzando_h = cruzando_h || (
				self.carros_via_h[i].pos_atual > 0.0  &&
				self.carros_via_h[i].pos_atual < 0.0 + VIAV_LARGURA + self.carros_via_h[i].comprimento );
			i += 1;
		}	
		i = self.num_carros_sairam_v;
		while i < self.num_carros_criados_v {
			cruzando_v = cruzando_v || (
				self.carros_via_v[i].pos_atual > 0.0  &&
				self.carros_via_v[i].pos_atual < 0.0 + VIAH_LARGURA + self.carros_via_v[i].comprimento );
			i += 1;
		}
		if cruzando_h && cruzando_v {
			return Some("Colisão dentro do cruzamento");
			}

		// Não tem colisão
		None
	}


	// Chega um novo carro no transito
	pub fn chega_carro( &mut self, via:Via) -> bool {

		let jah_tem = match via {
			Via::ViaH => self.num_carros_criados_h,
			Via::ViaV => self.num_carros_criados_v,
		};

		if jah_tem == VIA_MAXIMO_CARROS {
			return false;
		}

		let mut nova_placa = String::from("CCC");
		nova_placa.push_str( &format!("{:04}",jah_tem) );
		let novo_carro = Carro::new(nova_placa, via.clone(), veiculos::ACELERACAO_MAXIMA);

		match via {					// Posso usar aqui pois foi usado um clone antes
			Via::ViaH => {
				self.carros_via_h[self.num_carros_criados_h] = novo_carro;
				self.num_carros_criados_h += 1;
			}
			Via::ViaV => {
				self.carros_via_v[self.num_carros_criados_v] = novo_carro;
				self.num_carros_criados_v += 1;
			}
		}

		true
	}


	// Avança o estado de todos os carros por tickms milissegundos
	pub fn tick( &mut self, tickms:f64) {
		println!("transito.tick");

		let mut i;

		i = self.num_carros_sairam_h;
		while i < self.num_carros_criados_h {
			self.carros_via_h[i].tick(tickms);
			i += 1;
		}

		i = self.num_carros_sairam_v;
		while i < self.num_carros_criados_v {
			self.carros_via_v[i].tick(tickms);
			i += 1;	
		}

		// Carro mais antigo na via H saiu do sistema ?
		if self.num_carros_sairam_h < self.num_carros_criados_h {
			let mais_antigo_h = &self.carros_via_h[ self.num_carros_sairam_h ];
			if mais_antigo_h.pos_atual > mais_antigo_h.comprimento + VIAV_LARGURA {
				println!("@{} saiu da via H", mais_antigo_h.placa);
				self.num_carros_sairam_h += 1;
			}
		}

		// Carro mais antigo na via V saiu do sistema ?
		if self.num_carros_sairam_v < self.num_carros_criados_v {
			let mais_antigo_v = &self.carros_via_v[ self.num_carros_sairam_v ];
			if mais_antigo_v.pos_atual > mais_antigo_v.comprimento + VIAH_LARGURA {
				println!("@{} saiu da via H", mais_antigo_v.placa);
				self.num_carros_sairam_v += 1;
			}
		}

	}


	// Mostra estado das vias
	pub fn mostra_vias(&self) {
		println!("___Carros na via H___");
		let mut i = self.num_carros_sairam_h;
		while i < self.num_carros_criados_h {
			self.carros_via_h[i].mostra();
			i += 1;
		}

		println!("___Carros na via V___");
		i = self.num_carros_sairam_v;
		while i < self.num_carros_criados_v {
			self.carros_via_v[i].mostra();
			i += 1;
		}
	}


	// Verifica se algum carro no sistema
	pub fn vazio(&self) -> bool {
		self.num_carros_criados_h == self.num_carros_sairam_h  &&
		self.num_carros_criados_v == self.num_carros_sairam_v
	}





} 






