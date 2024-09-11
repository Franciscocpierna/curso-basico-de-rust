/*
	Enum			[6.1. Defining an Enum]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



// Enum simples
#[derive(Debug)]
enum TipoEndIp {
	V4,
	V6,
}


fn exemplo_enum_simples() {
	let tipo1 = TipoEndIp::V4;
	let tipo2: TipoEndIp;

	tipo2 = TipoEndIp::V6;

	println!("\nexemplo_enum_simples");
	println!("tipo1={:?}   tipo2={:?}", tipo1, tipo2);
}




// Usando uma struct
#[derive(Debug)]
struct EndIp{
	tipo: TipoEndIp,
	endereco: String,
}

fn exemplo_com_struct() {
	let home = EndIp {
		tipo: TipoEndIp::V4,
		endereco: String::from("127.0.0.1"),
	};

	let loopback = EndIp {
		tipo: TipoEndIp::V6,
		endereco: String::from("::1"),
	};

	println!("\nexemplo_com_struct");	
	println!("home {:?}    loopback {:?}", home, loopback);
}




// Usando enum com valores
#[derive(Debug)]
enum IpAddr {
	V4(String),
	V6(String),
}


fn exemplo_com_enum() {
	let home = IpAddr::V4(String::from("127.0.0.1"));
	let loopback = IpAddr::V6(String::from("::1"));

	println!("\nexemplo_com_enum");
	println!("home {:?}    loopback {:?}", home, loopback);
}



// Usando enum com valores de tipos diferentes
#[derive(Debug)]
enum IpAddrDif {
	V4(u8, u8, u8, u8),
	V6(String),
}

fn exemplo_com_enum2() {
	let home = IpAddrDif::V4(127, 0, 0, 1);
	let loopback = IpAddrDif::V6(String::from("::1"));

	println!("\nexemplo_com_enum2");
	println!("home {:?}    loopback {:?}", home, loopback);
}





fn main() {
	exemplo_enum_simples();
	exemplo_com_struct();
	exemplo_com_enum();
	exemplo_com_enum2();
}


