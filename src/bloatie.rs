use std::iter;

use tinybit::widgets::Text;
use tinybit::Color;
use tinybit::ScreenPos;
use tinybit::Viewport;

pub struct Bloatie {
    sprite_buffers: &'static [&'static str],
    animation: Option<BloatieAnimation>,
    x: u16,
    y: u16,
    frame: usize,
}

impl Bloatie {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            sprite_buffers: &["(._. )"],
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
        let text = text.to_owned();
        let frames: Vec<&str> = vec!["(⋅-⋅ )", "(⋅o⋅ )"]
            .into_iter()
            .cycle()
            .take(text.len())
            .collect();

        let speech = text
            .char_indices()
            .step_by(2)
            .map(|(i, _)| text[..i].to_owned())
            .chain(std::iter::once(text.clone()))
            .collect();

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
                        let frame = Text::new(animation.frames[n], Some(Color::White), None);
                        viewport.draw_widget(&frame, ScreenPos::new(self.x, self.y));
                        false
                    }
                    _ => {
                        let frame = Text::new(
                            animation.frames.last().cloned().unwrap_or_default(),
                            Some(Color::White),
                            None,
                        );
                        viewport.draw_widget(&frame, ScreenPos::new(self.x, self.y));
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
                                speech_frames.last().map(String::as_str).unwrap_or_default(),
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
                let frame = Text::new("(._. )".to_string(), Some(Color::White), None);
                viewport.draw_widget(&frame, ScreenPos::new(self.x, self.y));
            }
        }
    }

    fn speech(&self, dialogue: &str, viewport: &mut Viewport) {
        let text = Text::new(
            format!(" {} ", dialogue),
            Some(Color::Black),
            Some(Color::White),
        );

        let space = iter::repeat(' ')
            .take(dialogue.len() + 2)
            .collect::<String>();

        viewport.draw_widget(
            &Text::new(format!("{}", space), Some(Color::White), None),
            ScreenPos::new(
                self.x - 2_u16 - dialogue.len() as u16 / 2,
                self.y + self.sprite_buffers.len() as u16 + 1,
            ),
        );

        viewport.draw_widget(
            &Text::new("".to_string(), Some(Color::White), None),
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

    pub fn relocate(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}

pub struct BloatieAnimation {
    frames: Vec<&'static str>,
    speech: Option<Vec<String>>,
}

impl BloatieAnimation {
    pub fn hello() -> Self {
        Self {
            frames: vec!["(⋅-⋅ )", "(⋅o⋅ )", "(⋅-⋅ )", "(⋅o⋅ )", "(⋅-⋅ )"],
            speech: Some(
                iter::once("")
                    .chain(iter::once("He"))
                    .chain(iter::repeat("Hello!").take(8))
                    .map(str::to_owned)
                    .collect(),
            ),
        }
    }

    pub fn idle() -> Self {
        Self {
            frames: iter::repeat("(._. )")
                .take(2)
                .chain(iter::once("(⋅-⋅ )"))
                .chain(iter::repeat("( ⋅-⋅)").take(8))
                .chain(iter::once("(⋅-⋅ )"))
                .chain(iter::once("(._. )"))
                .collect(),
            speech: None,
        }
    }
    pub fn sleep() -> Self {
        Self {
            frames: vec!["(─ρ─ )"],
            speech: Some(vec!["Z".to_string(), "Zz".to_string(), "Zzz".to_string()]),
        }
    }
    pub fn sleep_alt() -> Self {
        Self {
            frames: iter::repeat("(°ρ° )")
                .take(5)
                .chain(iter::repeat("( °ρ°)").take(6))
                .chain(iter::repeat("(°ρ° )").take(3))
                .chain(iter::once("(-ρ- )"))
                .collect(),
            speech: None,
        }
    }
}
