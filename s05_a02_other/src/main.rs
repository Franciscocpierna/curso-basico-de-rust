/*
	Match			[6.2. The match Control Flow Construct]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/





fn add_fancy_hat() {
	println!("add_fancy_hat");
}

fn remove_fancy_hat() {
	println!("remove_fancy_hat");
}

fn move_player(num_spaces: u8) {
	println!("move_player {num_spaces}");
}

fn reroll() {
	println!("reroll");
}


fn main() {
  
	let dice_roll = 9;

	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other => move_player(other),		// Todos os outros casos
	}

	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => reroll(),					// Todos os outros casos
	}

	let nada = 
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		_ => (),							// unit value
	};

	println!("nada:    {:?}", nada);

	
}

