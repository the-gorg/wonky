//      █░█░█ █▀█ █▄░█ █▄▀ █▄█
//      ▀▄▀▄▀ █▄█ █░▀█ █░█ ░█░
// For your terminal monitoring needs
//
use anyhow::Result;
use chrono::{Local, Timelike};
use rand::Rng;

use tinybit::events::{
    events, Event, EventModel, KeyCode, KeyEvent, KeyModifiers,
};
use tinybit::render::{Renderer, StdoutTarget};
use tinybit::widgets::Text;
use tinybit::{term_size, Color, ScreenPos, ScreenSize, Viewport};

mod bloatie;
mod meter_theme;
mod settings;

pub use bloatie::{Bloatie, BloatieAnimation};
pub use meter_theme::MeterTheme;
pub use settings::{Conf, Widget};

fn main() -> Result<()> {
    let (mut width, mut height) = term_size()?;
    let target = StdoutTarget::new()?;
    let mut renderer = Renderer::new(target);
    let mut viewport =
        Viewport::new(ScreenPos::zero(), ScreenSize::new(width, height));

    let mut conf = settings::load()?;

    let mut positions: [Vec<&mut Widget>; 4] = [vec![], vec![], vec![], vec![]];
    for w in conf.widgets.iter_mut() {
        match w {
            Widget::Meter(m) => {
                m.init()?;
                positions[pos_index(m.right, m.bottom)].push(w);
            }
            Widget::Indicator(i) => {
                i.init()?;
                positions[pos_index(i.right, i.bottom)].push(w);
            }
        }
    }

    let mut bloatie = Bloatie::new(width - 6, 0);
    bloatie.speak("Hello!!");

    let sleepy_time = 0..7;

    let meter_themes = vec![
        MeterTheme::halfblock((width / 2 - 2) as u8, ""),
        MeterTheme::default((width / 2 - 2) as u8, ""),
    ];
    #[allow(unused_variables)]
    let mut blocky_theme =
        MeterTheme::default((width / 2 - 2) as u8, "Progress:");
    let mut normal_theme = MeterTheme::halfblock((width / 2 - 2) as u8, "");

    let mut timer = std::time::Instant::now();

    for event in events(EventModel::Fps(3)) {
        match event {
            Event::Tick => {
                (0..4).try_for_each(|n| {
                    let right = n == 1 || n == 3;
                    let bottom = n == 2 || n == 3;

                    let increment = if bottom { -1 } else { 1 };

                    let vertical_pos =
                        if bottom { height as i16 - 2 } else { 0 };
                    let horizontal_pos = if right { width / 2 + 3 } else { 0 };

                    let mut i = 0;

                    for thing in positions[n].iter_mut() {
                        match thing {
                            Widget::Meter(m) => {
                                m.update_and_draw(
                                    &mut viewport,
                                    &mut ScreenPos::new(
                                        horizontal_pos,
                                        (vertical_pos + i) as u16,
                                    ),
                                    &meter_themes[m.theme],
                                    increment,
                                )?;

                                // If single-line or not, make prettier
                                if !m.meter || !m.reading || m.title == "" {
                                    i += increment;
                                } else {
                                i += increment * 2;
                            }
                            }
                            Widget::Indicator(_) => {
                                //
                            }
                        }
                    }

                    Ok::<_, anyhow::Error>(())
                })?;

                // Character
                if conf.settings.bloatie {
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
                                    bloatie.play_animation(
                                        BloatieAnimation::sleep(),
                                    );
                            }
                        } else {
                            let mut rng = rand::thread_rng();
                            if timer.elapsed().as_secs() > 10
                                && rng.gen_range(0..100) > 95
                            {
                                timer = std::time::Instant::now();
                                    bloatie.play_animation(
                                        BloatieAnimation::idle(),
                                    );
                            }
                        }
                    }
                }
                bloatie.update(&mut viewport);
                }
                renderer.render(&mut viewport);
            }
            Event::Key(KeyEvent { code, modifiers }) => match code {
                KeyCode::Enter | KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => {
                    return Ok(())
                }
                KeyCode::Char('f') => bloatie.speak("It works!!"),
                _ => {}
            },
            Event::Resize(w, h) => {
                width = w;
                height = h;
                viewport.resize(width, height);
                renderer.clear();

                bloatie.relocate(width - 6, 0);
                blocky_theme.resize(width as u8);
                normal_theme.resize((width / 2 - 2) as u8);
            }
        }
    }

    Ok(())
}
#[allow(dead_code, clippy::unnecessary_wraps)]
fn fg_color() -> Option<Color> {
    Some(Color::Green)
}

#[allow(dead_code, clippy::unnecessary_wraps)]
fn bg_color() -> Option<Color> {
    Some(Color::DarkGreen)
}

fn pos_index(right: bool, bottom: bool) -> usize {
    right as usize | (bottom as usize) << 1
}
