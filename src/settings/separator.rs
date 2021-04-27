use anyhow::Result;
use serde::Deserialize;
use tinybit::{widgets::Text, Color, ScreenPos, Viewport};

use super::{parse_ansi, Widget};

#[derive(Debug, Deserialize)]
pub struct SeperatorTheme {
    fg: Option<u8>,
    bg: Option<u8>,

    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl SeperatorTheme {
    pub fn init(&mut self) {
        self.fg_color = parse_ansi(self.fg);
        self.bg_color = parse_ansi(self.bg);
    }
}

impl Default for SeperatorTheme {
    fn default() -> Self {
        Self {
            fg: Some(7),
            bg: Some(245),
            fg_color: None,
            bg_color: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Separator {
    pub title: Option<String>,
    pub right: bool,
    pub bottom: bool,

    #[serde(default)]
    pub theme: SeperatorTheme,
}

impl Separator {}

//----------------------------------------------------------------------------+
// Trait Impl                                                                 |
//----------------------------------------------------------------------------+
impl Default for Separator {
    fn default() -> Self {
        Self {
            title: None,
            right: false,
            bottom: false,
            theme: SeperatorTheme {
                fg: Some(7),
                bg: Some(245),
                fg_color: None,
                bg_color: None,
            },
        }
    }
}
impl Widget for Separator {
    fn update_and_draw(
        &mut self,
        viewport: &mut Viewport,
        pos: &mut ScreenPos,
        _resized: &bool,
    ) -> Result<()> {
        if let Some(t) = &self.title {
            viewport.draw_widget(
                &Text::new(t, self.theme.fg_color, self.theme.bg_color),
                ScreenPos::new(pos.x, pos.y),
            );
        }

        Ok(())
    }

    fn is_bottom(&self) -> bool {
        self.bottom
    }

    fn is_right(&self) -> bool {
        self.right
    }

    fn vertical_size(&self) -> u8 {
        1
    }
}
