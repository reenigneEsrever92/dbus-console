use crate::{dbus::DBusClient, filter::filter_bus_names};

pub struct App {
    pub bus_name_state: ListState,
    pub paths: Vec<String>,
    pub methods: Vec<String>,
    pub filter_aliases: bool,
    pub focus: Focus,
}

#[derive(Default)]
pub struct ListState {
    pub entries: Vec<String>,
    pub direction: Direction,
    pub selected: Option<String>,
}

pub enum Direction {
    UP,
    DOWN,
}

impl Default for Direction {
    fn default() -> Self {
        Self::DOWN
    }
}

pub enum Focus {
    BusFrame,
}

pub enum Action {
    None,
    Quit,
    LoadBusNames,
    LoadPaths(String),
    SelectLastBusName,
    SelectNextBusName,
}

#[derive(Debug)]
pub enum AppEvent {
    None,
    BusNamesLoaded(Vec<String>),
    PathsLoaded(Vec<String>),
    MethodsLoaded(Vec<String>),
    SelectedBusName(String),
}

impl Default for App {
    fn default() -> Self {
        Self {
            focus: Focus::BusFrame,
            methods: Default::default(),
            paths: Default::default(),
            filter_aliases: true,
            bus_name_state: ListState::default(),
        }
    }
}

pub fn action_to_events(a: Action, app: &App) -> AppEvent {
    match a {
        Action::LoadBusNames => AppEvent::BusNamesLoaded(DBusClient::default().list_names()),
        Action::Quit => todo!(),
        Action::LoadPaths(service_name) => {
            AppEvent::PathsLoaded(DBusClient::default().get_paths(&service_name))
        }
        Action::SelectLastBusName => select_last_bus_name(app),
        Action::SelectNextBusName => select_next_bus_name(app),
        Action::None => AppEvent::None,
    }
}

fn select_next_bus_name(app: &App) -> AppEvent {
    let mut bus_names = filter_bus_names(app);

    match app.bus_name_state.selected.as_ref() {
        Some(bus_name) => {
            // find bus name in buses
            match bus_names.position(|bus| &bus == &bus_name) {
                Some(_) => match bus_names.next() {
                    Some(name) => AppEvent::SelectedBusName(String::from(name)),
                    None => AppEvent::None,
                },
                None => AppEvent::None,
            }
        }
        None => match app.bus_name_state.entries.get(0) {
            Some(name) => AppEvent::SelectedBusName(String::from(name)),
            None => AppEvent::None,
        },
    }
}

fn select_last_bus_name(app: &App) -> AppEvent {
    let mut bus_names = filter_bus_names(app).rev();

    match app.bus_name_state.selected.as_ref() {
        Some(bus_name) => {
            // find bus name in buses
            match bus_names.position(|bus| &bus == &bus_name) {
                Some(_) => match bus_names.next() {
                    Some(name) => AppEvent::SelectedBusName(String::from(name)),
                    None => AppEvent::None,
                },
                None => AppEvent::None,
            }
        }
        None => match app.bus_name_state.entries.get(0) {
            Some(name) => AppEvent::SelectedBusName(String::from(name)),
            None => AppEvent::None,
        },
    }
}

impl App {
    pub fn reduce(&mut self, action: Action) {
        match reduce_event(action_to_events(action, self), self) {
            Some(event) => self.reduce(event),
            None => {}
        }
    }
}

fn reduce_event(e: AppEvent, app: &mut App) -> Option<Action> {
    match e {
        AppEvent::BusNamesLoaded(bus_names) => {
            app.bus_name_state.entries = bus_names;
            None
        }
        AppEvent::None => None,
        AppEvent::MethodsLoaded(methods) => {
            app.methods = methods;
            None
        }
        AppEvent::PathsLoaded(paths) => {
            app.paths = paths;
            None
        }
        AppEvent::SelectedBusName(bus_name) => {
            app.bus_name_state.selected = Some(bus_name.clone());
            Some(Action::LoadPaths(bus_name))
        }
    }
}
