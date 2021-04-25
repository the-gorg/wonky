use std::time::Instant;

use anyhow::Result;
use serde::Deserialize;
use tinybit::{widgets::Text, Color, ScreenPos, Viewport};

use super::{CommandExt, Widget};

#[derive(Debug, Deserialize)]
pub struct Indicator {
    title: Option<String>,
    command: Vec<String>,
    frequency: u64,

    pub right: bool,
    pub bottom: bool,

    fg_color: Option<Color>,
    bg_color: Option<Color>,

    #[serde(skip_deserializing)]
    value: bool,
    #[serde(skip_deserializing)]
    reading: String,
    #[serde(skip_deserializing)]
    timer: Option<Instant>,
}

impl Indicator {
    pub fn update(&mut self) -> Result<()> {
        if self
            .timer
            .map(|t| t.elapsed().as_secs() > self.frequency)
            .unwrap_or(true)
        {
            self.timer = Some(Instant::now());

            if let Some(mut cmd) = super::construct_command(&self.command) {
                let output = cmd.get_stdout();

                self.parse_output(output);
            }
        }

        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        if let Some(output) =
            super::construct_command(&self.command).map(|mut cmd| cmd.get_stdout())
        {
            self.parse_output(output);
        }

        Ok(())
    }

    fn parse_output(&mut self, output: String) {
        let mut split = output.split(',');

        self.fg_color = Color::parse_ansi(&format!("5;{}", split.next().unwrap_or("0"))[..]);
        self.bg_color = Color::parse_ansi(&format!("5;{}", split.next().unwrap_or("2"))[..]);
        self.reading = split.collect();
    }
}

//----------------------------------------------------------------------------+
// Trait Impl                                                                 |
//----------------------------------------------------------------------------+

impl Widget for Indicator {
    fn update_and_draw(
        &mut self,
        viewport: &mut Viewport,
        pos: &mut ScreenPos,
        _resized: &bool,
    ) -> Result<()> {
        self.update()?;

        viewport.draw_widget(
            &Text::new(
                " ".repeat((viewport.size.width / 2 - 2) as usize),
                None,
                self.bg_color,
            ),
            *pos,
        );

        viewport.draw_widget(
            &Text::new(&self.reading, self.fg_color, self.bg_color),
            ScreenPos::new(pos.x, pos.y),
        );

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
