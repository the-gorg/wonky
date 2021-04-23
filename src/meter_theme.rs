use tinybit::widgets::Text;
use tinybit::Color;
use tinybit::ScreenPos;
use tinybit::Viewport;

pub struct MeterTheme {
    start: char,
    end: char,
    meter: char,
    meterbg: Option<char>,
    width: u8,

    text: String,
}

impl MeterTheme {
    pub fn draw_meter(
        &self,
        viewport: &mut Viewport,
        (current, max): (f32, f32),
        position: ScreenPos,
    ) {
        let progress = current / max * (self.width as f32 - 2_f32 - self.text.len() as f32);
        let bar = std::iter::repeat(self.meter)
            .take(progress as usize)
            .collect::<String>();

        let clear = std::iter::repeat(' ')
            .take((self.width as usize).saturating_sub(2 + self.text.len()))
            .collect::<String>();

        // draw background
        viewport.draw_widget(
            &Text::new(
                format!("{}{}{}{}", self.text, self.start, clear, self.end),
                fg_color(),
                None,
            ),
            position,
        );
        match self.meterbg {
            Some(c) => {
                let bgbar = std::iter::repeat(c)
                    .take((self.width as usize).saturating_sub(2 + self.text.len()))
                    .collect::<String>();
                viewport.draw_widget(
                    &Text::new(format!("{}", bgbar), bg_color(), None),
                    ScreenPos::new(position.x + self.text.len() as u16 + 1, position.y),
                );
            }
            _ => {}
        }

        // draw meter
        viewport.draw_widget(
            &Text::new(format!("{}", bar), fg_color(), None),
            ScreenPos::new(position.x + self.text.len() as u16 + 1, position.y),
        );
    }
}

// Meter presets
impl MeterTheme {
    pub fn default(width: u8, title: &str) -> Self {
        Self {
            start: '[',
            end: ']',
            meter: '▪',
            width,
            text: title.to_string(),
            meterbg: Some('□'),
        }
    }
    pub fn halfblock(width: u8, title: &str) -> Self {
        Self {
            start: '▀',
            end: ' ',
            meter: '▀',
            width,
            text: title.to_string(),
            meterbg: Some('▀'),
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
