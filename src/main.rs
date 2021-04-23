use chrono::{Datelike, Local, Timelike};
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
pub use settings::Conf;

fn main() {
    let (width, height) = term_size().unwrap();

    let mut test = Meter::new();
    test.init();

    let mut settings = Config::default();

    settings.merge(File::with_name("config.toml")).unwrap();
    let mut conf = settings.try_into::<Conf>().unwrap();

    for blerp in conf.meters.iter_mut() {
        blerp.init();
    }

    let target = StdoutTarget::new().unwrap();
    let mut renderer = Renderer::new(target);
    let mut viewport =
        Viewport::new(ScreenPos::zero(), ScreenSize::new(width, height));

    let mut time = Local::now();
    let days_month = days_in_month::days_in_month(time.year(), time.month());

    let mut hd1 = psutil::disk::disk_usage("/").expect("blerp");
    let mut hd2 = psutil::disk::disk_usage("/mnt/dump").expect("blerp");

    let mut bloatie = Bloatie::new(width - 6, 0);
    bloatie.speak("Hello!!");

    let sleepy_time = 0..7;

    let meter_test = MeterTheme::default(width as u8, "Progress:");
    let other_meter = MeterTheme::halfblock((width / 2 - 2) as u8, "");

    let mut frame_counter = 0;
    let mut timer = std::time::Instant::now();

    for event in events(EventModel::Fps(3)) {
        match event {
            Event::Tick => {
                let mut top = 0_u16;
                for blerp in conf.meters.iter_mut() {
                    blerp.update();
                    viewport.draw_widget(
                        &Text::new(
                            format!(
                                "{}:{:05}/{:05}{}",
                                blerp.title,
                                blerp.current_value,
                                blerp.max_value,
                                blerp.unit
                            ),
                            fg_color(),
                            None,
                        ),
                        ScreenPos::new(0, top * 2),
                    );
                    other_meter.draw_meter(
                        &mut viewport,
                        (blerp.current_value as f32, blerp.max_value as f32),
                        ScreenPos::new(0, top * 2 + 1),
                    );

                    top += 1
                }

                // TODO: Prob make up something better
                if frame_counter == 5 {
                    hd1 = psutil::disk::disk_usage("/").expect("blerp");
                    hd2 = psutil::disk::disk_usage("/mnt/dump").expect("blerp");

                    frame_counter = 0
                }
                time = Local::now();

                // Weekmeter
                viewport.draw_widget(
                    &Text::new(
                        format!(
                            "{}           {}/{}",
                            time.weekday(),
                            time.weekday().number_from_monday(),
                            7
                        ),
                        fg_color(),
                        None,
                    ),
                    ScreenPos::new(0, height - 5),
                );

                other_meter.draw_meter(
                    &mut viewport,
                    (time.weekday().number_from_monday() as f32, 7 as f32),
                    ScreenPos::new(0, height - 4),
                );

                // Weekmeter
                viewport.draw_widget(
                    &Text::new(
                        format!(
                            "Week        {:01}/{}",
                            time.iso_week().week(),
                            52
                        ),
                        fg_color(),
                        None,
                    ),
                    ScreenPos::new(0, height - 3),
                );

                other_meter.draw_meter(
                    &mut viewport,
                    (time.iso_week().week() as f32, 52 as f32),
                    ScreenPos::new(0, height - 2),
                );

                // Date and stuff
                //viewport.draw_widget(&month_text, ScreenPos::new(0, height - 3));
                //viewport.draw_widget(&week_text, ScreenPos::new(0, height - 2));

                meter_test.draw_meter(
                    &mut viewport,
                    (time.day() as f32, days_month as f32),
                    ScreenPos::new(0, height - 1),
                );

                // Character
                match bloatie.animation() {
                    Some(_) => {}
                    None => {
                        if sleepy_time.contains(&time.hour()) {
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
                frame_counter += 1;
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

#[allow(dead_code)]
fn fg_color() -> Option<Color> {
    Some(Color::Green)
}

#[allow(dead_code)]
fn bg_color() -> Option<Color> {
    Some(Color::DarkGreen)
}
