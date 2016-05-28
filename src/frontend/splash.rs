use std::thread::{self, JoinHandle};
use ui::UI;
use conrod::{color, Canvas, Frameable, Colorable, Widget};
use std::sync::{Arc, Mutex};
use window::*;
use piston_window::UpdateEvent;
use std::any::Any;

widget_ids! {
    CANVAS,
}

pub struct SplashWindow {
    thread: JoinHandle<()>
}

impl SplashWindow {
    pub fn new(ui: Arc<Mutex<UI>>) -> SplashWindow {
        SplashWindow {
            thread: thread::spawn(move || {
                let (mut window, mut conrod_ui) = create_window("BitDMX Splashscreen".to_string(), (500, 300), 1, true);
                while let Some(event) = window.next() {
                    conrod_ui.handle_event(&event);
                    window.draw_2d(&event, |c, g| conrod_ui.draw(c, g));

                    event.update(|_| conrod_ui.set_widgets(|mut conrod_ui| {
                        Canvas::new()
                            .frame(1.0)
                            .pad(30.0)
                            .color(color::rgb(0.236, 0.239, 0.900))
                            .set(CANVAS, &mut conrod_ui);
                    }));

                    if ui.lock().unwrap().watchdog.is_alive() { break };
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
