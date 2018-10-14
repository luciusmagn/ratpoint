extern crate easycurses;

use std::io::Read;
use std::fs::File;
use std::env::args;

use easycurses::*;
use easycurses::Color::*;

trait Interpret {
	fn interpret(&self, &mut EasyCurses, bool);
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
			| "let"
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
			"Typ" => true,
			x if x.starts_with("\"") && x.ends_with("\"") => true,
			x if x.starts_with("'") && x.ends_with("'") => true,
			x if x.starts_with("::") => true,
			_ => false,
		}
	}
}

fn tokenize(s: &str) -> Vec<String> {
	let mut v = Vec::new();
	let mut current: String = String::new();
	let mut chars = s.chars().peekable();

	while let Some(next) = chars.next() {
		match next {
			c if c.is_whitespace() => { v.push(current.clone()); current.clear(); v.push(c.to_string()) },
			c @ '{' | c @ '}' | c @ '(' | c @ ')' | c @ ';' | c @ '<' | c @ '>' | c @ '[' | c @ ']' |
			c @ '&' | c @ '@' | c @ ',' | c @ '=' | c @ '+' | c @ '-' | c @ '*' | c @ '|' => {
				v.push(current.clone());
				v.push(c.to_string());
				current.clear();
			},
			'"' => {
				current.push('"');

				while let Some(next) = chars.next() {
					current.push(next);
					if next == '"' {
						break;
					}
				}

				v.push(current.clone());
				current.clear();
			},
			'\'' => {
				current.push('\'');

				while let Some(next) = chars.next() {
					current.push(next);
					if next == '\'' {
						break;
					}
				}

				v.push(current.clone());
				current.clear();
			},
			'/' => match chars.peek() {
				Some(&'/') => {
					current.push('/');

					while let Some(next) = chars.next() {
						current.push(next);
						if chars.peek() == Some(&'\n') {
							break;
						}
					}

					v.push(current.clone());
					current.clear();
				},
				Some(&'*') => {
					current.push('/');
					let mut level = 1;

					while let Some(next) = chars.next() {
						current.push(next);

						match (next, chars.peek()) {
							('/', Some(&'*')) => level += 1,
							('*', Some(&'/')) => level -= 1,
							_ => (),
						}

						if level == 0 {
							break;
						}
					}

					v.push(current.clone());
					current.clear();
				},
				_ => {
					v.push(current.clone());
					v.push('/'.to_string());
					current.clear();
				}
			},
			':' => match chars.peek() {
				Some(&':') => {
					v.push(current.clone());
					current.clear();
					current.push(':');
				},
				_ => current.push(':'),
			},
			c => current.push(c),
		}
	}
	v.retain(|x| x != "");
	v
}

fn color(s: &str) -> String {
	tokenize(s).iter().fold(String::new(), |mut acc, x| {
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
	fn interpret(&self, window: &mut EasyCurses, clear: bool) {
		let mut chars = self.chars().peekable();
		let mut indent = 0;
		if clear { window.clear(); }
		window.set_bold(false);
		window.set_underline(false);
		window.set_color_pair(colorpair!(White on Black));

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
							10 => window.print("\r       •  "),
							15 => window.print("\r            >  "),
							_ => window.print(format!("\r{}  ‣  ", " ".repeat(indent - 5))),
						};
					},
					Some('<') => {
						if indent != 0 { indent -= 5; window.print("\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08"); };
						match indent {
							5 => window.print("  o  "),
							10 => window.print("  •  "),
							15 => window.print("  >  "),
							_ => window.print("  ‣  "),
						};
					},
					Some(x) => { window.print("\\"); window.print(x.to_string()); },
					None => { window.print("\\"); },
				},
				'\n' => match chars.peek() {
					Some(&'\n') => {
						chars.next();
						if indent != 0 {
							indent = 0;
						}
						window.print("\n\n");
					},
					Some(&x) => {
						window.print("\n");
						window.print(" ".repeat(indent));
					},
					None => {
						window.print("\n");
					},
				},
				'.' => match (chars.next(), chars.peek()) {
					(Some('H'), Some(&'D')) => {
						chars.next();
						let heading = chars.clone().take_while(|x| *x != '\n').collect::<String>();
						(0..heading.len()).for_each(|_| { chars.next(); });

						format!("\n  \\fB\\fU{}\\fR\n", heading.trim_left()).interpret(window, false);
					},
					(Some(x), _) => { window.print(&format!(".{}", x)); },
					(None, _)    => { window.print("."); },
				}
				x => { window.print(x.to_string()); },
			};
		}
	}

	fn compile_rust(&self) -> String {
		let chars = self.chars().chain("   ".chars()).collect::<Vec<char>>();
		let mut windows = chars.windows(3).peekable();
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
							&['.', 'r', 's'] => break,
							&[a, _,  _ ] => temp.push(a),
							_ => unreachable!(),
						}
					}

					new_string.push_str(&color(&temp));
					temp.clear();
					windows.next();
					windows.next();
				}
				x => { new_string.push(x[0]) },
			}
		}

		new_string
	}
}

