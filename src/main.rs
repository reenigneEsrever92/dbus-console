use ui::run_ui;

mod app;
mod dbus;
mod filter;
mod ui;
mod widgets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_ui()
}
