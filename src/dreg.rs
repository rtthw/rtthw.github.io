//! Dummy Dreg Implementation



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

    let painter = ui.painter();
    for x in 0..cols {
        for y in 0..rows {
            let i = buf.index_of(x, y);
            let pos = egui::pos2(x as f32 * cell_width, y as f32 * state.font_size);
            painter.text(
                pos,
                egui::Align2::LEFT_TOP,
                buf.get(i),
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
        Self {
            cells: Vec::with_capacity((rows * cols) as usize),
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
}
