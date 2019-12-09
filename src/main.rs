mod parse;
use crate::parse::*;

mod card;
use crate::card::*;

mod event;
use crate::event::{Event, Events};

mod app;
use crate::app::App;

mod ui;
use crate::ui::Ui;

use std::{env, process, fs, io::{Error, prelude::*}};
use termion::event::Key;

fn main() -> Result<(), failure::Error>{

    // Setup event handlers
    let events = Events::new();

    // Parsing file
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Program requires two arguments: <input_file_location> <output_file_name>");
        process::exit(1);
    }
    let markdown = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");
    let cards = match parse_md(&markdown) {
        Ok(parsed_md) => { parsed_md },
        Err(e) => { println!("Error: {}",e); return Ok(()) }
    };

    let mut app = App::new(cards);
    let mut ui = Ui::new()?;

    loop {
        ui.render(&app)?;

        // Event handlers
        if app.is_editing {
            if let Event::Input(input) = events.next()? { match input {
                Key::Esc => {
                    app = app.disable_edit();
                }
                Key::Char(char) => {
                    app.push(char);
                }
                Key::Backspace => {
                    app.pop();
                }
                Key::Left => {
                    app.move_cursor_left();
                }
                Key::Right => {
                    app.move_cursor_right();
                }
                _ => {}
            }}
        } else if let Event::Input(input) = events.next()? { match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.selected = None;
                }
                Key::Down => {
                    app.next();
                }
                Key::Up => {
                    app.prev();
                }
                Key::Char('f') => {
                    app.enable_edit(Attribute::Front);
                }
                Key::Char('b') => {
                    app.enable_edit(Attribute::Back);
                }
                Key::Char('t') => {
                    app.enable_edit(Attribute::MainTag);
                }
                Key::Char('d') => {
                    app.cards.remove(app.last_selected);
                    if app.last_selected >= app.cards.len() {
                        app.selected = Some(0);
                        app.last_selected = 0;
                    }
                }
                Key::Char('n') => {
                    app.cards.push(Card::new("New Card".to_string(), "".to_string(), "".to_string()));
                }
                Key::Char('x') => {
                    export(&app.cards, args[2].clone())?;
                    break;
                }
                _ => {}
        }}

    }
    Ok(())
}

// Export cards by creating a text file Anki can import
fn export(cards: &[Card], name: String) -> Result<(), Error> {
    let mut output = "".to_string();
    for card in cards {
        output += &card.export();
    }
    let mut output_file = fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(name + ".txt")?;
    if let Err(e) = writeln!(output_file, "{}", output) {
        eprintln!("Couldn't write to file: {}", e);
    }
    Ok(())
}
