use std::usize;

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
    pub selected: Option<u32>,
    pub skip: u32,
    pub visible_lines: u32,
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
    SelectNextBusName,
    SelectPreviousBusName,
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

pub fn action_to_events(a: Action) -> Vec<AppEvent> {
    match a {
        Action::LoadBusNames => vec![AppEvent::BusNamesLoaded(DBusClient::default().list_names())],
        Action::Quit => todo!(),
        Action::LoadPaths(service_name) => {
            vec![AppEvent::PathsLoaded(
                DBusClient::default().get_paths(&service_name),
            )]
        }
        Action::SelectLastBusName => vec![AppEvent::SelectPreviousBusName],
        Action::SelectNextBusName => vec![AppEvent::SelectNextBusName],
        Action::None => vec![AppEvent::None],
    }
}

// fn select_next_bus_name(_app: &App) -> AppEvent {
//     AppEvent::SelectNextBusName
// }

fn select_last_bus_name(_app: &App) -> AppEvent {
    AppEvent::SelectPreviousBusName
    // let mut bus_names = filter_bus_names(app).rev();

    // match app.bus_name_state.selected.as_ref() {
    //     Some(bus_name) => {
    //         // find bus name in buses
    //         match bus_names.position(|bus| &bus == &bus_name) {
    //             Some(_) => match bus_names.next() {
    //                 Some(name) => AppEvent::SelectedBusName(String::from(name)),
    //                 None => AppEvent::None,
    //             },
    //             None => AppEvent::None,
    //         }
    //     }
    //     None => match app.bus_name_state.entries.get(0) {
    //         Some(name) => AppEvent::SelectedBusName(String::from(name)),
    //         None => AppEvent::None,
    //     },
    // }
}

impl App {
    pub fn reduce(&mut self, action: Action) {
        action_to_events(action).into_iter().for_each(|event| {
            reduce_event(event, self);
        });
        // match reduce_event(action_to_events(action, self), self) {
        //     Some(event) => self.reduce(event),
        //     None => {}
        // }
    }
}

fn reduce_event(e: AppEvent, app: &mut App) {
    match e {
        AppEvent::BusNamesLoaded(bus_names) => {
            app.bus_name_state.entries = bus_names;
        }
        AppEvent::None => {}
        AppEvent::MethodsLoaded(methods) => {
            app.methods = methods;
        }
        AppEvent::PathsLoaded(paths) => {
            app.paths = paths;
        }
        AppEvent::SelectNextBusName => {
            select_next_bus_name(app);
            // todo!()
        }
        AppEvent::SelectPreviousBusName => todo!(),
    }
}

fn select_next_bus_name(app: &mut App) {
    let mut bus_names = filter_bus_names(app);
    app.bus_name_state.direction = Direction::DOWN;

    match app.bus_name_state.selected.as_ref() {
        Some(bus_name) => {
            // find bus name in buses
            match bus_names.position(|bus| &bus == &bus_name) {
                Some(_) => match bus_names.next() {
                    Some(name) => app.bus_name_state.selected = Some(name.to_owned()),
                    None => {}
                },
                None => {}
            }
        }
        None => match app.bus_name_state.entries.get(0) {
            Some(name) => app.bus_name_state.selected = Some(String::from(name)),
            None => {}
        },
    }
}
