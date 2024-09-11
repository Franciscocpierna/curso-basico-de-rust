/*
	Match			[6.2. The match Control Flow Construct]

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/

*/





enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


fn value_in_cents(coin: Coin) -> u8 {
	match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


enum CoinYear {
    Penny,
    Nickel,
    Dime,
    Quarter(usize),
}



fn value_in_cents2(coin: CoinYear) -> usize {
    match coin {
        CoinYear::Penny => 1,
        CoinYear::Nickel => 5,
        CoinYear::Dime => 10,
        CoinYear::Quarter(ano) => {
            println!("Quarter do ano {}!", ano);
            25
        }
    }
}





fn main() {
	let m1 = CoinYear::Penny;
	let m2 = CoinYear::Quarter(1999);

	println!("value_in_cents2(m1) retornou {}", value_in_cents2(m1));
	println!("...");
	println!("value_in_cents2(m2) retornou {}", value_in_cents2(m2));

}
