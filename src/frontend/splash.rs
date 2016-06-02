use std::thread::{self, JoinHandle};
use structures::ui::ui::UI;
use conrod::{Canvas, Text, Frameable, Colorable, Sizeable, Positionable, Widget};
use std::sync::{Arc, Mutex};
use piston_window::UpdateEvent;
use std::any::Any;

use structures::ui::colors::FlatColor;
use structures::ui::window::{create_window, DMXWindow};

widget_ids! {
    CANVAS,
    SPLASH_TEXT_BIT,
    SPLASH_TEXT_DMX,
    SPLASH_SEARCHING_FOR_SERVER
}

pub struct SplashWindow {
    thread: JoinHandle<()>
}

impl SplashWindow {
    pub fn new(ui: Arc<Mutex<UI>>) -> SplashWindow {
        SplashWindow {
            thread: thread::spawn(move || {
                let (mut window, mut conrod_ui) = match create_window("BitDMX Splashscreen".to_string(), (500, 300), 3, true) {
                    Ok(res) => res,
                    Err(e) => {
                        exit!(3, e);
                    }
                };

                let mut i = 0;
                while let Some(event) = window.next() {
                    conrod_ui.handle_event(&event);
                    window.draw_2d(&event, |c, g| conrod_ui.draw(c, g));

                    event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| {
                        Canvas::new()
                            .frame(1.0)
                            .pad(5.0)
                            .color(FlatColor::midnight_blue())
                            .set(CANVAS, &mut conrod_ui);

                        Text::new("Bit")
                            .w_h(200.0, 100.0)
                            .middle_of(CANVAS)
                            .font_size(100)
                            .color(FlatColor::clouds())
                            .set(SPLASH_TEXT_BIT, &mut conrod_ui);

                        Text::new("DMX")
                            .w_h(50.0, 20.0)
                            .bottom_right_of(SPLASH_TEXT_BIT)
                            .font_size(20)
                            .color(FlatColor::clouds())
                            .set(SPLASH_TEXT_DMX, &mut conrod_ui);

                        let label = "Searching for server";
                        let dots = (0..i).map(|_| " .").collect::<String>();
                        Text::new(&format!("{}{}", label, dots))
                            .w_h(150.0, 20.0)
                            // .x_y(290.0, 270.0)
                            .bottom_right_with_margin_on(CANVAS, 5.0)
                            .font_size(12)
                            .color(FlatColor::clouds())
                            .set(SPLASH_SEARCHING_FOR_SERVER, &mut conrod_ui);

                        i = if i > 2 { 0 } else { i+1 };
                    }));

                    if ui.lock().expect("Failed to lock Arc!").watchdog.is_alive() { break };
                };
            })
        }
    }
}

impl DMXWindow for SplashWindow {
    fn join(self) -> Result<(), Box<Any + Send + 'static>> {
        self.thread.join()
    }
}
