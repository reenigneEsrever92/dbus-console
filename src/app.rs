use crate::dbus::DBusClient;

pub struct App {
    pub bus_names: Vec<String>,
    pub paths: Vec<String>,
    pub methods: Vec<String>,
    pub selected_bus: Option<String>,
    pub filter_aliases: bool,
    pub focus: Focus,
}

pub enum Focus {
    BusFrame,
}

pub enum Action {
    None,
    Initialize,
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
            bus_names: Default::default(),
            focus: Focus::BusFrame,
            selected_bus: None,
            methods: Default::default(),
            paths: Default::default(),
            filter_aliases: true,
        }
    }
}

pub fn action_to_events(a: Action, app: &App) -> AppEvent {
    match a {
        Action::LoadBusNames => AppEvent::BusNamesLoaded(DBusClient::default().list_names()),
        Action::Quit => todo!(),
        Action::Initialize => action_to_events(Action::LoadBusNames, app),
        Action::LoadPaths(service_name) => {
            AppEvent::PathsLoaded(DBusClient::default().get_paths(&service_name))
        }
        Action::SelectLastBusName => select_last_bus_name(app),
        Action::SelectNextBusName => select_next_bus_name(app),
        Action::None => AppEvent::None,
    }
}

fn select_next_bus_name(app: &App) -> AppEvent {
    match app.selected_bus.as_ref() {
        Some(bus_name) => {
            // find bus name in buses
            match app.bus_names.iter().position(|bus| &bus == &bus_name) {
                Some(index) => {
                    if index < app.bus_names.len() - 1 {
                        match app.bus_names.get(index + 1) {
                            Some(name) => AppEvent::SelectedBusName(String::from(name)),
                            None => AppEvent::None,
                        }
                    } else {
                        AppEvent::None
                    }
                }
                None => AppEvent::None,
            }
        }
        None => match app.bus_names.get(0) {
            Some(name) => AppEvent::SelectedBusName(String::from(name)),
            None => AppEvent::None,
        },
    }
}

fn select_last_bus_name(app: &App) -> AppEvent {
    match app.selected_bus.as_ref() {
        Some(bus_name) => {
            // find bus name in buses
            match app.bus_names.iter().position(|bus| &bus == &bus_name) {
                Some(index) => {
                    if index > 0 {
                        match app.bus_names.get(index - 1) {
                            Some(name) => AppEvent::SelectedBusName(String::from(name)),
                            None => AppEvent::None,
                        }
                    } else {
                        AppEvent::None
                    }
                }
                None => AppEvent::None,
            }
        }
        None => match app.bus_names.get(0) {
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
            app.bus_names = bus_names;
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
            app.selected_bus = Some(bus_name.clone());
            Some(Action::LoadPaths(bus_name))
        }
    }
}
