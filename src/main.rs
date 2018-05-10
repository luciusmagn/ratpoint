extern crate easycurses;

use std::io::Read;
use std::fs::File;
use std::env::args;

use easycurses::*;

trait Interpret {
    fn interpret(&self, &mut EasyCurses);
}

impl Interpret for String {
    fn interpret(&self, window: &mut EasyCurses) {
        let mut chars = self.chars();
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
                        Some('R') => { window.set_bold(false); window.set_underline(false); }
                        _ => (),
                    }
                    _ => (),
                },
                x => { window.print(x.to_string()); },
            };
        }
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
