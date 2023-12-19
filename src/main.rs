use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};

fn main() -> io::Result<()> {
    App::run()
}

#[derive(Copy, Clone)]
enum Turn {
    Red,
    Blue,
}

#[derive(Copy, Clone)]
enum Field {
    Empty,
    Red,
    Blue,
}

impl Into<Field> for Turn {
    fn into(self) -> Field {
        match self {
            Turn::Red => Field::Red,
            Turn::Blue => Field::Blue,
        }
    }
}

struct App {
    board: [[Field; 7]; 6],
    turn: Turn,
    input: String,
}

impl App {
    fn new() -> App {
        App {
            board: [[Field::Empty; 7]; 6],
            turn: Turn::Red,
            input: String::new(),
        }
    }

    pub fn run() -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut app = App::new();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16);
        loop {
            let _ = terminal.draw(|frame| app.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(c) => {
                            if c.is_digit(10) {
                                app.input.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Enter => {
                            app.turn();
                        }
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        restore_terminal()
    }

    pub fn turn(&mut self) {
        let column: usize = match self.input.parse() {
            Ok(c) => c,
            Err(_) => {
                return;
            }
        };

        if column == 0 || column > self.board[0].len() {
            return;
        }

        let column = column - 1;

        let mut i: usize = 0;

        loop {
            if i >= self.board.len() {
                return;
            }
            match self.board[i][column] {
                Field::Empty => {
                    break;
                }
                _ => {
                    i += 1;
                }
            }
        }

        self.board[i][column] = self.turn.into();

        self.input = String::new();
        self.turn = match self.turn {
            Turn::Red => Turn::Blue,
            Turn::Blue => Turn::Red,
        };
    }

    fn ui(&self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
            .split(frame.size());

        let controls_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);

        frame.render_widget(self.board_canvas(), main_layout[0]);
        frame.render_widget(self.red_player_canvas(), controls_layout[0]);
        frame.render_widget(self.blue_player_canvas(), controls_layout[1]);
    }

    fn red_player_canvas(&self) -> impl Widget + '_ {
        let (color, text) = match self.turn {
            Turn::Red => (Color::Red, self.input.as_str()),
            Turn::Blue => (Color::White, ""),
        };
        Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Red player")
                    .border_style(Style::default().fg(color)),
            )
            .alignment(Alignment::Center)
    }

    fn blue_player_canvas(&self) -> impl Widget + '_ {
        let (color, text) = match self.turn {
            Turn::Blue => (Color::Blue, self.input.as_str()),
            Turn::Red => (Color::White, ""),
        };
        Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Blue player")
                    .border_style(Style::default().fg(color)),
            )
            .alignment(Alignment::Center)
    }

    fn board_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("4 in a row"))
            .paint(|ctx| {
                for (i, row) in self.board.iter().enumerate() {
                    for (j, field) in row.iter().enumerate() {
                        match field {
                            Field::Empty => {}
                            Field::Blue => {
                                ctx.draw(&Rectangle {
                                    x: (j as f64) * 50.0,
                                    y: (i as f64) * 50.0,
                                    width: 40.0,
                                    height: 40.0,
                                    color: Color::Blue,
                                });
                            }
                            Field::Red => {
                                ctx.draw(&Rectangle {
                                    x: (j as f64) * 50.0,
                                    y: (i as f64) * 50.0,
                                    width: 40.0,
                                    height: 40.0,
                                    color: Color::Red,
                                });
                            }
                        }
                    }
                }
            })
            .x_bounds([0.0, 350.0])
            .y_bounds([0.0, 300.0])
            .marker(Marker::HalfBlock)
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
