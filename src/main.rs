extern crate easycurses;

use std::io::Read;
use std::fs::File;
use std::env::args;

use easycurses::*;
use easycurses::Color::*;

trait Interpret {
	fn interpret(&self, &mut EasyCurses);
	fn compile_rust(&self) -> String;
}

trait Language {
	fn is_red(&self) -> bool;
	fn is_green(&self) -> bool;
	fn is_magenta(&self) -> bool;
	fn is_cyan(&self) -> bool;
	fn is_blue(&self) -> bool;
	fn is_yellow(&self) -> bool;
}

impl Rust for Language {
	fn is_red(&self) -> bool { self.is_red() }
	fn is_green(&self) -> bool { self.is_green() }
	fn is_magenta(&self) -> bool { self.is_magenta() }
	fn is_cyan(&self) -> bool { self.is_cyan() }
	fn is_blue(&self) -> bool { self.is_blue() }
	fn is_yellow(&self) -> bool { self.is_yellow() }
}


trait Rust {
	fn is_red(&self) -> bool;
	fn is_green(&self) -> bool;
	fn is_magenta(&self) -> bool;
	fn is_cyan(&self) -> bool;
	fn is_blue(&self) -> bool;
	fn is_yellow(&self) -> bool;
}

impl Rust for str {
	fn is_red(&self) -> bool {
		match self {
			"mod"
			| "use"
			| "extern"
			| "crate"
			| "self" => true,
			_ => false,
		}
	}

	fn is_green(&self) -> bool {
		match self {
			"mut"
			| "pub"
			| "as" => true,
			x if x.ends_with('!') => true,
			_ => false,
		}
	}

	fn is_magenta(&self) -> bool {
		match self {
			"match"
			| "if"
			| "else"
			| "trait"
			| "struct"
			| "enum"
			| "i8"
			| "i16"
			| "i32"
			| "i64"
			| "i128"
			| "u8"
			| "u16"
			| "u32"
			| "u64"
			| "u128"
			| "f32"
			| "f64"
			| "f128"
			| "usize"
			| "isize" => true,
			_ => false,
		}
	}

	fn is_cyan(&self) -> bool {
		match self {
			"fn"
			| "impl"
			| "bool"
			| "str" => true,
			_ => false,
		}
	}

	fn is_blue(&self) -> bool {
		match self {
			x if x.starts_with("//") => true,
			x if x.starts_with("/*") => true,
			_ => false,
		}
	}

	fn is_yellow(&self) -> bool {
		match self {
			x if x.starts_with("\"") && x.ends_with("") => true,
			x if x.starts_with("::") => true,
			_ => false,
		}
	}
}

fn tokenize(s: &str) -> Vec<&str> {
	unimplemented!()
}

fn color(s: &str) -> String {
	tokenize(s).into_iter().fold(String::new(), |mut acc, x| {
		match x {
			x if x.is_red()     => acc.push_str(&"\\cr".chars().chain(x.chars()).chain("\\cW".chars()).collect::<String>()),
			x if x.is_green()   => acc.push_str(&"\\cg".chars().chain(x.chars()).chain("\\cW".chars()).collect::<String>()),
			x if x.is_magenta() => acc.push_str(&"\\cm".chars().chain(x.chars()).chain("\\cW".chars()).collect::<String>()),
			x if x.is_cyan()    => acc.push_str(&"\\cc".chars().chain(x.chars()).chain("\\cW".chars()).collect::<String>()),
			x if x.is_blue()    => acc.push_str(&"\\cb".chars().chain(x.chars()).chain("\\cW".chars()).collect::<String>()),
			x if x.is_yellow()  => acc.push_str(&"\\cy".chars().chain(x.chars()).chain("\\cW".chars()).collect::<String>()),
			x => acc.push_str(&x),
		}
		acc
	})
}

