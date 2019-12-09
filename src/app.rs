use tui::style::{Color, Style};
use tui::widgets::Text;
use crate::parse::Attribute;
use crate::card::Card;

pub struct App {
    pub cards: Vec<Card>,
    default_text: String,
    pub text: String,
    pub title: &'static str,
    pub is_editing: bool,
    field_editing: Attribute,
    pub cursor_location: usize,
    pub selected: Option<usize>,
    pub last_selected: usize,
    pub highlight_style: Style,
}

impl App {

    pub fn new(cards: Vec<Card>) -> App {
        let text = "[q] quit | [f] edit front | [b] edit back | [t] edit tags | [n] new | [d] delete | [x] export";
        App {
            cards,
            default_text: text.to_string(),
            text: text.to_string(),
            title: "Info",
            is_editing: false,
            field_editing: Attribute::Front,
            cursor_location: 0,
            selected: None,
            last_selected: 0,
            highlight_style: Style::default().fg(Color::Black).bg(Color::White),
        }
    }

    // Select the next list item
    pub fn next(&mut self) {
        let selected = if let Some(selected) = self.selected {
            if selected >= self.cards.len() - 1 {
                Some(0)
            } else {
                Some(selected + 1)
            }
        } else {
            Some(0)
        };
        self.selected = selected;
        if let Some(s) = selected { self.last_selected = s }
    }

    // Select the previous list item
    pub fn prev(&mut self) {
        let selected = if let Some(selected) = self.selected {
            if selected > 0 {
                Some(selected - 1)
            } else {
                Some(self.cards.len() - 1)
            }
        } else {
            Some(0)
        };
        self.selected = selected;
        if let Some(s) = selected { self.last_selected = s }
    }

    // Begin editing one of the fields of the selected card
    pub fn enable_edit(&mut self, field_editing: Attribute) {
        self.is_editing = true;
        self.title = "Editing. Press <esc> when finished";
        match field_editing {
            Attribute::Front => {
                self.cursor_location = self.cards[self.last_selected].front.len();
                self.text = self.cards[self.last_selected].front.clone();
                self.field_editing = Attribute::Front;
            },
            Attribute::Back => {
                self.cursor_location = self.cards[self.last_selected].back.len();
                self.text = self.cards[self.last_selected].back.clone();
                self.field_editing = Attribute::Back;
            },
            Attribute::MainTag | Attribute::SubTag => {
                self.cursor_location = self.cards[self.last_selected].tags.len();
                self.text = self.cards[self.last_selected].tags.clone();
                self.field_editing = Attribute::MainTag;
            }
        }
    }

    // Stop editing the selected card
    pub fn disable_edit(mut self) -> App {
        self.is_editing = false;
        self.title = "Info";
        match self.field_editing {
            Attribute::Front => { self.cards[self.last_selected].front = self.text; },
            Attribute::Back => { self.cards[self.last_selected].back = self.text; },
            Attribute::MainTag => { self.cards[self.last_selected].tags = self.text; },
            _ => {}
        }

        self.text = self.default_text.clone();
        self
    }

    // Moveing the cursor while editing
    pub fn move_cursor_left(&mut self) {
        if self.cursor_location != 0 { self.cursor_location -= 1; }
    }
    pub fn move_cursor_right(&mut self) {
        if self.cursor_location != self.text.len() { self.cursor_location += 1; }
    }

    // Adding a character to the text being edited at the cursor location
    pub fn push(&mut self, c: char) {
        self.text.insert(self.cursor_location, c);
        self.cursor_location += 1;
    }

    // Removing a character from the text at the cursor location
    pub fn pop(&mut self) {
        if self.cursor_location > 0 {
            self.text.remove(self.cursor_location-1);
            self.cursor_location -= 1;
        }
    }

    // Returns an array of Text. The first element is the raw text before the cursor, the second
    // is the styled character at the cursor location, and the last is the raw text after the cursor.
    pub fn get_formatted_text(&self) -> [tui::widgets::Text;3] {
        let cursor_style = Style::default().fg(Color::Black).bg(Color::White);
        if self.text.is_empty() {
            return [Text::raw(""),Text::styled(" ", cursor_style), Text::raw("")]
        }
        if !self.is_editing {
            // If not editing, just return the text
            [Text::raw(&self.text),Text::raw(""), Text::raw("")]
        } else {
            // If cursor is at start of line:
            if self.cursor_location == 0 {
                [
                Text::styled(&self.text[..1], cursor_style),
                Text::raw(&self.text[1..]),
                Text::raw("")
                ]
            // If cursor is at end of line:
            } else if self.cursor_location >= self.text.len() {
                [
                Text::raw(&self.text),
                Text::styled(" ", cursor_style),
                Text::raw("")
                ]
            // If cursor is anywhere inbetween:
            } else {
                [
                Text::raw(&self.text[..self.cursor_location]),
                Text::styled(&self.text[self.cursor_location..=self.cursor_location], cursor_style),
                Text::raw(&self.text[self.cursor_location+1..])
                ]
            }
        }
    }
}
