use tinybit::widgets::Text;
use tinybit::Color;
use tinybit::ScreenPos;
use tinybit::Viewport;

#[allow(unused_macros)]
macro_rules! logit {
    ($($arg:tt)*) => {
        use std::fs::OpenOptions;
        use std::io::Write;
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("/tmp/logit.txt") {
                file.write_all(format!($($arg)*).as_bytes());
        }
    };
}

pub struct Bloatie {
    sprite_buffers: Vec<String>,
    animation: Option<BloatieAnimation>,
    x: u16,
    y: u16,
    frame: usize,
}

impl Bloatie {
    pub fn new(x: u16, y: u16) -> Self {
        let mut sprite = Vec::new();
        sprite.push("(._. )".to_string());
        Self {
            sprite_buffers: sprite,
            x,
            y,
            frame: 0,
            animation: None,
        }
    }

    pub fn animation(&self) -> &Option<BloatieAnimation> {
        &self.animation
    }

    pub fn play_animation(&mut self, animation: BloatieAnimation) {
        self.frame = 0;
        self.animation = Some(animation)
    }

    pub fn speak(&mut self, text: &str) {
        let mut speech_text = text.to_owned();
        let frames: Vec<&str> = vec!["(⋅-⋅ )", "(⋅o⋅ )"]
            .into_iter()
            .cycle()
            .take(speech_text.len())
            .collect();

        let mut speech: Vec<String> = Vec::new();

        while !speech_text.is_empty() {
            let mut frame = String::new();
            speech.last().map(|s| frame.push_str(s));

            // TODO: look at this
            if speech_text.len() == 1 {
                speech.push(frame + &speech_text.pop().unwrap().to_string());
            } else {
                speech
                    .push(frame + &speech_text.drain(0..2).collect::<String>());
            }
        }

        let animation = BloatieAnimation {
            frames,
            speech: Some(speech),
        };

        self.play_animation(animation)
    }

    pub fn update(&mut self, viewport: &mut Viewport) {
        match &self.animation {
            Some(animation) => {
                let animation_complete = match self.frame {
                    n if n < animation.frames.len() => {
                        let frame = Text::new(
                            format!("{}", animation.frames[n]),
                            Some(Color::White),
                            None,
                        );
                        viewport.draw_widget(
                            &frame,
                            ScreenPos::new(self.x, self.y),
                        );
                        false
                    }
                    _ => {
                        let frame = Text::new(
                            format!("{}", animation.frames.last().unwrap()),
                            Some(Color::White),
                            None,
                        );
                        viewport.draw_widget(
                            &frame,
                            ScreenPos::new(self.x, self.y),
                        );
                        true
                    }
                };

                let speech_complete = match &animation.speech {
                    Some(speech_frames) => match self.frame {
                        n if n < speech_frames.len() => {
                            self.speech(&speech_frames[n], viewport);
                            false
                        }
                        _ => {
                            self.speech(
                                &speech_frames.last().unwrap(),
                                viewport,
                            );
                            true
                        }
                    },
                    _ => true,
                };

                if speech_complete && animation_complete {
                    self.animation = None;
                }
                self.frame += 1;
            }
            _ => {
                let frame = Text::new(
                    format!("{}", "(._. )".to_string()),
                    Some(Color::White),
                    None,
                );
                viewport.draw_widget(&frame, ScreenPos::new(self.x, self.y));
            }
        }
    }

    fn speech(&self, dialogue: &String, viewport: &mut Viewport) {
        let text = Text::new(
            format!(" {} ", dialogue),
            Some(Color::Black),
            Some(Color::White),
        );

        let space = std::iter::repeat(' ')
            .take(dialogue.len() + 2)
            .collect::<String>();

        viewport.draw_widget(
            &Text::new(format!("{}", space), Some(Color::White), None),
            ScreenPos::new(
                self.x - 2 as u16 - dialogue.len() as u16 / 2,
                self.y + self.sprite_buffers.len() as u16 + 1,
            ),
        );

        viewport.draw_widget(
            &Text::new(format!(""), Some(Color::White), None),
            ScreenPos::new(self.x, self.y + self.sprite_buffers.len() as u16),
        );

        viewport.draw_widget(
            &text,
            ScreenPos::new(
                self.x - 1 - dialogue.len() as u16 / 2,
                self.y + self.sprite_buffers.len() as u16 + 1,
            ),
        );
    }
}

pub struct BloatieAnimation {
    frames: Vec<&'static str>,
    speech: Option<Vec<String>>,
}

impl BloatieAnimation {
    pub fn hello() -> Self {
        let mut frames = Vec::new();
        frames.push("(⋅-⋅ )");
        frames.push("(⋅o⋅ )");
        frames.push("(⋅-⋅ )");
        frames.push("(⋅o⋅ )");
        frames.push("(⋅-⋅ )");

        let mut speech = Vec::new();
        speech.push("".to_string());
        speech.push("He".to_string());
        speech.push("Hello!".to_string());
        speech.push("Hello!".to_string());
        speech.push("Hello!".to_string());
        speech.push("Hello!".to_string());
        speech.push("Hello!".to_string());
        speech.push("Hello!".to_string());
        speech.push("Hello!".to_string());
        speech.push("Hello!".to_string());

        Self {
            frames,
            speech: Some(speech),
        }
    }

    pub fn idle() -> Self {
        let mut frames = Vec::new();
        frames.push("(._. )");
        frames.push("(⋅_⋅ )");
        frames.push("(⋅-⋅ )");
        frames.push("( ⋅-⋅)");
        frames.push("( ⋅-⋅)");
        frames.push("( ⋅-⋅)");
        frames.push("( ⋅-⋅)");
        frames.push("( ⋅-⋅)");
        frames.push("( ⋅-⋅)");
        frames.push("( ⋅-⋅)");
        frames.push("( ⋅-⋅)");
        frames.push("(⋅-⋅ )");
        frames.push("(._. )");

        Self {
            frames,
            speech: None,
        }
    }
    pub fn sleep() -> Self {
        let mut frames = Vec::new();
        frames.push("(─ρ─ )");

        let mut speech = Vec::new();
        speech.push("Z".to_string());
        speech.push("Zz".to_string());
        speech.push("Zzz".to_string());

        Self {
            frames,
            speech: Some(speech),
        }
    }
    pub fn sleep_alt() -> Self {
        let mut frames = Vec::new();
        frames.push("(°ρ° )");
        frames.push("(°ρ° )");
        frames.push("(°ρ° )");
        frames.push("(°ρ° )");
        frames.push("(°ρ° )");
        frames.push("( °ρ°)");
        frames.push("( °ρ°)");
        frames.push("( °ρ°)");
        frames.push("( °ρ°)");
        frames.push("( °ρ°)");
        frames.push("( °ρ°)");
        frames.push("(°ρ° )");
        frames.push("(°ρ° )");
        frames.push("(°ρ° )");
        frames.push("(-ρ- )");

        Self {
            frames,
            speech: None,
        }
    }
}
