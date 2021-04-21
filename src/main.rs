use chrono::{Datelike, Local, Timelike};
use rand::Rng;

use tinybit::events::{events, Event, EventModel, KeyCode, KeyEvent};
use tinybit::render::{Renderer, StdoutTarget};
use tinybit::widgets::Text;
use tinybit::{term_size, Color, ScreenPos, ScreenSize, Viewport};

mod boopie;
mod meter;
pub use boopie::{Boopie, BoopieAnimation};
pub use meter::Meter;

fn main() {
    let (width, height) = term_size().unwrap();

    let target = StdoutTarget::new().unwrap();
    let mut renderer = Renderer::new(target);
    let mut viewport = Viewport::new(ScreenPos::zero(), ScreenSize::new(width, height));

    let time = Local::now();
    let days_month = days_in_month::days_in_month(time.year(), time.month());

    let mut hd1 = psutil::disk::disk_usage("/").expect("blerp");
    let mut hd2 = psutil::disk::disk_usage("/mnt/dump").expect("blerp");

    let mut blorper = Boopie::new(width - 6, 0);
    blorper.play_animation(BoopieAnimation::hello());
    let sleepy_time = 0..7;

    let week_text = Text::new(
        format!("Week: {}", &time.iso_week().week()),
        fg_color(),
        None,
    );

    let month_text = Text::new(
        format!(
            "Date: {} {}/{}",
            &time.weekday(),
            &time.day(),
            &time.month()
        ),
        fg_color(),
        None,
    );

    let meter_test = Meter::default(width as u8, "Progress:");
    let other_meter = Meter::halfblock((width / 2 - 2) as u8, "");

    let mut frame_counter = 0;
    let mut timer = std::time::Instant::now();

    for event in events(EventModel::Fps(3)) {
        match event {
            Event::Tick => {
                // TODO: Prob make up something better
                if frame_counter == 5 {
                    hd1 = psutil::disk::disk_usage("/").expect("blerp");
                    hd2 = psutil::disk::disk_usage("/mnt/dump").expect("blerp");

                    frame_counter = 0
                }

                // Hd1
                viewport.draw_widget(
                    &Text::new(
                        format!(
                            "Hd1:   {:03}/{:03}gb",
                            hd1.free() / 1000000000,
                            hd1.total() / 1000000000
                        ),
                        fg_color(),
                        None,
                    ),
                    ScreenPos::new(0, 0),
                );

                other_meter.draw_meter(
                    &mut viewport,
                    (hd1.free() as f32, hd1.total() as f32),
                    ScreenPos::new(0, 1),
                );

                // Hd2
                viewport.draw_widget(
                    &Text::new(
                        format!(
                            "Hd2:   {:03}/{:03}gb",
                            hd2.free() / 1000000000,
                            hd2.total() / 1000000000
                        ),
                        fg_color(),
                        None,
                    ),
                    ScreenPos::new(0, 2),
                );

                other_meter.draw_meter(
                    &mut viewport,
                    (hd2.free() as f32, hd2.total() as f32),
                    ScreenPos::new(0, 3),
                );

                // Date and stuff
                viewport.draw_widget(&month_text, ScreenPos::new(0, height - 3));
                viewport.draw_widget(&week_text, ScreenPos::new(0, height - 2));

                meter_test.draw_meter(
                    &mut viewport,
                    (time.day() as f32, days_month as f32),
                    ScreenPos::new(0, height - 1),
                );

                // Character
                match blorper.animation() {
                    Some(_) => {}
                    None => {
                        if sleepy_time.contains(&time.hour()) {
                            let mut rng = rand::thread_rng();
                            if rng.gen_range(0..200) == 199 {
                                blorper.play_animation(BoopieAnimation::sleep_alt());
                            } else {
                                blorper.play_animation(BoopieAnimation::sleep());
                            }
                        } else {
                            let mut rng = rand::thread_rng();
                            if timer.elapsed().as_secs() > 10 && rng.gen_range(0..100) > 95 {
                                timer = std::time::Instant::now();
                                blorper.play_animation(BoopieAnimation::idle());
                            }
                        }
                    }
                }

                blorper.update(&mut viewport);
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
            }) => blorper.speak("It works!!"),
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
