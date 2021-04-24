use chrono::{Local, Timelike};
use config::{Config, File};
use rand::Rng;

use tinybit::events::{events, Event, EventModel, KeyCode, KeyEvent};
use tinybit::render::{Renderer, StdoutTarget};
use tinybit::widgets::Text;
use tinybit::{term_size, Color, ScreenPos, ScreenSize, Viewport};

use crate::settings::Meter;

mod bloatie;
mod meter_theme;
mod settings;

pub use bloatie::{Bloatie, BloatieAnimation};
pub use meter_theme::MeterTheme;
pub use settings::{Conf, Widget};

fn main() {
    let (width, height) = term_size().unwrap();

    let mut test = Meter::new();
    test.init();

    let mut settings = Config::default();

    settings.merge(File::with_name("config.toml")).unwrap();
    let mut conf = settings.try_into::<Conf>().unwrap();

    let mut positions: [Vec<&mut Widget>; 4] = [vec![], vec![], vec![], vec![]];

    for w in conf.widgets.iter_mut() {
        match w {
            Widget::Meter(m) => {
                m.init();
                positions[pos_index(m.right, m.bottom)].push(w);
            }
            Widget::Indicator(i) => {
                i.init();
                positions[pos_index(i.right, i.bottom)].push(w);
            }
        }
    }

    let target = StdoutTarget::new().unwrap();
    let mut renderer = Renderer::new(target);
    let mut viewport =
        Viewport::new(ScreenPos::zero(), ScreenSize::new(width, height));

    let mut bloatie = Bloatie::new(width - 6, 0);
    bloatie.speak("Hello!!");

    let sleepy_time = 0..7;

    #[allow(unused_variables)]
    let blocky_theme = MeterTheme::default(width as u8, "Progress:");
    let normal_theme = MeterTheme::halfblock((width / 2 - 2) as u8, "");

    let mut timer = std::time::Instant::now();

    for event in events(EventModel::Fps(3)) {
        match event {
            Event::Tick => {
                for n in 0..4 {
                    let right: bool = n == 1 || n == 3;
                    let bottom: bool = n == 2 || n == 3;

                    let increment: i16 = if bottom { -1 } else { 1 };

                    let vertical_pos: i16 =
                        if bottom { height as i16 - 2 } else { 0 };

                    let mut i = 0;

                    for thing in positions[n].iter_mut() {
                        match thing {
                            Widget::Meter(m) => {
                                m.update();
                                viewport.draw_widget(
                                    &Text::new(
                                        format!("{}", m.title,),
                                        fg_color(),
                                        None,
                                    ),
                                    ScreenPos::new(
                                        0,
                                        (vertical_pos + (i)) as u16,
                                    ),
                                );
                                let test = Text::new(
                                    format!(
                                        "{}/{}{}",
                                        m.current_value, m.max_value, m.unit
                                    ),
                                    fg_color(),
                                    None,
                                );
                                viewport.draw_widget(
                                    &test,
                                    ScreenPos::new(
                                        // TODO: why 3?!?
                                        width / 2 - 3 - test.0.len() as u16,
                                        (vertical_pos + (i)) as u16,
                                    ),
                                );

                                normal_theme.draw_meter(
                                    &mut viewport,
                                    (
                                        m.current_value as f32,
                                        m.max_value as f32,
                                    ),
                                    ScreenPos::new(
                                        0,
                                        (vertical_pos + 1 + (i)) as u16,
                                    ),
                                );
                                i += increment * 2;
                            }
                            Widget::Indicator(_) => {
                                //
                            }
                        }
                    }
                }

                // Character
                match bloatie.animation() {
                    Some(_) => {}
                    None => {
                        if sleepy_time.contains(&Local::now().hour()) {
                            let mut rng = rand::thread_rng();
                            if rng.gen_range(0..350) == 199 {
                                bloatie.play_animation(
                                    BloatieAnimation::sleep_alt(),
                                );
                            } else {
                                bloatie
                                    .play_animation(BloatieAnimation::sleep());
                            }
                        } else {
                            let mut rng = rand::thread_rng();
                            if timer.elapsed().as_secs() > 10
                                && rng.gen_range(0..100) > 95
                            {
                                timer = std::time::Instant::now();
                                bloatie
                                    .play_animation(BloatieAnimation::idle());
                            }
                        }
                    }
                }

                bloatie.update(&mut viewport);
                renderer.render(&mut viewport);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => return,
            Event::Key(KeyEvent {
                code: KeyCode::Char('f'),
                ..
            }) => bloatie.speak("It works!!"),
            _ => {}
        }
    }
}

fn pos_index(right: bool, bottom: bool) -> usize {
    right as usize | (bottom as usize) << 1
}

#[allow(dead_code)]
fn fg_color() -> Option<Color> {
    Some(Color::Green)
}

#[allow(dead_code)]
fn bg_color() -> Option<Color> {
    Some(Color::DarkGreen)
}
