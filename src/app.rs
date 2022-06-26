use crate::{action::Action, dbus::load_bus_names};

pub struct App {
    pub bus_names: Vec<String>,
    pub selected_bus: Option<String>,
    pub focus: Focus,
}

pub enum Focus {
    BusFrame,
}

#[derive(Debug)]
pub enum AppEvent {
    None,
    BusNames(Vec<String>),
    MethodCall,
    InvalidMessage,
    MethodResponse,
    DBusError,
    SelectLastBusName,
    SelectNextBusName,
}

impl Default for App {
    fn default() -> Self {
        Self {
            bus_names: Default::default(),
            focus: Focus::BusFrame,
            selected_bus: None,
        }
    }
}

pub fn action_to_events(a: Action) -> Vec<AppEvent> {
    match a {
        Action::LoadBusNames => load_bus_names(),
        Action::SelectLastBusName => vec![AppEvent::SelectLastBusName],
        Action::SelectNextBusName => vec![AppEvent::SelectNextBusName],
        Action::Quit => todo!(),
        Action::None => vec![],
        Action::Initialize => {
            let mut events: Vec<AppEvent> = load_bus_names();
            events.append(&mut vec![AppEvent::SelectNextBusName]);
            events
        }
    }
}

impl App {
    pub fn reduce(&mut self, events: Vec<AppEvent>) {
        events
            .into_iter()
            .for_each(|event| reduce_event(event, self));
    }
}

fn reduce_event(e: AppEvent, app: &mut App) {
    match e {
        AppEvent::BusNames(bus_names) => app.bus_names = bus_names,
        AppEvent::MethodCall => {}
        AppEvent::InvalidMessage => todo!(),
        AppEvent::MethodResponse => {}
        AppEvent::DBusError => todo!(),
        AppEvent::SelectNextBusName => select_next_bus_name(app),
        AppEvent::SelectLastBusName => select_last_bus_name(app),
        AppEvent::None => {}
    }
}

fn select_next_bus_name(app: &mut App) {
    let selected_index = app.selected_bus.as_ref().map_or(0, |selected_bus| {
        app.bus_names
            .iter()
            .position(|bus_name| bus_name == selected_bus)
            .unwrap_or(0)
    });

    app.selected_bus = app.bus_names.get(selected_index + 1).cloned()
}

fn select_last_bus_name(app: &mut App) {
    let selected_index = app.selected_bus.as_ref().map_or(0, |selected_bus| {
        app.bus_names
            .iter()
            .position(|bus_name| bus_name == selected_bus)
            .unwrap_or(0)
    });

    app.selected_bus = app.bus_names.get(selected_index - 1).cloned()
}
