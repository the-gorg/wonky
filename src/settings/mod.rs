use std::process::Command;

use anyhow::{anyhow, Context, Result};
use directories_next::ProjectDirs;
use serde::Deserialize;
use tinybit::{Color, ScreenPos, Viewport};

pub use self::meter::Meter;
use self::{indicator::Indicator, separator::Separator};

mod indicator;
mod meter;
pub mod meter_theme;
mod separator;

pub fn load() -> Result<Conf> {
    let config_file = ProjectDirs::from("github", "the-gorg", "wonky")
        .context("project directory not found")?
        .config_dir()
        .join("config.toml");

    let buf = std::fs::read(&config_file)
        .with_context(|| anyhow!("no config file found at: {}", config_file.display()))?;

    toml::from_slice(&buf).map_err(Into::into)
}

pub fn load_at_path(path: &str) -> Result<Conf> {
    let buf = std::fs::read(&path).with_context(|| anyhow!("no config file found at: {}", path))?;

    toml::from_slice(&buf).map_err(Into::into)
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub bloatie: bool,
    #[serde(default)]
    pub single_row: bool,
}

pub trait Widget {
    fn update_and_draw(
        &mut self,
        viewport: &mut Viewport,
        pos: &mut ScreenPos,
        resized: &bool,
    ) -> Result<()>;
    fn is_bottom(&self) -> bool;
    fn is_right(&self) -> bool;
    fn vertical_size(&self) -> u8;
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Element {
    Meter(Meter),
    Indicator(Indicator),
    Separator(Separator),
}

#[derive(Deserialize)]
pub struct Conf {
    pub widgets: Vec<Element>,
    pub settings: Settings,
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

//-------------------------------------------------------------------------------------
// Common
//-------------------------------------------------------------------------------------

fn construct_command(command: &[String]) -> Option<Command> {
    let mut iter = command.iter();

    let mut command = Command::new(iter.next()?);
    command.args(iter);

    Some(command)
}

fn parse_ansi(color_number: Option<u8>) -> Option<Color> {
    match color_number {
        Some(n) => Color::parse_ansi(&("5;".to_string() + &n.to_string())[..]),
        None => None,
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
