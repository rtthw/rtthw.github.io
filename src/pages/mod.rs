//! Pages



use dreg::prelude::*;

use crate::Corners;



pub enum Page {
    Home,
    About,
    NotFound,
}

impl Page {
    pub fn from_hashchange(hashchange: &str) -> Self {
        if let Some((_part_a, part_b)) = hashchange.split_once('#') {
            match part_b {
                "about" => Self::About,
                _ => Self::NotFound,
            }
        } else {
            Self::Home
        }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        Corners::new().render(area, buf);

        match self {
            Page::Home => {
                let label_area = area.inner_centered(5, 1);
                buf.set_stringn(
                    label_area.x,
                    label_area.y,
                    "rtthw",
                    5,
                    Style::new().bold(),
                );
            }
            Page::About => {
                let label_area = area.inner_centered(5, 1);
                buf.set_stringn(
                    label_area.x,
                    label_area.y,
                    "About",
                    5,
                    Style::new().bold(),
                );
            }
            Page::NotFound => {
                let label_area = area.inner_centered(14, 1);
                buf.set_stringn(
                    label_area.x,
                    label_area.y,
                    "404: Not Found",
                    14,
                    Style::new().bold(),
                );
            }
        }
    }
}
