use regex::Regex;

use crate::app::App;

pub fn filter_bus_names<'a>(app: &'a App) -> impl DoubleEndedIterator<Item = &String> {
    let regex = Regex::new(r":\d.\d").unwrap();

    app.bus_names
        .iter()
        .filter(move |bus_name| app.filter_aliases && !regex.is_match(bus_name))
}
