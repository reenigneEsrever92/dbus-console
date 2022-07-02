use std::error::Error;

use crate::{
    action::Action,
    app::{action_to_events, App, Focus},
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{
        Constraint,
        Direction::{Horizontal, Vertical},
        Layout,
    },
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};

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
        let root_layout = Layout::default()
            .direction(Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        let left_pane = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(root_layout[0]);

        let right_pane = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(root_layout[1]);

        f.render_widget(draw_bus_names(state), left_pane[0]);
        f.render_widget(draw_bus_paths(state), left_pane[1]);
        f.render_widget(draw_methods(state), right_pane[0]);
    })?;

    Ok(())
}

fn draw_methods(state: &App) -> Table {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let rows = state.methods.iter().map(|method| {
        let cell = Cell::from(method.as_str());
        let row = Row::new([cell]);

        row
    });

    Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Methods"))
        .highlight_style(selected_style)
        .widths(&[Constraint::Percentage(100)])
}

fn draw_bus_paths(state: &App) -> Table {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let rows = state.paths.iter().map(|bus_name| {
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
