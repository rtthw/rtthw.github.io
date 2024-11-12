//! Pages



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
}
