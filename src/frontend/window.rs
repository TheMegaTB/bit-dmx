use piston_window::{ EventLoop, OpenGL, Glyphs, PistonWindow, WindowSettings, self };
use find_folder;
use std::any::Any;
use conrod::{Theme, self};

pub type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
pub type Ui = conrod::Ui<Backend>;
pub type UiCell<'a> = conrod::UiCell<'a, Backend>;

const OPEN_GL: OpenGL = OpenGL::V3_2;

pub trait DMXWindow {
    fn join(self) -> Result<(), Box<Any + Send + 'static>>;
}

pub fn create_window(title: String, size: (u32, u32), ups: u64, esc: bool) -> (PistonWindow, Ui) {
    let mut window: PistonWindow = WindowSettings::new(title, size)
                                    .opengl(OPEN_GL).exit_on_esc(esc).vsync(true).build().unwrap();

    let conrod_ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    window.set_ups(ups);
    (window, conrod_ui)
}
