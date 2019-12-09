use termion::raw::RawTerminal;
use std::io;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Alignment};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, SelectableList, Widget, Paragraph, Text};
use tui::Terminal;
use crate::app::App;

pub struct Ui {
    terminal: tui::terminal::Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<std::io::Stdout>>>>>,
}
impl Ui {

    pub fn new() -> Result<Ui, io::Error> {
        // Initialising terminal:
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(Ui { terminal })
    }

    // Render the UI
    pub fn render(&mut self, app: &App) -> Result<(), io::Error> {

        self.terminal.draw(|mut f| {

            let size = f.size();

            //  Drawing frame
            Block::default().borders(Borders::ALL).render(&mut f, size);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(f.size());

            // heading
            Paragraph::new(app.get_formatted_text().iter())
            .block(Block::default().title(app.title).borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut f, chunks[0]);

            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[1]);

                // List of cards
                let items: Vec<String> = app.cards.iter().map(|c| c.front.clone()).collect();
                SelectableList::default()
                    .block(Block::default().borders(Borders::ALL).title("Cards"))
                    .select(app.selected)
                    .items(&items[..])
                    .highlight_style(app.highlight_style)
                    .highlight_symbol(">>")
                    .render(&mut f, chunks[0]);

                {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints([Constraint::Percentage(40), Constraint::Percentage(40), Constraint::Percentage(20)].as_ref())
                        .split(chunks[1]);
                    let block = Block::default()
                        .title_style(Style::default().modifier(Modifier::BOLD))
                        .borders(Borders::ALL);
                    let card_index = match app.selected {
                        Some(n) => { n },
                        None => { app.last_selected }
                    };

                    // Card front preview
                    Paragraph::new([Text::raw(app.cards[card_index].front.clone())].iter())
                        .block(block.clone().title("Front"))
                        .style(Style::default().fg(Color::Black).bg(Color::White))
                        .alignment(Alignment::Center)
                        .wrap(true)
                        .render(&mut f, chunks[0]);

                    // Card back preview
                    Paragraph::new([Text::raw(app.cards[card_index].back.clone().replace("<br>", "\n"))].iter())
                        .block(block.clone().title("Back"))
                        .style(Style::default().fg(Color::Black).bg(Color::White))
                        .alignment(Alignment::Center)
                        .wrap(true)
                        .render(&mut f, chunks[1]);

                    // Card tags preview
                    Paragraph::new([Text::raw(app.cards[card_index].tags.clone())].iter())
                        .block(block.clone().title("Tags"))
                        .style(Style::default().fg(Color::Black).bg(Color::White))
                        .alignment(Alignment::Center)
                        .wrap(true)
                        .render(&mut f, chunks[2]);
                }
            }
        })?;

        Ok(())
    }
}
