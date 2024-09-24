/*
	Acesso ao terminal no estilo do ncurses: 'crossterm'
	

	Várias possibilidades:
		ratatui -> completo
		ncurses-rs -> minimalista, para portar código C to Rust


https://docs.rs/crossterm/latest/crossterm/

No terminal executar: 
	$ cargo add crossterm
Ele inclui:
	[dependencies]
	crossterm = "0.27.0"

----------------------

Crossterm é uma biblioteca de acesso à terminal, desenvolvida em Rust
Torna possível programar interfaces baseadas em texto para múltiplas plataformas.

Suporta todos os terminais UNIX e Windows desde o Windows 7.

Existem duas formas diferentes de executar comandos:

	Lazy Execution (execução em lotes)
		Usa uma fila onde comandos são enfileirados,
		quando Write::flush é chamado esses comandos são executados.

	Direct Execution (execução direta)
		Cada comando é executado imediatamente, contando com um flush implícito.


Comandos suportados:
    Module cursor
        Visibility - Show, Hide
        Appearance - EnableBlinking, DisableBlinking, SetCursorStyle
        Position - SavePosition, RestorePosition, MoveUp, MoveDown, MoveLeft, MoveRight, MoveTo, 
					MoveToColumn,MoveToRow, MoveToNextLine, MoveToPreviousLine
	Module event
        Keyboard events - PushKeyboardEnhancementFlags, PopKeyboardEnhancementFlags
        Mouse events - EnableMouseCapture, DisableMouseCapture
	Module style
        Colors - SetForegroundColor, SetBackgroundColor, ResetColor, SetColors
        Attributes - SetAttribute, SetAttributes, PrintStyledContent
	Module terminal
        Scrolling - ScrollUp, ScrollDown
        Miscellaneous - Clear, SetSize, SetTitle, DisableLineWrap, EnableLineWrap
        Alternate screen - EnterAlternateScreen, LeaveAlternateScreen


Módulos:
    cursor
	    A module to work with the terminal cursor
    event
    	A module to read events.
    style
    	A module to apply attributes and colors on your text.
    terminal
	    A module to work with the terminal.
    tty
    	A module to query if the current instance is a tty.
		Making it a little more convenient and safe to query whether something is a terminal teletype or not.


	UTF-8 is an encoding scheme for Unicode text. Caracteres Unicode:
	https://www.unicode.org/charts/PDF/U2580.pdf	Elementos do tipo Blocos
	https://www.unicode.org/charts/PDF/U2500.pdf	Desenho de Caixas
	etc ...
	https://en.wikipedia.org/wiki/List_of_Unicode_characters

*/


use std::io::{self,stdout,Error,Write};
use std::time::Duration;

use crossterm::{execute,ExecutableCommand, QueueableCommand,terminal,cursor};
use crossterm::style::{self,SetAttribute,Attribute,Stylize,SetColors,ResetColor,Colors,
									Print,SetForegroundColor,SetBackgroundColor};
use crossterm::style::Color::*;

use crossterm::event::{Event,KeyCode,KeyModifiers,poll};


// Espera o usuário teclar <Enter>
fn espera_enter() {
	let mut linha = String::new();
	println!("{}{}","Tecle enter".on_dark_red()," para continuar:".on_green());
	io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");
}


// Limpa toda a tela, posiciona cursor no topo à esquerda
fn _limpa_tela() -> Result<bool, Error> {
	let mut stdout = stdout();
	stdout
		.queue(cursor::MoveTo(0,0))?
		.queue(terminal::Clear(terminal::ClearType::All))?;
	stdout.flush()?;
	Ok(true)
}



