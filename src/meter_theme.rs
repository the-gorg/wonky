use std::iter;

use tinybit::widgets::Text;
use tinybit::Color;
use tinybit::ScreenPos;
use tinybit::Viewport;

pub struct MeterTheme<'a> {
    start: Option<char>,
    end: Option<char>,
    meter: char,
    meterbg: Option<char>,
    width: u8,

    text: &'a str,
}

impl<'a> MeterTheme<'a> {
    pub fn draw(
        &self,
        viewport: &mut Viewport,
        (current, max): (f32, f32),
        position: ScreenPos,
    ) {
        let decoration_size =
            self.start.is_some() as u8 + self.end.is_some() as u8;

        let start = match self.start {
            Some(c) => c.to_string(),
            _ => "".to_string(),
        };
        let end = match self.end {
            Some(c) => c.to_string(),
            _ => "".to_string(),
        };

        let bar_width = self.width - self.text.len() as u8 - decoration_size;

        let progress = current / max
            * (self.width as f32
                - decoration_size as f32
                - self.text.len() as f32);

        let bar = iter::repeat(self.meter)
            .take(progress as usize)
            .collect::<String>();

        let clear = iter::repeat(' ')
            .take(bar_width as usize)
            .collect::<String>();

        // draw background
        viewport.draw_widget(
            &Text::new(
                format!("{}{}{}{}", self.text, start, clear, end),
                fg_color(),
                None,
            ),
            position,
        );

        if let Some(c) = self.meterbg {
            let bgbar =
                iter::repeat(c).take(bar_width as usize).collect::<String>();
            viewport.draw_widget(
                &Text::new(bgbar, bg_color(), None),
                ScreenPos::new(
                    position.x
                        + self.start.is_some() as u16
                        + self.text.len() as u16,
                    position.y,
                ),
            );
        }

        // draw meter
        viewport.draw_widget(
            &Text::new(bar, fg_color(), None),
            ScreenPos::new(
                position.x
                    + self.start.is_some() as u16
                    + self.text.len() as u16,
                position.y,
            ),
        );
    }

    pub fn resize(&mut self, width: u8) {
        self.width = width;
    }

    pub fn default(width: u8, title: &'a str) -> Self {
        Self {
            start: Some('['),
            end: Some(']'),
            meter: '▪',
            width,
            text: title,
            meterbg: Some('□'),
        }
    }
    pub fn halfblock(width: u8, title: &'a str) -> Self {
        Self {
            start: None,
            end: None,
            meter: '▀',
            width,
            text: title,
            meterbg: Some('▀'),
        }
    }
}

#[allow(dead_code, clippy::unnecessary_wraps)]
fn fg_color() -> Option<Color> {
    Some(Color::Green)
}

#[allow(dead_code, clippy::unnecessary_wraps)]
fn bg_color() -> Option<Color> {
    Some(Color::DarkGreen)
}