impl Interpret for String {
	fn interpret(&self, window: &mut EasyCurses) {
		let mut chars = self.chars();
		let mut indent = 0;
		window.clear();

		while let Some(next) = chars.next() {
			match next {
				'\\' => match chars.next() {
					Some('\\') => { window.print("\\"); },
					Some('f') => match chars.next() {
						Some('U') | Some('I') => { window.set_underline(true); },
						Some('u') | Some('i') => { window.set_underline(false); },
						Some('B') => { window.set_bold(true); },
						Some('b') => { window.set_bold(false); },
						Some('R') => { window.set_bold(false); window.set_underline(false); },
						_ => (),
					},
					Some('c') => match chars.next() {
						Some('W') => { window.set_color_pair(colorpair!(White on Black)); },
						Some('B') => { window.set_color_pair(colorpair!(Black on Black)); },
						Some('b') => { window.set_color_pair(colorpair!(Blue on Black)); },
						Some('y') => { window.set_color_pair(colorpair!(Yellow on Black)); },
						Some('g') => { window.set_color_pair(colorpair!(Green on Black)); },
						Some('m') => { window.set_color_pair(colorpair!(Magenta on Black)); },
						Some('c') => { window.set_color_pair(colorpair!(Cyan on Black)); },
						Some('r') => { window.set_color_pair(colorpair!(Red on Black)); },
						_ => (),
					},
					Some('*') => {
						indent += 5;
						match indent {
							5 => window.print("  o  "),
							10 => window.print("  •  "),
							15 => window.print("  >  "),
							_ => window.print("  ‣  "),
						};
					},
					Some('-') => {
						match indent {
							5 => window.print("\r  o  "),
							10 => window.print("\r        •  "),
							15 => window.print("\r             >  "),
							_ => window.print(&format!("\r{}  ‣  ", " ".repeat(indent - 5))),
						};
					},
					_ => (),
				},
				'\n' => match chars.next() {
					Some('\n') => {
						if indent != 0 {
							indent = 0;
						}
						window.print("\n\n");
					},
					Some(x) => {
						window.print("\n");
						window.print(" ".repeat(indent));
						window.print(x.to_string());
					},
					None => {
						window.print("\n");
					},
				}
				x => { window.print(x.to_string()); },
			};
		}
	}

	fn compile_rust(&self) -> String {
		let chars = self.chars().collect::<Vec<char>>();
		let mut windows = chars.windows(3);
		let mut new_string = String::new();
		let mut temp = String::new();

		while let Some(next) = windows.next() {
			match next {
				&['.', 'r', 's'] => {
					windows.next();
					windows.next();
					windows.next();

					while let Some(view) = windows.next() {
						match view {
							&['.', 'r', 's'] => {

								break;
							},
							&[a, _,  _ ] => temp.push(a),
							&[a, b] => temp.push_str(&format!("{}{}", a, b)),
							&[a] => temp.push(a),
							&[] => break,
							_ => unreachable!(),
						}
					}

					new_string.push_str(&color(&temp));
					temp.clear();
				}
				x => new_string.push(x[0]),
			}
		}

		new_string
	}
}

fn main() {
	let files: Vec<String> = args()
		.skip(1)
		.map(|x| File::open(&x).expect(&format!("couldn't open file {}", x)))
		.map(|mut x| {let mut buf = String::new(); x.read_to_string(&mut buf).expect("couldn't read file"); buf})
		.collect()
	;

	if args().count() != 1 {
		let mut i = 0;
		let mut window = match EasyCurses::initialize_system() {
			Some(w) => w,
			None => {eprintln!("failed to init curses"); return}
		};

		window.set_cursor_visibility(CursorVisibility::Invisible);
		window.set_echo(false);
		window.clear();
		window.refresh();

		files[i].interpret(&mut window);
		while let Some(input) = window.get_input() {
			use Input::*;

			match input {
				// exiting
				Unknown(27) | Character('q') | KeyF5 => return,
				// first
				Character('u') | KeyBeg | KeyHome => files[{i = 0; i}].interpret(&mut window),
				// last
				Character('i') | KeyEnd => files[{i = files.len() - 1; i}].interpret(&mut window),
				// next
				Character('j')
				| Character('l')
				| Character('B')
				| Character('C')
				| KeyRight
				| KeyDown
				| KeyNPage => files[if i < files.len() - 1 { i += 1; i } else { i }].interpret(&mut window),
				// previous
				Character('h')
				| Character('k')
				| Character('A')
				| Character('D')
				| KeyLeft
				| KeyUp
				| KeyPPage => files[if i != 0 { i -= 1; i } else { i }].interpret(&mut window),
				_ => (),
			}
		}
	}
	else { eprintln!("usage: ratpoint files ...") }
}
