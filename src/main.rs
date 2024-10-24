


use dreg::prelude::*;



#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    let program = Website {};
    let platform = CrosstermPlatform::new()?;
    run_program(program, platform)?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<()> {
    let program = Website {};
    let platform = WasmPlatform::new();
    run_program(program, platform).unwrap();
    Ok(())
}



struct Website {}

impl Program for Website {
    fn update(&mut self, frame: Frame) {
        let label_area = frame.area.inner_centered(5, 1);
        frame.buffer.set_stringn(
            label_area.x,
            label_area.y,
            "rtthw",
            5,
            Style::new().add_modifier(Modifier::BOLD),
        );
    }

    fn on_input(&mut self, input: Input) {
        match input {
            _ => {}
        }
    }

    fn on_platform_request(&self, request: &str) -> Option<&str> {
        Some(match request {
            "font" => "29px Hack, monospace",
            "font_size" => "29",
            "web::default_fg_style" => "#bcbec4",
            "web::default_bg_style" => "#1e1f22",
            _ => {
                return None;
            }
        })
    }

    // Exiting the website is obviously handled by the browser itself, no need to handle it.
    fn should_exit(&self) -> bool { false }
}
