/*
	Devo usar 'String', '&String', '&str' ou '&'static str' ?

Baseado em:
The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community
This version of the text assumes you’re using Rust 1.67.1 (released 2023-02-09) or later
https://doc.rust-lang.org/stable/book/


*/


/// Recebe parâmetros do tipo String
fn recebe_string(s1: String, mut s2: String) {
	println!("recebe_string recebeu s1: {}",s1);
	println!("recebe_string recebeu s2: {}",s2);
	//s1.push_str("zzz");		// cannot mutate immutable variable `s1`
	s2.push_str("zzz");
}


/// Recebe parâmetros do tipo Referência para String
fn recebe_ref_string(s1: &String, s2: &mut String) {
	println!("recebe_ref_string recebeu s1: {}",s1);
	println!("recebe_ref_string recebeu s2: {}",s2);
	//s1.push_str("zzz");		// cannot borrow `*s1` as mutable, as it is behind a `&` reference
	s2.push_str("zzz");
}


/// Recebe parâmetros do tipo String Slice
fn recebe_string_slice( s1: &str, s2: &str, s3: &mut str, s4: &mut str) {
	println!("recebe_string_slice recebeu s1: {}",s1);
	println!("recebe_string_slice recebeu s2: {}",s2);
	println!("recebe_string_slice recebeu s3: {}",s3);
	println!("recebe_string_slice recebeu s4: {}",s4);

	//s3 = "ooooooooooooooo";		// expected &mut str, found &str
	//s3.push_str("zzz");			// no method named `push_str` found
	//s1.make_ascii_uppercase();	// cannot borrow `*s1` as mutable, as it is behind a `&` reference
	s3.make_ascii_lowercase();
	s4.make_ascii_uppercase();
	println!("recebe_string_slice s3.make_ascii_uppercase(): {}",s3);
	println!("recebe_string_slice s4.make_ascii_uppercase(): {}",s4);
}


/// Recebe parâmetros do tipo String Literal
fn recebe_string_literal( s1: &str, s2: &'static str) {
	println!("recebe_string_literal recebeu s1: {}",s1);
	println!("recebe_string_literal recebeu s2: {}",s2);
}




fn main() {

	println!("\n-----Exemplos com String-----");
	let sa = String::from("aaaa");
	let mut sb = String::from("bbbb");
	println!("main: antes de recebe_string");
	println!("sa: {}     sb: {}\n", sa, sb);

	recebe_string(sa, sb);
	println!("main: depois de recebe_string");
	//println!("main sb: {} {}", sa, sb);		// borrow of moved value: `sa` `sb`


	println!("\n-----Exemplos com &String-----");
	let sc = String::from("cccc");
	let mut sd = String::from("dddd");
	println!("main: antes de recebe_ref_string");
	println!("sc: {}     sd: {}\n", sc, sd);

	recebe_ref_string(&sc, &mut sd);
	println!("main: depois de recebe_ref_string");
	println!("sc: {}     sd: {}", sc, sd);


	println!("\n-----Exemplos com string slice-----");
	let se = String::from("eeeeeeee");
	let sse = se.as_str();
	let sf = String::from("ffffffff");
	let ssf = &sf[0..4];
	let mut sg = String::from("gGgGgGgG");
	let ssg = sg.as_mut_str();
	let mut sh = String::from("hHhHhHhH");
	let ssh: &mut str = &mut String::from("hHhHhHhH");
	println!("main: antes de recebe_string_slice");
	println!("sse: {}     ssf: {}     ssg: {}     ssh: {}\n", sse, ssf, ssg, ssh);

	//recebe_ref_string(sse, ssh);	//arguments to this function are incorrect
											// &str não pode substituir &String

	recebe_string_slice(sse, ssf, ssg, ssh);
	println!("main: depois de recebe_string_slice");
	println!("sse: {}     ssf: {}     ssg: {}     ssh: {}\n", sse, ssf, ssg, ssh);
	
	recebe_string_slice(&se, "qwerty", ssg, &mut sh);
	println!("main: depois de recebe_string_slice com &String e String Estático");
	println!("sse: {}     ssf: {}     ssg: {}     ssh: {}\n", sse, ssf, ssg, ssh);



	println!("\n-----Exemplos com string literal-----");

	let sli = "iiiiiiii";
	let mut slj = "jjjjjjjj";

	println!("main: antes de recebe_string_literal");
	println!("sli: {}     slj: {}", sli, slj);
	slj = "JJJJJJJJ";
	println!("sli: {}     slj: {}\n", sli, slj);

	recebe_string_literal("hhhh", "iiii");

	recebe_string_literal(sli, "iiii");

	recebe_string_literal("zzzz", slj);

	let sk = "kkkkkkkk".to_string();
	let ssk = &sk[0..4];
	//recebe_string_literal("zzzz", ssk);	// argument requires that borrow lasts for `'static`

}

