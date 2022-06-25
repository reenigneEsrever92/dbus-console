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
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use zbus::{Connection, Message};

struct App {
    log: Vec<(Arc<Message>, bool)>,
    bus_names: Vec<String>,
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
    Raw(Arc<Message>, bool),
    BusNames(Vec<String>),
    MethodCall,
    InvalidMessage,
    MethodResponse,
    DBusError,
}

enum Action {
    None,
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
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_ui().await.unwrap();

    Ok(())
}

async fn run_ui() -> Result<(), Box<dyn Error>> {
    let mut app = Arc::new(App::default());

    enable_raw_mode().unwrap();

    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    disable_raw_mode().unwrap();

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
                KeyCode::Up => todo!(),
                KeyCode::Down => todo!(),
                _ => Action::None,
            },
        },
        _ => Action::None,
    }
}

fn reduce(events: Vec<AppEvent>, app: &mut Arc<App>) {}

fn reduce_event(e: AppEvent, app: App) -> App {
    match e {
        AppEvent::Raw(_, _) => todo!(),
        AppEvent::BusNames(_) => todo!(),
        AppEvent::MethodCall => todo!(),
        AppEvent::InvalidMessage => todo!(),
        AppEvent::MethodResponse => todo!(),
        AppEvent::DBusError => todo!(),
    }
}

async fn to_events(a: Action) -> Vec<AppEvent> {
    match a {
        Action::LoadBusNames => load_bus_names().await,
        Action::SelectLastBusName => todo!(),
        Action::SelectNextBusName => todo!(),
        Action::Quit => todo!(),
        Action::None => vec![],
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

    vec![ // TODO failing to deserialize body should not hinder the MethodReturn from propagating
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

fn key_event_to_action(app: &App, input_event: Option<Result<Event, std::io::Error>>) -> Action {
    match input_event {
        Some(Ok(event)) => match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Esc => Action::Quit,
                _ => {
                    todo!()
                }
            },
            Event::Mouse(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        },
        Some(Err(_e)) => todo!(),
        None => todo!(),
    }
}

fn draw_ui<B: Backend>(
    state: &App,
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Paragraph::new(
            state
                .log
                .iter()
                .map(|msg| {
                    if msg.1 {
                        format!("> {:?}", msg.0)
                    } else {
                        format!("< {:?}", msg.0)
                    }
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .block(Block::default().title("Logs").borders(Borders::ALL));
        f.render_widget(block, size);
    })?;

    Ok(())
}
