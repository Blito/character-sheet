use std::io;
use termion::raw::IntoRawMode;
use tui::{Frame, Terminal};
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Layout, Constraint, Direction, Rect, Alignment};
use tui::widgets::{Widget, Block, Borders, Text, Paragraph};
use tui::style::{Color, Modifier, Style};

fn draw_main_layout<B>(f: &mut Frame<B>)
where
    B: Backend 
{
    let layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());

    draw_player_header(f, layout_chunks[0]);
    Block::default()
        .title("Block 2")
        .borders(Borders::ALL)
        .render(f, layout_chunks[2]);
}

fn draw_player_header<B>(f: &mut Frame<B>, layout_chunk: Rect) 
where
    B: Backend 
{
    Block::default()
        .title("Block")
        .borders(Borders::ALL)
        .render(f, layout_chunk);

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(10), // Picture
                Constraint::Percentage(30), // Name
                Constraint::Percentage(40), // Free Space
                Constraint::Percentage(30)  // Short / Long Rests
            ].as_ref()
        )
        .split(layout_chunk);

    let text = [
        Text::styled("\nDandelion\n", Style::default().modifier(Modifier::BOLD)),
        Text::styled("Rock Gnome Wizard Lvl 3\n", Style::default())
    ];
    Paragraph::new(text.iter())
        .wrap(true)
        .render(f, inner_layout[1]);

    draw_rests(f, inner_layout[3])
}

fn draw_rests<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50), // Short Rest
                Constraint::Percentage(50), // Long Rest
            ].as_ref()
        )
        .split(layout_chunk);

    draw_short_rest(f, inner_layout[0]);
    draw_long_rest(f, inner_layout[1]);
}

fn draw_short_rest<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .borders(Borders::ALL).render(f, layout_chunk);

    let inner_short_rest_layout = Layout::default()
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(100), // Short Rest
            ].as_ref()
        )
        .split(layout_chunk);

    let short_rest = [
        Text::styled("S", Style::default().modifier(Modifier::UNDERLINED)),
        Text::styled("hort rest ⛺", Style::default()),
    ];
    Paragraph::new(short_rest.iter())
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, inner_short_rest_layout[0]);
}

fn draw_long_rest<B>(f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
{
    Block::default()
        .borders(Borders::ALL).render(f, layout_chunk);

    let inner_long_rest_layout = Layout::default()
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(100), // Short Rest
            ].as_ref()
        )
        .split(layout_chunk);

    let long_rest = [
        Text::styled("L", Style::default().modifier(Modifier::UNDERLINED)),
        Text::styled("ong rest 🌖", Style::default()),
    ];
    Paragraph::new(long_rest.iter())
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, inner_long_rest_layout[0]);
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear();

    terminal.draw(|mut f| {
        draw_main_layout(&mut f);
    })

}