// Testa modos 'execução direta' e 'execução em lote'
// https://docs.rs/crossterm/latest/crossterm/style/
fn testa_modos_escrita() -> Result<bool, Error> {

	// Saídas acontecem através dessa variável 
	let mut stdout = stdout();

	// Execução direta
	stdout.execute(terminal::Clear(terminal::ClearType::All))?;

	// Tamanho do terminal
	let (largura,altura) = terminal::size()?;
	stdout.execute(cursor::MoveTo(largura-30,altura-1))?;
	println!("largura={} altura={}", largura, altura);
	stdout.execute(cursor::MoveTo(0,0))?;
	println!("println! normal: largura={}   altura={}", largura, altura);

	// No próprio println! usando atributos
	println!("largura sublinhada={}{}{}   altura={}",Attribute::Underlined,largura,
													Attribute::NoUnderline,altura);

	// No próprio println! usando funções sobre o string
	println!("{}", format!("preto sobre branco: largura={}   altura={}", largura, altura)
							.black()
							.on_white() );

	// Usando comandos
	stdout.execute(SetAttribute(Attribute::Bold))?;
	println!("largura negrito={}   ", largura);
	stdout.execute(SetAttribute(Attribute::SlowBlink))?;
	println!("altura piscando={}   ", altura);
	stdout.execute(SetAttribute(Attribute::Reset))?;

	// Usando comandos com a macro 'execute!'
	execute!(
        io::stdout(),
        SetForegroundColor(Blue),
        SetBackgroundColor(Red),
		Print( format!("azul sobre vermelho: largura={}   altura={}\n", largura, altura) ),
        ResetColor
    )?;

	// Caracteres especiais são possíveis
	print!(" {} ",'\u{2b24}');
	espera_enter();

	// Caracteres UTF-8, retângulo em verde
	stdout.execute(terminal::Clear(terminal::ClearType::All))?;
	for y in altura/4..=altura/3 {
		for x in largura/4..=largura/3 {
			if (y == altura/4 || y == altura/3) || (x == largura/4 || x == largura/3) {
				stdout.execute( cursor::MoveTo(x,y) )?;
				stdout.execute( style::PrintStyledContent('\u{2588}'.green()) )?;
		 	}
		}
	}
	espera_enter();


	// Execução em lote
	for y in 0..=altura/2 {
		for x in 0..=largura/2 {
			if (y == 0 || y == altura/2) || (x == 0 || x == largura/2) {
				stdout
				.queue(cursor::MoveTo(x,y))?
				.queue(style::PrintStyledContent("█".magenta()))?;
			}
		}
	}
	stdout.flush()?;
	espera_enter();


	const CANTO_SUP_ESQ: char = '\u{250C}';
	const CANTO_SUP_ESQ2: char = '┌';

	stdout.execute(SetColors(Colors::new(Blue, Yellow)))?;
	print!("KKK");
	stdout.execute(ResetColor)?;
	espera_enter();

	println!("XYZ{} {} {} {} {} {} {} {} {}","xyz".cyan(), '\u{2500}','\u{2502}','\u{2510}',
				'\u{2514}','\u{2518}','\u{250C}',CANTO_SUP_ESQ,CANTO_SUP_ESQ2);
	for ch in '\u{2500}'..'\u{2520}' {
		print!("{}",ch);
	}			
	espera_enter();

	// Barra horizontal superior
	stdout
		.queue( cursor::MoveTo(2,2) )?
		.queue( style::PrintStyledContent("\u{250C}".blue()) )?
		.queue( style::PrintStyledContent("\u{2500}".repeat(4).blue()) )?
		.queue( style::PrintStyledContent("\u{2510}".blue()) )?;

	// Barra horizontal inferior
	stdout
		.queue( cursor::MoveTo(2,7) )?
		.queue( style::PrintStyledContent("\u{2514}".blue()) )?
		.queue( style::PrintStyledContent("\u{2500}".repeat(4).blue()) )?
		.queue( style::PrintStyledContent("\u{2518}".blue()) )?;

	// Barras verticais
	for y in 3..=6 {
		stdout
			.queue( cursor::MoveTo(2,y) )?
			.queue( style::PrintStyledContent("\u{2502}".blue()) )?;
	}
	for y in 3..=6 {
		stdout
			.queue( cursor::MoveTo(7,y) )?
			.queue( style::PrintStyledContent("\u{2502}".blue()) )?;
	}

	stdout.flush()?;
	stdout.execute(cursor::MoveTo(0,altura/2+7))?;
	espera_enter();

	Ok(true)

}




