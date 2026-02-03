use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame};

use crate::cube::{Cube, Rotate};

pub fn main() -> std::io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

struct App {}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut visible_cube = Cube::solved();
    let mut moves = vec![
        Rotate::Rp,
        Rotate::L,
        Rotate::Fp,
        Rotate::Rp,
        Rotate::L,
        Rotate::Dp,
        Rotate::Rp,
        Rotate::L,
        Rotate::B,
        Rotate::B,
        // second half
        Rotate::R,
        Rotate::Lp,
        Rotate::Dp,
        Rotate::R,
        Rotate::Lp,
        Rotate::Fp,
        Rotate::R,
        Rotate::Lp,
        Rotate::U,
        Rotate::U,
    ];
    moves.reverse();

    loop {
        terminal.draw(|f| render(f, &visible_cube))?;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('n') => {
                    if let Some(a) = moves.pop() {
                        visible_cube.rotate(a);
                    }
                }
                KeyCode::Char('u') => visible_cube.rotate(Rotate::U),
                KeyCode::Char('d') => visible_cube.rotate(Rotate::D),
                KeyCode::Char('r') => visible_cube.rotate(Rotate::R),
                KeyCode::Char('l') => visible_cube.rotate(Rotate::L),
                KeyCode::Char('f') => visible_cube.rotate(Rotate::F),
                KeyCode::Char('b') => visible_cube.rotate(Rotate::B),
                KeyCode::Char('U') => visible_cube.rotate(Rotate::Up),
                KeyCode::Char('D') => visible_cube.rotate(Rotate::Dp),
                KeyCode::Char('R') => visible_cube.rotate(Rotate::Rp),
                KeyCode::Char('L') => visible_cube.rotate(Rotate::Lp),
                KeyCode::Char('F') => visible_cube.rotate(Rotate::Fp),
                KeyCode::Char('B') => visible_cube.rotate(Rotate::Bp),
                // handle other key events
                _ => {}
            },
            // handle other events
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, cube: &Cube) {
    let s = cube
        .to_string()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'W' => Span::styled("#", Style::new().fg(Color::Rgb(250, 250, 250))),
                    'O' => Span::styled("#", Style::new().fg(Color::Rgb(200, 120, 0))),
                    'G' => Span::styled("#", Style::new().fg(Color::Rgb(0, 200, 0))),
                    'Y' => Span::styled("#", Style::new().fg(Color::Rgb(240, 200, 0))),
                    'R' => Span::styled("#", Style::new().fg(Color::Rgb(250, 0, 0))),
                    'B' => Span::styled("#", Style::new().fg(Color::Rgb(0, 0, 250))),
                    '+' => Span::raw("+"),
                    '-' => Span::raw("-"),
                    '|' => Span::raw("|"),
                    ' ' | _ => Span::raw(" "),
                })
                .collect::<Vec<Span>>()
        })
        .map(|l| Line::from(l))
        .collect::<Vec<Line>>();
    frame.render_widget(Text::from(s), frame.area());
}
