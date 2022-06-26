use ui::run_ui;

mod action;
mod app;
mod dbus;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_ui()
}
