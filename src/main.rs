//
//      █░▄░█ █▀█ █▄░█ █▄▀ █▄█
//      ▀█▀█▀ █▄█ █░▀█ █░█ ░█░
// For your terminal monitoring needs
//
use crate::settings::Widget;
use anyhow::Result;
use chrono::{Local, Timelike};
use rand::Rng;
use std::env;

use tinybit::events::{
    events, Event, EventModel, KeyCode, KeyEvent, KeyModifiers,
};
use tinybit::render::{Renderer, StdoutTarget};
use tinybit::{term_size, Color, ScreenPos, ScreenSize, Viewport};

mod bloatie;
mod meter_theme;
mod settings;

pub use bloatie::{Bloatie, BloatieAnimation};
pub use meter_theme::MeterTheme;
pub use settings::{Conf, Element};

fn main() -> Result<()> {
    let (mut width, mut height) = term_size()?;
    let target = StdoutTarget::new()?;
    let mut renderer = Renderer::new(target);
    let mut viewport =
        Viewport::new(ScreenPos::zero(), ScreenSize::new(width, height));

    let mut conf = match env::args().nth(1) {
        None => settings::load()?,
        Some(path) => settings::load_at_path(&path)?,
    };

    #[allow(unused_variables)]
    let mut meter_themes = vec![
        MeterTheme::halfblock((width / 2 - 2) as u8),
        MeterTheme::default((width / 2 - 2) as u8),
    ];

    let mut positions: [Vec<&mut dyn Widget>; 4] =
        [vec![], vec![], vec![], vec![]];
    for w in conf.widgets.iter_mut() {
        match w {
            Element::Meter(m) => {
                m.init()?;
                m.set_theme(meter_themes[m.theme]);
                positions[pos_index(m.right, m.bottom)].push(m);
            }
            Element::Indicator(i) => {
                i.init()?;
                positions[pos_index(i.right, i.bottom)].push(i);
            }
            Element::Seperator(s) => {
                positions[pos_index(s.right, s.bottom)].push(s);
            }
        }
    }

    let mut bloatie = Bloatie::new(width - 6, 0);
    bloatie.speak("Hello!!");

    let sleepy_time = 0..7;

    let mut timer = std::time::Instant::now();

    for event in events(EventModel::Fps(3)) {
        match event {
            Event::Tick => {
                (0..4).try_for_each(|n| {
                    let right = n == 1 || n == 3;
                    let bottom = n == 2 || n == 3;

                    let increment = if bottom { -1 } else { 1 };

                    // TODO: Offsets are not great, but it works, figure out why
                    // at some point
                    let vertical_pos =
                        if bottom { height as i16 - 1 } else { 0 };
                    let horizontal_pos = if right { width / 2 + 2 } else { 0 };

                    let mut i = 0;

                    for thing in positions[n].iter_mut() {
                        thing.update_and_draw(
                            &mut viewport,
                            &mut ScreenPos::new(
                                horizontal_pos,
                                (vertical_pos + i) as u16,
                            ),
                        )?;

                        // If single-line or not, make prettier?
                        if thing.vertical_size() == 1 {
                            i += increment;
                        } else {
                            i += increment * 2;
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

                for mt in &mut meter_themes {
                    mt.resize((width / 2 - 2) as u8);
                }
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
