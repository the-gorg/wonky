use std::{process::Command, time::Instant};

use anyhow::Result;
use serde::Deserialize;
use tinybit::{widgets::Text, ScreenPos, Viewport};

use crate::MeterTheme;

use super::{CommandExt, Widget};

#[derive(Debug, Deserialize)]
pub struct Meter {
    pub title: Option<String>,
    pub unit: Option<String>,
    pub prefix: Option<String>,

    max_command: Vec<String>,
    value_command: Vec<String>,
    frequency: u64,

    pub right: bool,
    pub bottom: bool,

    pub meter: bool,
    pub reading: bool,

    #[serde(skip_deserializing)]
    pub max_value: u64,
    #[serde(skip_deserializing)]
    pub current_value: u64,

    #[serde(default)]
    pub theme: MeterTheme,

    #[serde(skip_deserializing)]
    max_cmd: Option<Command>,
    #[serde(skip_deserializing)]
    value_cmd: Option<Command>,
    #[serde(skip_deserializing)]
    timer: Option<Instant>,
}

impl Meter {
    pub fn update(&mut self) -> Result<()> {
        if self
            .timer
            .map(|t| t.elapsed().as_secs() > self.frequency)
            .unwrap_or(true)
        {
            self.timer = Some(Instant::now());

            if let Some(mut cmd) = super::construct_command(&self.value_command) {
                self.current_value = cmd.get_stdout().parse()?;
            }
        }

        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        if let Some(mut cmd) = super::construct_command(&self.max_command) {
            self.max_value = cmd.get_stdout().parse()?;
        }

        Ok(())
    }

    pub fn set_theme(&mut self, theme: MeterTheme) {
        self.theme = theme;
    }

    pub fn new() -> Self {
        Self::default()
    }
}

//----------------------------------------------------------------------------+
// Trait Impl                                                                 |
//----------------------------------------------------------------------------+

impl Default for Meter {
    fn default() -> Self {
        Self {
            title: Some("RAM".to_string()),
            unit: Some("mb".to_string()),
            max_value: 0,
            current_value: 0,
            max_command: vec!["echo 16014".to_string()],
            value_command: vec!["memcheck".to_string()],
            frequency: 1,
            timer: None,
            value_cmd: super::construct_command(&["memcheck".to_string()]),
            max_cmd: super::construct_command(&["echo 16000".to_string()]),
            prefix: None,
            right: true,
            bottom: false,
            meter: true,
            reading: true,
            theme: MeterTheme::default(0),
        }
    }
}

impl Widget for Meter {
    fn update_and_draw(
        &mut self,
        viewport: &mut Viewport,
        pos: &mut ScreenPos,
        resized: &bool,
    ) -> Result<()> {
        self.update()?;

        // Offset one up if bottom aligned
        if self.reading || self.title.is_some() {
            pos.y = if self.bottom { pos.y - 1 } else { pos.y };
        }

        if self.reading {
            let unit = match &self.unit {
                Some(u) => u.clone(),
                None => "".to_string(),
            };

            let value_reading = Text::new(
                format!("{}/{}{}", self.current_value, self.max_value, unit),
                super::fg_color(),
                None,
            );

            viewport.draw_widget(
                &value_reading,
                ScreenPos::new(
                    // TODO: why 2?!?
                    pos.x + (viewport.size.width / 2 - 2 - value_reading.0.len() as u16),
                    pos.y,
                ),
            );
        }

        // if we have a title or reading offset bar by 1
        let bar_offset = if self.title.is_some() || self.reading {
            1_u16
        } else {
            0_u16
        };

        if let Some(t) = &self.title {
            viewport.draw_widget(
                &Text::new(t, super::fg_color(), None),
                ScreenPos::new(pos.x, pos.y),
            );
        };

        if *resized {
            self.theme.resize((viewport.size.width / 2 - 2) as u8)
        };

        self.theme.draw(
            viewport,
            self,
            (self.current_value as f32, self.max_value as f32),
            ScreenPos::new(pos.x, pos.y + bar_offset),
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
        if self.reading || self.title.is_some() {
            2
        } else {
            1
        }
    }
}
