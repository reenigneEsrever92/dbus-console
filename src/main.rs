use std::{error::Error, sync::Arc, time::Duration};

use async_std::{channel::*, prelude::StreamExt};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::future::{select, Either};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Terminal,
};
use zbus::{Connection, Message};

struct App {
    log: Vec<(Arc<Message>, bool)>,
    bus_names: Vec<String>,
    selected_bus: Option<i32>,
    exit: bool,
    focus: Focus,
}

enum Focus {
    BusFrame,
}

enum LogEntry {
    IncomingDbusMessage(Message),
    OutgoingDbusMessage(Message),
}

#[derive(Debug)]
enum AppEvent {
    None,
    Raw(Arc<Message>, bool),
    BusNames(Vec<String>),
    MethodCall,
    InvalidMessage,
    MethodResponse,
    DBusError,
    SelectLastBusName,
    SelectNextBusName,
}

enum Action {
    None,
    Initialize,
    Quit,
    LoadBusNames,
    SelectLastBusName,
    SelectNextBusName,
}

impl Default for App {
    fn default() -> Self {
        Self {
            log: Default::default(),
            bus_names: Default::default(),
            exit: Default::default(),
            focus: Focus::BusFrame,
            selected_bus: None,
        }
    }
}

async fn run_ui() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();

    enable_raw_mode().unwrap();

    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    disable_raw_mode().unwrap();

    let events = to_events(Action::Initialize).await;
    reduce(events, &mut app);

    loop {
        // draw ui -> action -> app event -> state -> redraw
        draw_ui(&app, &mut terminal).unwrap();
        let action = wait_for_user_input(&app);
        if let Action::Quit = action {
            break;
        }
        let events = to_events(action).await;
        reduce(events, &mut app);
    }

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    terminal.show_cursor().unwrap();

    Ok(())
}

fn wait_for_user_input(app: &App) -> Action {
    match crossterm::event::read() {
        Ok(Event::Key(key)) => match app.focus {
            Focus::BusFrame => match key.code {
                KeyCode::Enter => todo!(),
                KeyCode::Right => todo!(),
                KeyCode::Up => Action::SelectLastBusName,
                KeyCode::Down => Action::SelectNextBusName,
                _ => Action::None,
            },
        },
        Ok(Event::Resize(_, _)) => Action::None,
        _ => Action::None,
    }
}

fn reduce(events: Vec<AppEvent>, app: &mut App) {
    events
        .into_iter()
        .for_each(|event| reduce_event(event, app));
}

fn reduce_event(e: AppEvent, app: &mut App) {
    match e {
        AppEvent::Raw(_, _) => todo!(),
        AppEvent::BusNames(bus_names) => app.bus_names = bus_names,
        AppEvent::MethodCall => {}
        AppEvent::InvalidMessage => todo!(),
        AppEvent::MethodResponse => {}
        AppEvent::DBusError => todo!(),
        AppEvent::SelectNextBusName => {
            app.selected_bus = app.selected_bus.map_or(Some(0), |index| Some(index + 1))
        }
        AppEvent::SelectLastBusName => todo!(),
        AppEvent::None => {}
    }
}

async fn to_events(a: Action) -> Vec<AppEvent> {
    match a {
        Action::LoadBusNames => load_bus_names().await,
        Action::SelectLastBusName => vec![AppEvent::SelectLastBusName],
        Action::SelectNextBusName => vec![AppEvent::SelectNextBusName],
        Action::Quit => todo!(),
        Action::None => vec![],
        Action::Initialize => {
            let mut events = load_bus_names().await;
            events.append(&mut vec![AppEvent::SelectNextBusName]);
            events
        }
    }
}

async fn load_bus_names() -> Vec<AppEvent> {
    let connection = Connection::session().await.unwrap();

    let message = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "ListNames",
            &(),
        )
        .await
        .unwrap();

    vec![
        // TODO failing to deserialize body should not hinder the MethodReturn from propagating
        message_to_app_event(&message),
        AppEvent::BusNames(message.body().unwrap()),
    ]
}

fn message_to_app_event(message: &Arc<Message>) -> AppEvent {
    match message.message_type() {
        zbus::MessageType::Invalid => AppEvent::InvalidMessage,
        zbus::MessageType::MethodCall => AppEvent::MethodCall,
        zbus::MessageType::MethodReturn => AppEvent::MethodResponse,
        zbus::MessageType::Error => AppEvent::DBusError,
        zbus::MessageType::Signal => todo!(),
    }
}

fn draw_ui<B: Backend>(
    state: &App,
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(50)].as_ref())
            .split(f.size());

        f.render_widget(draw_bus_names(state), rects[0]);
    })?;

    Ok(())
}

fn draw_bus_names(state: &App) -> Table {
    let rows = state.bus_names.iter().map(|bus_name| {
        let cell = Cell::from(bus_name.as_str());
        Row::new([cell])
    });

    Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Bus Names"))
        .widths(&[Constraint::Percentage(100)])
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_ui().await.unwrap();

    Ok(())
}