/*
	Raw Mode
	- Input will not be forwarded to screen
	- Input will not be processed on enter press
	- Input will not be line buffered (input sent byte-by-byte to input buffer)
	- Special keys like backspace and CTRL+C will not be processed by terminal driver
	- New line character will not be processed therefore println! can’t be used, use write! instead

	Obs: Funciona no Windows mas não no Visual Studio Code do Windows, fica 'Enter' na entrada.
*/
fn testa_leitura_com_bloqueio() -> Result<bool, Error> {
	// Saídas acontecem através dessa variável 
	let mut stdout = stdout();

	// Execução direta
	stdout.execute(terminal::Clear(terminal::ClearType::All))?;
	stdout.execute(cursor::MoveTo(0,0))?;

	terminal::enable_raw_mode()?;

	//Blocking read
	print!("Bloqueio até algo ser teclado:\r\n");

	let evento = crossterm::event::read()?;
	stdout.execute(cursor::MoveToNextLine(1))?;
	print!("{:?}\r\n", evento );

	match evento {
		Event::Key(key_event) => {

			match (key_event.code,key_event.modifiers) {
				(KeyCode::Char(x),m) if m == KeyModifiers::CONTROL => {
					print!("Control-{}\r\n", x);
					terminal::disable_raw_mode()?;
				}

				(KeyCode::Char(x), _) => {
					//panic!("ZZZZZZZZZZZZZZZZZZZZ");
					print!("{}\r\n", x );
				}
				(KeyCode::Up, _) => {
					print!("UP\r\n");
				}
				(KeyCode::Down, _) => {
					print!("DOWN\r\n");
				}
				(KeyCode::Left, _) => {
					print!("LEFT\r\n");
				}
				(KeyCode::Right, _) => {
					print!("RIGHT\r\n");
				}

				_ => {},
			}
		}
		Event::FocusGained => {},
		Event::FocusLost => {},
		Event::Mouse(_mouse_event) => {},
		Event::Paste(_s) => {},
		Event::Resize(_colunas,_linhas) => {},
	}

	// Retorna ao modo normal
	terminal::disable_raw_mode()?;
	Ok(true)
}
 




/*
	Raw Mode
	- Input will not be forwarded to screen
	- Input will not be processed on enter press
	- Input will not be line buffered (input sent byte-by-byte to input buffer)
	- Special keys like backspace and CTRL+C will not be processed by terminal driver
	- New line character will not be processed therefore println! can’t be used, use write! instead

	Obs: Funciona no Windows mas não no Visual Studio Code do Windows, fica 'Enter' na entrada.
*/
fn testa_leitura_sem_bloqueio() -> Result<bool, Error> {

	terminal::enable_raw_mode()?;

	//Non-blocking read
	print!("Bloqueio até algo ser teclado ou 2s:\r\n");
	if poll(Duration::from_secs(2))? {
		// É garantido que 'read' não vai bloquear dado que 'poll' retornou 'Ok(true)'
		let evento = crossterm::event::read()?;
		print!("{:?}\r\n", evento);

		match evento {
			Event::Key(key_event) => {

				match (key_event.code,key_event.modifiers) {
					(KeyCode::Char(x),m) if m == KeyModifiers::CONTROL => {
						print!("Control-{}\r\n", x);
						terminal::disable_raw_mode()?;
					}

					(KeyCode::Char(x), _) => {
						print!("{}\r\n", x);
					}
					(KeyCode::Up, _) => {
						print!("UP\r\n");
					}
					(KeyCode::Down, _) => {
						print!("DOWN\r\n");
					}
					(KeyCode::Left, _) => {
						print!("LEFT\r\n");
					}
					(KeyCode::Right, _) => {
						print!("RIGHT\r\n");
					}
					_ => {},
				}
			}
			Event::FocusGained => {},
			Event::FocusLost => {},
			Event::Mouse(_mouse_event) => {},
			Event::Paste(_s) => {},
			Event::Resize(_colunas,_linhas) => {},

		}
	} else {
		print!("Timeout, nenhum evento disponível\r\n");
    }

	// Retorna ao modo normal
	terminal::disable_raw_mode()?;
	Ok(true)
}
 




struct Limpeza;

impl Drop for Limpeza {
	fn drop(&mut self) {
		terminal::disable_raw_mode().expect("Falha ao sair do raw mode");
    }
}



fn main() {
	let _limpeza = Limpeza;
    println!("Início");

	// Saídas acontecem através dessa variável 
	let mut stdout = stdout();

	if testa_modos_escrita().is_err() {
		let _ = terminal::disable_raw_mode();
		println!("testa_modos_escrita() retornou erro.");	// panic! ???
	}


	stdout.execute(cursor::MoveToNextLine(1)).expect("Erro ao mover o cursor");
	println!("É possível fazer uma entrada de dados usual:");
	let mut linha = String::new();
	io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");
	println!("leu>>>{}", linha);
	espera_enter();

	if testa_leitura_com_bloqueio().is_err() {
		let _ = terminal::disable_raw_mode();
		println!("testa_leitura_com_bloqueio() retornou erro.");	// panic! ???
	}


	stdout.execute(cursor::MoveToNextLine(1)).expect("Erro ao mover o cursor");
	println!("Tecle enter para fazer uma entrada de dados sem bloqueio:  ");
	io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");

	if testa_leitura_sem_bloqueio().is_err() {
		let _ = terminal::disable_raw_mode();
		println!("testa_leitura_sem_bloqueio() retornou erro.");	// panic! ???
	}

	println!("\n");
	println!("Fim");
}


