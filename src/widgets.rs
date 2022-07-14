use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Text,
    widgets::{Block, Widget},
};

#[derive(Debug, Clone)]
struct ConsoleListEntry<'a> {
    selected: bool,
    style: Style,
    content: Text<'a>,
}

impl<'a> ConsoleListEntry<'a> {
    pub fn new<T>(content: T) -> ConsoleListEntry<'a>
    where
        T: Into<Text<'a>>,
    {
        ConsoleListEntry {
            selected: false,
            style: Style::default(),
            content: content.into(),
        }
    }
}

struct ConsoleList<'a> {
    block: Option<Block<'a>>,
    rows: Vec<ConsoleListEntry<'a>>,
    selected_style: Style,
    skip: Option<i32>,
}

impl<'a> ConsoleList<'a> {
    fn do_render(mut self, area: Rect, buf: &mut Buffer) {
        let skip = self.calc_skip();

        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }
    }

    fn calc_skip(&self) -> i32 {
        todo!()
    }
}

impl<'a> Widget for ConsoleList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.do_render(area, buf)
    }
}
