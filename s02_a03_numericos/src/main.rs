/* Tipos de Dados: Tipos Escalares		[3.2. Data Types: Scalar Types]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/




// Velocidade máxima de qualquer veículo em metros por segundo
const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);

// Comprimento máximo de qualquer veículo em metros
// const COMPRIMENTO_MAXIMO = 22;


/*
Table 3-1: Integer Types in Rust

Length	Signed	Unsigned
8-bit	i8		u8
16-bit	i16		u16
32-bit	i32		u32
64-bit	i64		u64
128-bit	i128	u128
arch	isize	usize

Obs1: i32 é default para inteiros
Obs2: Em caso de overflow temos "panico na execução" (debug mode) ou "dá a volta" (release mode)
Obs3: Existem vários métodos na biblioteca padrão para lidar com overflow
Obs4: Existem dois tipos de ponto flutuante: f32 e f64 (default)
*/


/*
Table 3-2: Integer Literals in Rust

Number literals		Example
Decimal				98_222
Hex					0xff
Octal				0o77
Binary				0b1111_0000
Byte (u8 only)		b'A'

Obs1: Ponto flutuante aceita  7.6e-2  ou  0.076
*/




fn main() {
 	let chassi: i32 = 123456;				// identificação de um carro
 	let acel_max: f64 = 3.0;				// metros por segundo ao quadrado
 	let acel_min: f64 = -10.0;				// metros por segundo ao quadrado
	//let vel_max: f32 = VELOCIDADE_MAXIMA;	//  as f32;	// metros por segundo
	let comprimento: i32 = 4;				// metros
	let posicao_atual: f64 = -100.0;		// metros do cruzamento
	let vel_atual: f64 = 0.0;				// metros por segundo
	let acel_atual: f64 = 0.0;				// metros por segundo ao quadrado


	// adição
	let sum = posicao_atual + 10.0;

	// subtração
	let difference = vel_atual - 4.3;

	// multiplicação
	let product = comprimento * 2;			// pode 2.0 ???

	// divisão
	let quotient = acel_atual / 2.0;
	let floored = 2 / 3;					// truncado

	// resto da divisão
	let remainder = 43 % 5;


	// transformação de tipos
	let xxx :f64 = 123.55;

	//let yyy = xxx + 88;
	//let yyy = xxx + 88f64;
	//let yyy = xxx + 88 as f64;
	
	
	//https://doc.rust-lang.org/std/primitive.f64.html
	println!("trunc {}, round {}, ceil {}, floor {}", 
				xxx.trunc(), xxx.round(), xxx.ceil(), xxx.floor());


	println!("Alô numéricos!");
}


