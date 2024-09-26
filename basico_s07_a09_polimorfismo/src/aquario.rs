/*

Nesta aula:

	Como lidar com variáveis que podem ter tipos diferentes (polimorfismo) ?

	O enum 'embrulha' os diversos tipos específicos, provendo um tipo geral.

*/



/********************
 *	Base Quadrada	*
********************/

struct BaseQuadrada {
	lado: f64,
}

impl BaseQuadrada {
	fn new(lado:f64) -> BaseQuadrada {
		BaseQuadrada { lado }
	}
	fn area(&self) -> f64 {
		self.lado * self.lado
	}
}


/************************
 *	Base Retangular		*
************************/

struct BaseRetangular {
	lado_1: f64,
	lado_2: f64,
}

impl BaseRetangular {
	fn new(lado_1:f64,lado_2:f64) -> BaseRetangular {
		BaseRetangular { lado_1, lado_2 }
	}

	fn area(&self) -> f64 {
		self.lado_1 * self.lado_2
	}
}


/********************
 *	Base Circular	*
********************/

struct BaseCircular {
	raio: f64,
}

impl BaseCircular {
	fn new(raio:f64) -> BaseCircular {
		BaseCircular { raio }
	}

	fn area(&self) -> f64 {
		self.raio * self.raio * std::f64::consts::PI
	}
}


/********************
 *	Base Aquário	*
********************/

enum BaseAquario {
	Quadrada(BaseQuadrada),
	Retangular(BaseRetangular),
	Circular(BaseCircular),
}

impl BaseAquario {
	fn area(&self) -> f64 {
		match self {
			BaseAquario::Quadrada(base) => base.area(), 
			BaseAquario::Retangular(base) => base.area(),
			BaseAquario::Circular(base) => base.area(), 
		}
	}
}


/****************
 *	Aquario		*
****************/

pub struct Aquario {
	base: BaseAquario,
	altura: f64,
}


impl Aquario {
	pub fn new_base_quadrada(lado:f64,altura:f64) -> Aquario {
		Aquario {
			base: BaseAquario::Quadrada(BaseQuadrada::new(lado)),
			altura,
		}
	}
	pub fn new_base_retangular(lado_1:f64,lado_2:f64,altura:f64) -> Aquario {
		Aquario {
			base: BaseAquario::Retangular(BaseRetangular::new(lado_1,lado_2)),
			altura,
		}
	}
	pub fn new_base_circular(raio:f64,altura:f64) -> Aquario {
		Aquario {
			base: BaseAquario::Circular(BaseCircular::new(raio)),
			altura,
		}
	}
	pub fn volume(&self) -> f64 {
		self.altura * self.base.area()
	}

}



