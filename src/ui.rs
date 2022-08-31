use crate::wilma;

use std::{io, time::Duration};
use tui::{
    backend::{CrosstermBackend, Backend },
    widgets::{TableState, Tabs, Wrap, Paragraph, Cell, Row, Table, Block, Borders},
    layout::{Rect, Layout, Constraint, Direction},
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
    fn go(&mut self) {
        if self.index < self.item_count { self.index += 1 }
        else { self.index = 0 }
    }
}

fn render_exams<B: Backend>(f: &mut Frame<B>, exams: &Vec<Vec<wilma::Schedule>>,
                               main_chunks: Vec<Rect>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(main_chunks[1]); }

fn render_schedule<B: Backend>(f: &mut Frame<B>, full_schedule: &Vec<Vec<wilma::Schedule>>,
                               main_chunks: Vec<Rect>) {


    let big_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ].as_ref()
        )
        .split(main_chunks[1]); 



    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    for i in 0..full_schedule.len() {
        let mut j = i;
        let mut k = 0;
        if i > 2 { j -= 3; k = 1; }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .split(big_chunks[j]); 


        let schedule = &full_schedule[i];
        let mut change_col = false;
        let today_schedule_table = Table::new(schedule.iter().map(|x|{
            let mut style = Style::default();
            if change_col { style = style.bg(Color::Black); }
            change_col = !change_col;
            Row::new(vec![&*x.name, &*x.room, &*x.time]).style(style)
        }))
        .header(
            Row::new(vec!["Name", "Room", "Time"])
                .style(Style::default().fg(Color::Blue))
        )
        .widths(&[Constraint::Percentage(33), Constraint::Percentage(33),
            Constraint::Percentage(33)])
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(days[i]),
        );


        f.render_widget(today_schedule_table, chunks[k]);
    }

}

fn render_overview<B: Backend>(f: &mut Frame<B>, today_schedule: &Vec<wilma::Schedule>,
                  litle_homework: &Vec<wilma::Homework>, homework_index: usize,
                  main_chunks: Vec<Rect>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ].as_ref()
        )
        .split(main_chunks[1]);

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
    let litle_homework_table = Table::new(litle_homework.iter().map(|x|{
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

    let mut content = "";
    if litle_homework.len() > 0 {
        content = &*litle_homework[homework_index].description;
    }
    let par = Paragraph::new(content)
        .block(Block::default().title("Homework Description").borders(Borders::ALL))
        .wrap(Wrap { trim: true });


    let mut homework_state = TableState::default();
    homework_state.select(Some(homework_index));



    f.render_widget(today_schedule_table, chunks[0]);
    f.render_stateful_widget(litle_homework_table, chunks[1], &mut homework_state);
    f.render_widget(par, chunks[2]);
}

fn ui<B: Backend>(f: &mut Frame<B>, today_schedule: &Vec<wilma::Schedule>,
                  litle_homework: &Vec<wilma::Homework>,
                  full_schedule: &Vec<Vec<wilma::Schedule>>,
                  homework_index: usize, tab_index: usize) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),

            ].as_ref()
        )
        .split(f.size()); 

    let titles = ["Overview", "Schedule", "Exams", "Notes", "Messages"].iter().cloned().map(Spans::from).collect();
    let tabs = Tabs::new(titles)
        .select(tab_index)
        .block(Block::default().title("Welcome to wilma-tui").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow));

    f.render_widget(tabs, main_chunks[0]);


    match tab_index {
        0 => render_overview(f, today_schedule, litle_homework, homework_index, main_chunks),
        1 => render_schedule(f, full_schedule, main_chunks),
        _ => {}
    };

}

pub fn run_ui(root: wilma::Root) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut homework_select = TableMove::new(root.homework.len());
    let mut tab_select = TableMove::new(5);

    loop {

        terminal.draw(|f| ui(f, &root.today_schedule, &root.homework, &root.full_schedule, homework_select.index, tab_select.index))?;
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => homework_select.down(),
                    KeyCode::Char('k') => homework_select.up(),
                    KeyCode::Tab => tab_select.go(),
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
