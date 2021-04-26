use anyhow::Result;
use serde::Deserialize;
use tinybit::{widgets::Text, ScreenPos, Viewport};

use super::Widget;

#[derive(Debug, Deserialize)]
pub struct Seperator {
    pub title: Option<String>,
    pub right: bool,
    pub bottom: bool,
}

impl Seperator {}

//----------------------------------------------------------------------------+
// Trait Impl                                                                 |
//----------------------------------------------------------------------------+

impl Widget for Seperator {
    fn update_and_draw(
        &mut self,
        viewport: &mut Viewport,
        pos: &mut ScreenPos,
        _resized: &bool,
    ) -> Result<()> {
        if let Some(t) = &self.title {
            viewport.draw_widget(
                &Text::new(t, super::fg_color(), None),
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
