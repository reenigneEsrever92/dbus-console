use std::{error::Error, sync::Arc};

use action::Action;
use app::{action_to_events, AppEvent};

use ui::run_ui;
use zbus::Message;

mod action;
mod app;
mod dbus;
mod ui;


enum LogEntry {
    IncomingDbusMessage(Message),
    OutgoingDbusMessage(Message),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_ui().unwrap();

    Ok(())
}
