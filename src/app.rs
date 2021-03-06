use std::usize;

use crate::{dbus::DBusClient, error::DBusConsoleError, filter::filter_bus_names};

pub struct App {
    pub bus_name_state: ListState<String>,
    pub paths: Vec<String>,
    pub methods: Vec<String>,
    pub filter_aliases: bool,
    pub focus: Section,
    pub log: ListState<LogEntry>,
}

#[derive(Debug)]
pub enum LogEntry {
    ActionEntry(Action),
    AppEventEntry(AppEvent),
}

pub struct ListState<T> {
    pub entries: Vec<T>,
    pub selected: Option<u32>,
    pub skip: u32,
    pub visible_lines: u32,
}

impl<T> Default for ListState<T> {
    fn default() -> Self {
        Self {
            entries: Default::default(),
            selected: Default::default(),
            skip: Default::default(),
            visible_lines: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Section {
    BusFrame,
    BusPath,
}

#[derive(Debug, Clone)]
pub enum Action {
    None,
    Quit,
    FocusBusNames,
    FocusPaths,
    LoadBusNames,
    LoadPaths { bus_name: String },
    Resize { section: Section, rows: i32 },
    SelectLastBusName,
    SelectNextBusName,
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    None,
    Error(DBusConsoleError),
    BusNamesLoaded(Vec<String>),
    PathsLoaded(Vec<String>),
    MethodsLoaded(Vec<String>),
    SelectNextBusName,
    SelectPreviousBusName,
    FocusBusNames,
    FocusPaths,
}

impl Default for App {
    fn default() -> Self {
        Self {
            focus: Section::BusFrame,
            methods: Default::default(),
            paths: Default::default(),
            filter_aliases: true,
            bus_name_state: ListState::default(),
            log: ListState::default(),
        }
    }
}

impl App {
    pub fn reduce(&mut self, action: Action) {
        // action_to_events(action).into_iter().for_each(|event| {
        //     reduce_event(event, self);
        // });
        // match action {
        //     Action::None => {}
        //     a => self.reduce(reduce_event(self, action_to_events(a))),
        // }
        match reduce_event(self, action_to_events(action)) {
            Action::None => {}
            a => {
                self.log.entries.push(LogEntry::ActionEntry(a.to_owned()));
                self.reduce(a)
            }
        }
    }
}

pub fn action_to_events(a: Action) -> AppEvent {
    match a {
        Action::LoadBusNames => AppEvent::BusNamesLoaded(DBusClient::default().list_names()),
        Action::Quit => todo!(),
        Action::LoadPaths { bus_name } => match DBusClient::default().get_paths(&bus_name) {
            Ok(paths) => AppEvent::PathsLoaded(paths),
            Err(e) => AppEvent::Error(e),
        },
        Action::SelectLastBusName => AppEvent::SelectPreviousBusName,
        Action::SelectNextBusName => AppEvent::SelectNextBusName,
        Action::None => AppEvent::None,
        Action::Resize { section, rows } => todo!(),
        Action::FocusBusNames => AppEvent::FocusBusNames,
        Action::FocusPaths => AppEvent::FocusPaths,
    }
}

fn reduce_event(app: &mut App, e: AppEvent) -> Action {
    app.log.entries.push(LogEntry::AppEventEntry(e.to_owned()));
    match e {
        AppEvent::BusNamesLoaded(bus_names) => {
            app.bus_name_state.entries = bus_names;
            Action::None
        }
        AppEvent::None => Action::None,
        AppEvent::MethodsLoaded(methods) => {
            app.methods = methods;
            Action::None
        }
        AppEvent::PathsLoaded(paths) => {
            app.paths = paths;
            Action::None
        }
        AppEvent::SelectNextBusName => select_next_bus_name(app),
        AppEvent::SelectPreviousBusName => select_last_bus_name(app),
        AppEvent::Error(_) => Action::None,
        AppEvent::FocusBusNames => {
            app.focus = Section::BusFrame;
            Action::None
        },
        AppEvent::FocusPaths => {
            app.focus = Section::BusPath;
            Action::None
        },
    }
}

fn select_next_bus_name(app: &mut App) -> Action {
    match app.bus_name_state.selected.as_ref() {
        Some(index) => select_bus_name(app, *index as i32 + 1),
        None => select_bus_name(app, 0),
    }
}

fn select_last_bus_name(app: &mut App) -> Action {
    match app.bus_name_state.selected.as_ref() {
        Some(index) => select_bus_name(app, *index as i32 - 1),
        None => select_bus_name(app, 0),
    }
}

fn select_bus_name(app: &mut App, index: i32) -> Action {
    let bus_names: Vec<&String> = filter_bus_names(app).collect();
    if index < i32::try_from(bus_names.len()).unwrap() && index >= 0 {
        app.bus_name_state.selected = Some(index as u32);
    }
    match app.bus_name_state.selected {
        Some(index) => Action::LoadPaths {
            bus_name: app
                .bus_name_state
                .entries
                .get(index as usize)
                .unwrap()
                .to_owned(),
        },
        None => Action::None,
    }
}
