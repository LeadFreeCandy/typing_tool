use std::io::{Write, stdout, stdin};
use crossterm::{QueueableCommand, ExecutableCommand, cursor};
use crossterm::terminal::{Clear, ClearType, enable_raw_mode};
use crossterm::{execute, Result, style::Print};
use crossterm::event::{read, Event, KeyEvent, KeyModifiers, KeyCode};

use crossterm::style::Stylize;

use std::thread::sleep;
use std::time::{Duration, Instant};

use super::data::*;

pub fn setup() -> Result<()> {
    enable_raw_mode()?;
    
    let mut stdout = stdout();

    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(cursor::MoveTo(0,0))?;

    stdout.queue(cursor::SetCursorShape(cursor::CursorShape::Line))?;

    stdout.flush()?;
    Ok(())
}


pub fn test_sentence(sentence: String)  -> Result<Sentence> {
    let mut stdout = stdout();


    // let sentence = sentence.replace(" ", "_");

    print!("{}", sentence);

    stdout.queue(cursor::MoveToColumn(0))?;

    stdout.flush()?;

    let sentence: Vec<char> = sentence.chars().collect();


    let mut delays = Vec::with_capacity(sentence.len());
    let mut pos: usize = 0;

    let mut last_inst = Instant::now();

    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(KeyEvent{code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL}) => break,
            Event::Key(KeyEvent{code: KeyCode::Esc, modifiers: _}) => break, //todo return error

            Event::Key(KeyEvent{code: KeyCode::Char(mut c), modifiers: _}) => {
                // stdout.queue(cursor::MoveTo(0,0))?;



                if pos < sentence.len(){

                    if c == sentence[pos] {
                        

                        stdout.queue(Print(c.green()))?;
                        

    
                        if delays.len() == pos{
                            delays.push(last_inst.elapsed().as_millis() as u32);
                            last_inst = Instant::now();
                        }
    
                    } else {

                        if sentence[pos] == ' ' {
                            stdout.queue(Print("_".red()))?;
                        } else {
                            stdout.queue(Print(sentence[pos].red()))?;
                        }
                        
                    }
    
                    stdout.flush()?;

                    pos += 1;
                }

                if delays.len() >= sentence.len() {
                    break
                }

            },

            Event::Key(KeyEvent{code: KeyCode::Backspace, modifiers: _}) => {
                pos -= 1;

                stdout.queue(cursor::MoveLeft(1))?;
                stdout.queue(Print(sentence[pos]))?;
                stdout.queue(cursor::MoveLeft(1))?;


                stdout.flush()?;
                
            },
            _ => {},
        }
    }
    delays.remove(0);

    stdout.execute(cursor::MoveToNextLine(1))?;

    Ok(Sentence::new(sentence, delays))
}

