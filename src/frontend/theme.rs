use conrod::color;

#[derive(Debug, Clone)]
pub struct Theme {
     pub ui_scale: f64,
     pub base_font_size: f64,
     pub ui_padding: f64,
     pub bg_color: color::Color
}

impl Theme {
    pub fn default() -> Theme {
        Theme {
            ui_scale: 0.8,
            base_font_size: 20.0,
            ui_padding: 10.0,
            bg_color: color::rgb(0.236, 0.239, 0.241)
        }
    }
}
