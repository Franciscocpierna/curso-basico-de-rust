
/********************
 *	Bomba De Ar		*
********************/
pub struct BombaDeAr {
	potencia: f64,
	voltagem: u32,
}

impl BombaDeAr {
	pub fn new(potencia:f64,voltagem:u32) -> BombaDeAr {
		BombaDeAr {
			potencia,
			voltagem,
		}
	}
	pub fn _potencia(&self) -> f64 {
		self.potencia
	}
	pub fn _voltagem(&self) -> u32 {
		self.voltagem
	}
	pub fn volume(&self) -> f64 {
		if self.potencia > 100.0 {
			122.2
		} else {
			88.0
		}
	}
}

