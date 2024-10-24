//! Widgets



use dreg::prelude::*;



pub struct Corners {
    pub style: Style,
}

impl Corners {
    pub fn new() -> Self {
        Self {
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }
        buf.set_stringn(area.left(), area.top(), "╭", 1, self.style);
        buf.set_stringn(area.right().saturating_sub(1), area.top(), "╮", 1, self.style);
        buf.set_stringn(area.left(), area.bottom().saturating_sub(1), "╰", 1, self.style);
        buf.set_stringn(area.right().saturating_sub(1), area.bottom().saturating_sub(1), "╯", 1, self.style);
    }
}
