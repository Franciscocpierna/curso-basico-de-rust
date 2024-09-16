/*
	Tipos Genéricos: Parte 1		[10.1. Generic Data Types]
	

Baseado no livro:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/



fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}



fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}



// Parâmetro genérico, mas só aqueles que suportam '>'


//fn largest<T>(list: &[T]) -> &T {
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
	let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}





fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("P1: The largest number is {}", result);

	let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("P2: The largest char is {}", result);


    println!("P3: Genérico: The largest number is {}", largest(&number_list) );

	println!("P4: Genérico: The largest char is {}",  largest(&char_list) );


	let float_list = vec![1.1, 2.2, 3.3, 4.4];
 	let result = largest(&float_list);
	println!("The largest float is {}", result);

}



