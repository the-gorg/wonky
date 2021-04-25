use std::{path::PathBuf, process::Command, str::FromStr, time::Instant};

use anyhow::{anyhow, Context, Result};
use directories_next::ProjectDirs;
use serde::Deserialize;
use tinybit::{widgets::Text, Color};
use tinybit::{ScreenPos, Viewport};

use crate::MeterTheme;

pub fn load() -> Result<Conf> {
    let config_file = ProjectDirs::from("github", "the-gorg", "wonky")
        .context("project directory not found")?
        .config_dir()
        .join("config.toml");

    let buf = std::fs::read(&config_file).with_context(|| {
        anyhow!("no config file found at: {}", config_file.display())
    })?;

    toml::from_slice(&buf).map_err(Into::into)
}

pub fn load_at_path(path: &str) -> Result<Conf> {
    let test = PathBuf::from_str(path);
    let buf = std::fs::read(&path)
        .with_context(|| anyhow!("no config file found at: {}", path))?;

    toml::from_slice(&buf).map_err(Into::into)
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub bloatie: bool,
}

pub trait Widget {
    fn update_and_draw(
        &mut self,
        viewport: &mut Viewport,
        pos: &mut ScreenPos,
    ) -> Result<()>;
    fn is_bottom(&self) -> bool;
    fn is_right(&self) -> bool;
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Element {
    Meter(Meter),
    Indicator(Indicator),
    Seperator(Seperator),
}

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub widgets: Vec<Element>,
    pub settings: Settings,
}

#[derive(Debug, Deserialize)]
pub struct Seperator {
    pub title: Option<String>,
    pub right: bool,
    pub bottom: bool,
}

impl Seperator {}

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

            if let Some(mut cmd) = construct_command(&self.command) {
                let output = cmd.get_stdout();

                self.parse_output(output)?;
            }
        }

        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        if let Some(output) =
            construct_command(&self.command).map(|mut cmd| cmd.get_stdout())
        {
            self.parse_output(output)?;
        }

        Ok(())
    }

    fn parse_output(&mut self, output: String) -> Result<()> {
        let mut split = output.split(",").into_iter();

        self.fg_color = Color::parse_ansi(
            &format!("5;{}", split.next().unwrap_or("0"))[..],
        );
        self.bg_color = Color::parse_ansi(
            &format!("5;{}", split.next().unwrap_or("2"))[..],
        );
        self.reading = split.collect();

        Ok(())
    }
}

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

    pub theme: usize,

    #[serde(skip_deserializing)]
    pub max_value: u64,
    #[serde(skip_deserializing)]
    pub current_value: u64,

    #[serde(skip_deserializing)]
    pub meter_theme: MeterTheme,

    #[serde(skip_deserializing)]
    max_cmd: Option<Command>,
    #[serde(skip_deserializing)]
    value_cmd: Option<Command>,
    #[serde(skip_deserializing)]
    timer: Option<Instant>,
}

pub trait CommandExt {
    fn get_stdout(&mut self) -> String;
}

impl CommandExt for Command {
    fn get_stdout(&mut self) -> String {
        let output = self.output().expect("oops").stdout;

        std::str::from_utf8(&output)
            .expect("berp")
            .trim()
            .to_string()
    }
}

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
            value_cmd: construct_command(&vec!["memcheck".to_string()]),
            max_cmd: construct_command(&vec!["echo 16000".to_string()]),
            prefix: None,
            theme: 1,
            right: true,
            bottom: false,
            meter: true,
            reading: true,
            meter_theme: MeterTheme::default(0),
        }
    }
}

impl Meter {
    pub fn update(&mut self) -> Result<()> {
        if self
            .timer
            .map(|t| t.elapsed().as_secs() > self.frequency)
            .unwrap_or(true)
        {
            self.timer = Some(Instant::now());

            if let Some(mut cmd) = construct_command(&self.value_command) {
                self.current_value = cmd.get_stdout().parse()?;
            }
        }

        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        if let Some(mut cmd) = construct_command(&self.max_command) {
            self.max_value = cmd.get_stdout().parse()?;
        }

        Ok(())
    }

    pub fn set_theme(&mut self, theme: MeterTheme) {
        self.meter_theme = theme;
    }

    pub fn new() -> Self {
        Self::default()
    }
}

//----------------------------------------------------------------------------+
// Drawing                                                                    |
//----------------------------------------------------------------------------+

impl Widget for Meter {
    fn update_and_draw(
        &mut self,
        viewport: &mut Viewport,
        pos: &mut ScreenPos,
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
                fg_color(),
                None,
            );

            viewport.draw_widget(
                &value_reading,
                ScreenPos::new(
                    // TODO: why 2?!?
                    pos.x
                        + (viewport.size.width / 2
                            - 2
                            - value_reading.0.len() as u16),
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
                &Text::new(t, fg_color(), None),
                ScreenPos::new(pos.x, pos.y),
            );
        };

        self.meter_theme.draw(
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
}

impl Widget for Indicator {
    fn update_and_draw(
        &mut self,
        viewport: &mut Viewport,
        pos: &mut ScreenPos,
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
}

impl Widget for Seperator {
    fn update_and_draw(
        &mut self,
        viewport: &mut Viewport,
        pos: &mut ScreenPos,
    ) -> Result<()> {
        if let Some(t) = &self.title {
            viewport.draw_widget(
                &Text::new(t, fg_color(), None),
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
}

//-------------------------------------------------------------------------------------
// Common
//-------------------------------------------------------------------------------------

fn construct_command(command: &Vec<String>) -> Option<Command> {
    let mut iter = command.iter();

    let mut command = Command::new(iter.next()?);
    command.args(iter);

    Some(command)
}

#[allow(dead_code, clippy::unnecessary_wraps)]
fn fg_color() -> Option<Color> {
    Some(Color::Green)
}

#[allow(dead_code, clippy::unnecessary_wraps)]
fn bg_color() -> Option<Color> {
    Some(Color::DarkGreen)
}
