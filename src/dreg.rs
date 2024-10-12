//! Dummy Dreg Implementation



use std::borrow::Cow;

use crate::*;



pub fn route(req: egui_router::Request<State>) -> impl egui_router::Route<State> {
    let slug = req.params.get("slug").unwrap_or_default();
    req.state.slug = slug.to_string();

    move |ui: &mut egui::Ui, state: &mut State| {
        render(ui, state);
    }
}



fn render(ui: &mut egui::Ui, state: &mut State) {
    let font_id = egui::FontId::monospace(state.font_size);
    let cell_width = ui.ctx().fonts(|f| f.glyph_width(&font_id, ' '));

    let rows = (ui.available_height() / state.font_size).floor() as u16;
    let cols = (ui.available_width() / cell_width).floor() as u16;

    let mut buf = Buffer::new(rows, cols);
    let main_rect = Rect::new(0, 0, cols, rows);

    // RENDER TEST
    {
        Label::new("This is a test...")
            .render(main_rect, &mut buf);
    }

    let painter = ui.painter();
    for x in 0..cols {
        for y in 0..rows {
            let i = buf.index_of(x, y);
            let c = buf.get(i);
            if c == &' ' { continue; }; // Minor performance optimization.
            let pos = egui::pos2(x as f32 * cell_width, y as f32 * state.font_size);
            painter.text(
                pos,
                egui::Align2::LEFT_TOP,
                c,
                font_id.clone(),
                egui::Color32::from_gray(91),
            );
        }
    }
}



struct Buffer {
    cells: Vec<char>,
    rows: u16,
    cols: u16,
}

impl Buffer {
    fn new(rows: u16, cols: u16) -> Self {
        let size = (rows * cols) as usize;
        Self {
            cells: vec![' '; size],
            rows,
            cols,
        }
    }

    fn index_of(&self, x: u16, y: u16) -> usize {
        (y * self.cols + x) as usize
    }

    fn get(&self, i: usize) -> &char {
        &self.cells[i]
    }

    fn set(&mut self, i: usize, c: char) {
        self.cells[i] = c;
    }
}



#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Rect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Rect {
    pub const ZERO: Self = Self {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };

    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height, }
    }

    pub fn with_pos(self, x: u16, y: u16) -> Self {
        Self {
            x,
            y,
            ..self
        }
    }

    pub fn inner_centered(&self, width: u16, height: u16) -> Self {
        let x = self.x + (self.width.saturating_sub(width) / 2);
        let y = self.y + (self.height.saturating_sub(height) / 2);
        Rect::new(x, y, width.min(self.width), height.min(self.height))
    }

    pub fn hsplit_len(&self, length: u16) -> (Self, Self) {
        if length >= self.width {
            return (*self, Rect::ZERO);
        }
        (
            Rect::new(self.x, self.y, length, self.height),
            Rect::new(self.x + length, self.y, self.width - length, self.height),
        )
    }

    pub fn vsplit_len(&self, length: u16) -> (Self, Self) {
        if length >= self.height {
            return (*self, Rect::ZERO);
        }
        (
            Rect::new(self.x, self.y, self.width, length),
            Rect::new(self.x, self.y + length, self.width, self.height - length),
        )
    }
}



struct Label<'a> {
    content: Cow<'a, str>,
}

impl<'a> Label<'a> {
    fn new(s: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: s.into(),
        }
    }

    fn render(self, area: Rect, buf: &mut Buffer) {
        for (line_index, line) in self.content.lines().enumerate() {
            if line_index >= area.height as usize {
                return;
            }
            if line.len() > area.width as usize {
                // let (line_a, line_b) = self.content.chars()
                //     .enumerate()
                //     .partition(|(i, _)| i < &(area.width as usize));
            } else {
                for (i, c) in line.chars().enumerate() {
                    let index = buf.index_of(area.x + i as u16, area.y);
                    buf.set(index, c);
                }
            }
        }
    }
}