fn main() {
	let mut to_pdf = false;
	let files: Vec<String> = args()
		.skip(1)
		.filter(|x| if x == "-pdf" { to_pdf = true; false } else { true })
		.map(|x| File::open(&x).expect(&format!("couldn't open file {}", x)))
		.map(|mut x| {let mut buf = String::new(); x.read_to_string(&mut buf).expect("couldn't read file"); buf})
		.collect()
	;

	if to_pdf {
		println!("\
			\n.nr fp 9\
			\n.ll 6.3i\
			\n.fo ``- % -``\

			\n.de HD\
			\n.(l L\
			\n.ft B\
			\n\\\\$1\
			\n.ft\
			\n.)l\
			\n.ps\
			\n..\

			\n.2c\
		");

		let mut odd = true;
		files.iter()
			.map(|x| x.replace(r".rs", if odd { odd = false; ".rs\n.(l L" } else { odd = true; ".)l\n.rs" }))
			.map(|x| x.compile_rust())
			.map(|x| x.replace(r"\cW", r"\fR"))
			.map(|x| x.replace(r"\fU", r"\fI"))
			.map(|x| x.replace(r"\*", ".bu\n"))
			.map(|x| x.replace(r"\-", ".bu\n"))
			.map(|x| x.replace(r"\cr", r"\fB"))
			.map(|x| x.replace(r"\cg", r"\fB"))
			.map(|x| x.replace(r"\cm", r"\fB"))
			.map(|x| x.replace(r"\cc", r"\fB"))
			.map(|x| x.replace(r"\cy", r"\fB"))
			.map(|x| x.replace(r"\cB", r"\fI"))
			.for_each(|x| println!("{}", x));

		return;
	}

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

		files[i].compile_rust().interpret(&mut window, true);
		while let Some(input) = window.get_input() {
			use Input::*;

			match input {
				// exiting
				Unknown(27) | Character('q') | KeyF5 => return,
				// first
				Character('u') | KeyBeg | KeyHome => files[{i = 0; i}].compile_rust().interpret(&mut window, true),
				// last
				Character('i') | KeyEnd => files[{i = files.len() - 1; i}].compile_rust().interpret(&mut window, true),
				// next
				Character('j')
				| Character('l')
				| Character('B')
				| Character('C')
				| KeyRight
				| KeyDown
				| KeyNPage => files[if i < files.len() - 1 { i += 1; i } else { i }].compile_rust().interpret(&mut window, true),
				// previous
				Character('h')
				| Character('k')
				| Character('A')
				| Character('D')
				| KeyLeft
				| KeyUp
				| KeyPPage => files[if i != 0 { i -= 1; i } else { i }].compile_rust().interpret(&mut window, true),
				_ => (),
			}
		}
	}
	else { eprintln!("usage: ratpoint files ...") }
}
