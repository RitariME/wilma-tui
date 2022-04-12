use crate::wilma;

use std::{io, time::Duration};
use tui::{
    backend::{CrosstermBackend, Backend },
    widgets::{TableState, Tabs, Wrap, Paragraph, Cell, Row, Table, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
    style:: { Style,Color,Modifier},
    Frame,
    text::Spans
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct TableMove {
    index: usize,
    item_count: usize
}
impl TableMove {
    fn new(mut count: usize) -> TableMove {
        if count == 0 {
            count = 1;
        }
        TableMove { index: 0, item_count: count-1 }
    }
    fn down(&mut self) {
        if self.index < self.item_count { self.index += 1 }
    }
    fn up(&mut self) {
        if self.index > 0 { self.index -= 1 }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, today_schedule: &Vec<wilma::Schedule>,
                  little_homework: &Vec<wilma::Homework>, schedule_index: usize) {
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ].as_ref()
        )
        .split(f.size());
    let mut change_col = false;
    let today_schedule_table = Table::new(today_schedule.iter().map(|x|{
        let mut style = Style::default();
        if change_col { style = style.bg(Color::Black); }
        change_col = !change_col;
        Row::new(vec![&*x.name, &*x.teacher, &*x.room, &*x.time]).style(style)
    }))
    .header(
        Row::new(vec!["Name", "Teacher", "Room", "Time"])
            .style(Style::default().fg(Color::Blue))
    )
    .widths(&[Constraint::Percentage(25), Constraint::Percentage(25),
        Constraint::Percentage(25), Constraint::Percentage(25)])
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Today's Schedule"),
    );

    let mut change_col2 = false;
    let little_homework_table = Table::new(little_homework.iter().map(|x|{
        let mut style = Style::default();
        if change_col2 { style = style.bg(Color::Black); }
        change_col2 = !change_col2;
        Row::new(vec![&*x.name, &*x.teacher, &*x.description, &*x.date]).style(style)
    }))
    .header(
        Row::new(vec!["Name", "Teacher", "Description", "Date"])
            .style(Style::default().fg(Color::Blue))
    )
    .widths(&[Constraint::Percentage(25), Constraint::Percentage(25),
        Constraint::Percentage(25), Constraint::Percentage(25)])
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Homework"),
    );
    let par = Paragraph::new(&*little_homework[schedule_index].description)
        .block(Block::default().title("Homework Description").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    let titles = ["Overview", "Schedule", "Exams", "Homeworks", "Notes", "Messages"].iter().cloned().map(Spans::from).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().title("Welcome to wilma-tui").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow));


    let mut state = TableState::default();
    state.select(Some(schedule_index));

    f.render_widget(tabs, chunks[0]);
    f.render_widget(today_schedule_table, chunks[1]);
    f.render_stateful_widget(little_homework_table, chunks[2], &mut state);
    f.render_widget(par, chunks[3]);

}

pub fn run_ui(root: wilma::Root) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut homework_select = TableMove::new(root.homework.len());

    loop {
        terminal.draw(|f| ui(f, &root.today_schedule, &root.homework, homework_select.index))?;
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => homework_select.down(),
                    KeyCode::Char('k') => homework_select.up(),
                    //KeyCode::Tab => println!("down"),
                    _ => {}
                }
            }
        }
    }

    //thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
