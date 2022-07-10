use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::color::*;


use std::io::{Write, stdout, stdin};

const SENTENCE: &'static str = "The quick brown fox jumps over the lazy dog";
fn main() {


    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}{}{}\n{}{}{}",
           Fg(Red),
           termion::clear::All,
           termion::cursor::Goto(1, 1),

            SENTENCE,

           termion::cursor::Goto(1, 1),
           termion::cursor::BlinkingBar,
           termion::cursor::Show,
        //    termion::cursor::Hide)
    )
    .unwrap();
    stdout.flush().unwrap();



    for c in stdin.keys() {
        // write!(stdout,
        //        "{}{}",
        //        termion::cursor::Goto(1, 1),
        //        termion::clear::CurrentLine)
        //         .unwrap();

        match c.unwrap() {
            
            // Key::Ctrl('c') => break,
            Key::Esc => {
                write!(stdout,
                    "{}",
                    Fg(White),
                 //    termion::cursor::Hide)
             )
             .unwrap();
             break
            },
            // Key::Char(c) => print!("{}", c),
            Key::Char('\n') => {},
            Key::Char(c) => {
                write!(stdout,
                    "{}{}",
                    Fg(Green),
                    c,
                 //    termion::cursor::Hide)
             )
             .unwrap();
            }
            
            _ => {}
        }
        stdout.flush().unwrap();
    }

    // write!(stdout, "{}", termion::cursor::Show).unwrap();
}