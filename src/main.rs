use ui::run_ui;

mod app;
mod dbus;
mod error;
mod filter;
mod tokenizer;
mod ui;
mod widgets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_ui()
}
