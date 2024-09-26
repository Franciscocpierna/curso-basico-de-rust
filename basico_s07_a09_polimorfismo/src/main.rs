/*
Nesta aula:

	Como lidar com variáveis que podem ter tipos diferentes (polimorfismo) ?

	O enum 'embrulha' os diversos tipos específicos, provendo um tipo geral.


*/

mod aquario;
mod bomba_de_ar;

use aquario::Aquario;
use bomba_de_ar::BombaDeAr;



enum Produto {
	Aquario(Aquario),
	BombaDeAr(BombaDeAr),
}

struct ItemEstoque {
	id: u32,
	preco: f64,
	produto: Produto,
}
impl ItemEstoque {
	fn new(id:u32,preco:f64,produto:Produto) -> ItemEstoque {
		ItemEstoque {
			id,
			preco,	
			produto,
		}
	}
	fn id(&self) -> u32 {
		self.id
	}
	fn preco(&self) -> f64 {
		self.preco
	}
	fn volume(&self) -> f64 {
		match &self.produto {
			Produto::Aquario(aquario) => aquario.volume(),
			Produto::BombaDeAr(bomba) => bomba.volume(),
		}
	}
}




fn main() {
	let mut estoque = Vec::new();

	let aquario = Produto::Aquario(Aquario::new_base_quadrada(5.0,10.0) );
	estoque.push( ItemEstoque::new(1,11.11, aquario) );

	let aquario = Produto::Aquario(Aquario::new_base_quadrada(6.0,10.0) );
	estoque.push( ItemEstoque::new(2,22.22, aquario) );

	let aquario = Produto::Aquario(Aquario::new_base_retangular(2.0,3.0,10.0) );
	estoque.push( ItemEstoque::new(3,33.33, aquario) );

	let aquario = Produto::Aquario(Aquario::new_base_retangular(2.0,4.0,10.0) );
	estoque.push( ItemEstoque::new(4,44.44, aquario) );

	let aquario = Produto::Aquario(Aquario::new_base_circular(5.0,10.0) );
	estoque.push( ItemEstoque::new(5,55.55, aquario) );

	let aquario = Produto::Aquario(Aquario::new_base_circular(7.0,10.0) );
	estoque.push( ItemEstoque::new(6,66.66, aquario) );


	let bomba = Produto::BombaDeAr(BombaDeAr::new(100.0,110) );
	estoque.push( ItemEstoque::new(7,77.77, bomba) );

	let bomba = Produto::BombaDeAr(BombaDeAr::new(200.0,220) );
	estoque.push( ItemEstoque::new(8,88.88, bomba) );



	for item in estoque.iter() {
		println!("id={}  preco={:.2}  volume={:.1}",item.id(),item.preco(),item.volume());
	}

}
