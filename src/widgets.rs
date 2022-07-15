use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Span, Text},
    widgets::{Block, Widget},
};

#[derive(Debug, Clone)]
pub struct ConsoleListItem<'a> {
    selected: bool,
    style: Style,
    content: Span<'a>,
}

impl<'a> ConsoleListItem<'a> {
    pub fn new<T>(content: T) -> ConsoleListItem<'a>
    where
        T: Into<Span<'a>>,
    {
        ConsoleListItem {
            selected: false,
            style: Style::default(),
            content: content.into(),
        }
    }
}

pub struct ConsoleList<'a> {
    block: Option<Block<'a>>,
    items: Vec<ConsoleListItem<'a>>,
    selected_style: Option<Style>,
}

impl<'a> ConsoleList<'a> {
    pub fn new(items: Vec<ConsoleListItem<'a>>) -> Self {
        Self {
            block: None,
            items,
            selected_style: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    fn do_render(mut self, area: Rect, buf: &mut Buffer) {
        // let skip = self.calc_skip();

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

        self.items.iter().enumerate().for_each(|(pos, item)| {
            if (list_area.y + pos as u16) < (list_area.y + list_area.height) {
                buf.set_span(
                    list_area.x,
                    list_area.y + pos as u16,
                    &item.content,
                    list_area.width,
                );
            }
        });
        // buf.set_span(
        //     list_area.x,
        //     list_area.y,
        //     &Span::raw("test"),
        //     list_area.width,
        // );
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
