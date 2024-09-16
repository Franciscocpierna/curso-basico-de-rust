/*
	Tipos Genéricos: Parte 1 			[10.1. Generic Data Types]


Baseado em:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/


// Parâmetro é um slice de valores i32
fn largest(list: &[i32]) -> &i32 {
	let mut largest = &list[0];

	for item in list {
		if item > largest {
		//if *item > *largest {
			largest = item;
        }
    }

	largest
}




#[derive(Debug)]
struct UmaStruct {
	i1: i32,
	i2: i32,
}

#[derive(Debug)]
struct OutraStruct {
	f1: f64,
	f2: f64,
}


fn primeiroUma(lista: &[UmaStruct]) -> &UmaStruct {
	&lista[0]
}

fn primeiroOutra(lista: &[OutraStruct]) -> &OutraStruct {
	&lista[0]
}

fn primeiro<T>(lista: &[T]) -> &T {
	&lista[0]
}


fn main() {

	// Uma função evita a duplicação de código
	let vector1_numeros = vec![34, 50, 25, 100, 65];
	let maior1 = largest(&vector1_numeros);
	println!("P1: O maior número é {}", maior1);

	let vector2_numeros = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	let maior2 = largest(&vector2_numeros);
	println!("P2: O maior número é {}", maior2);


	// Array tem similaridade com vector
	let array3_numeros = [1, 2, 3, 4, 5];
	let maior3 = largest(&array3_numeros);
	println!("P3: O maior número é {}", maior3);


	// A função não funciona com floats, aqui vai entrar o Genérico mais adiante
	let array4_numeros = [1.0, 2.0, 3.0, 4.0, 5.0];
//	let maior4 = largest(&array4_numeros);
//	println!("P4: O maior número é {}", maior4);
    
    
	// A função não funciona com floats, aqui vai entrar o Genérico mais adiante
	let vector5_numeros = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//	let maior5 = largest(&vector5_numeros);
//	println!("P5: O maior número é {}", maior5);


	// E se o elemento for algo mais complexo ?
	let array6_struct = [ UmaStruct{i1:1,i2:1}, 
											UmaStruct{i1:2,i2:2}, 
											UmaStruct{i1:3,i2:3} ];

	let vector7_struct = vec![OutraStruct{f1:1.0,f2:1.0}, 
												OutraStruct{f1:2.0,f2:2.0}, 
												OutraStruct{f1:3.0,f2:3.0} ];

	println!("O primeiro elemento de array6_struct é {:?}", primeiroUma(&array6_struct) );
	println!("O primeiro elemento de vector7_struct é {:?}", primeiroOutra(&vector7_struct) );

	//println!("O primeiro elemento de array6_struct é {:?}", primeiroOutra(&array6_struct) );
	//println!("O primeiro elemento de vector7_struct é {:?}", primeiroUma(&vector7_struct) );

	println!("O primeiro elemento de array6_struct é {:?}", primeiro(&array6_struct) );
    println!("O primeiro elemento de vector7_struct é {:?}", primeiro(&vector7_struct) );

	// Primeiro deveria retornar Option<&T>, None se lista estiver vazia
	//let x: [i32;0] = [];
	//primeiro( &x );


}	




