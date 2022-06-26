use std::error::Error;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};
use crate::{action::Action, app::{App, Focus, action_to_events}};


pub fn run_ui() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::default();
    let events = action_to_events(Action::Initialize);
    app.reduce(events);

    loop {
        // draw ui -> action -> app event -> state -> redraw
        draw_ui(&app, &mut terminal).unwrap();
        let action = wait_for_user_input(&app);
        if let Action::Quit = action {
            break;
        }
        let events = action_to_events(action);
        app.reduce(events);
    }

    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    terminal.show_cursor().unwrap();

    Ok(())
}

fn draw_ui<B: Backend>(
    state: &App,
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        f.render_widget(draw_bus_names(state), rects[0]);
        f.render_widget(draw_bus_paths(state), rects[1]);
    })?;

    Ok(())
}

fn draw_bus_paths(state: &App) -> Table {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let rows = state.bus_names.iter().map(|bus_name| {
        let cell = Cell::from(bus_name.as_str());
        let row = Row::new([cell]);

        if let Some(selected_bus) = &state.selected_bus {
            if selected_bus == bus_name {
                row.style(selected_style)
            } else {
                row
            }
        } else {
            row
        }
    });

    Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Paths"))
        .highlight_style(selected_style)
        .widths(&[Constraint::Percentage(100)])
}

fn draw_bus_names(state: &App) -> Table {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let rows = state.bus_names.iter().map(|bus_name| {
        let cell = Cell::from(bus_name.as_str());
        let row = Row::new([cell]);

        if let Some(selected_bus) = &state.selected_bus {
            if selected_bus == bus_name {
                row.style(selected_style)
            } else {
                row
            }
        } else {
            row
        }
    });

    Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Bus Names"))
        .highlight_style(selected_style)
        .widths(&[Constraint::Percentage(100)])
}

fn wait_for_user_input(app: &App) -> Action {
    match crossterm::event::read() {
        Ok(Event::Key(key)) => match app.focus {
            Focus::BusFrame => match key.code {
                KeyCode::Enter => todo!(),
                KeyCode::Right => todo!(),
                KeyCode::Up => Action::SelectLastBusName,
                KeyCode::Down => Action::SelectNextBusName,
                KeyCode::Char('q') => Action::Quit,
                _ => Action::None,
            },
        },
        Ok(Event::Resize(_, _)) => Action::None,
        _ => Action::None,
    }
}