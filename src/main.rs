


mod widgets;


use dreg::prelude::*;

use widgets::*;



#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    let program = Website {
        hashchange_info: String::new(),
    };
    let platform = CrosstermPlatform::new()?;
    run_program(program, platform)?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<()> {
    let program = Website {
        hashchange_info: String::new(),
    };
    let platform = WasmPlatform::new();
    run_program(program, platform).unwrap();
    Ok(())
}



struct Website {
    hashchange_info: String,
}

impl Program for Website {
    fn update(&mut self, mut frame: Frame) {
        Corners::new().render(frame.area, &mut frame.buffer);

        let label_area = frame.area.inner_centered(5, 1);
        frame.buffer.set_stringn(
            label_area.x,
            label_area.y,
            "rtthw",
            5,
            Style::new().add_modifier(Modifier::BOLD),
        );
        frame.buffer.set_stringn(
            1,
            1,
            &self.hashchange_info,
            frame.area.width.saturating_sub(2) as usize,
            Style::new().add_modifier(Modifier::DIM),
        );
    }

    fn on_input(&mut self, input: Input) {
        match input {
            _ => {}
        }
    }

    fn on_platform_request(&mut self, request: &str) -> Option<&str> {
        Some(match request {
            "font" => "29px Hack, monospace",
            "font_size" => "29",
            "web::default_fg_style" => "#bcbec4",
            "web::default_bg_style" => "#1e1f22",
            req => {
                if let Some(web_req) = req.strip_prefix("web::") {
                    if let Some(hashchange) = web_req.strip_prefix("hashchange::") {
                        self.hashchange_info = hashchange.to_string();
                    }
                }
                return None;
            }
        })
    }

    // Exiting the website is obviously handled by the browser itself, no need to handle it.
    fn should_exit(&self) -> bool { false }
}
