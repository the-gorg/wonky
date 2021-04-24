use std::{process::Command, time::Instant};

use anyhow::{anyhow, Context, Result};
use directories_next::ProjectDirs;
use serde::Deserialize;

pub fn load() -> Result<Conf> {
    let config_file = ProjectDirs::from("github", "the-gorg", "thingy")
        .context("project directory not found")?
        .config_dir()
        .join("config.toml");
    let buf = std::fs::read(&config_file)
        .with_context(|| anyhow!("no config file found at: {}", config_file.display()))?;

    toml::from_slice(&buf).map_err(Into::into)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Widget {
    Meter(Meter),
    Indicator(Indicator),
}

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub widgets: Vec<Widget>,
}

#[derive(Debug, Deserialize)]
pub struct Indicator {
    title: String,
    command: String,
    frequency: u64,

    pub right: bool,
    pub bottom: bool,

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
                self.value = cmd.get_stdout().parse()?;
            }
        }

        Ok(())
    }

    pub fn init(&mut self) -> Result<()> {
        if let Some(output) = construct_command(&self.command).map(|mut cmd| cmd.get_stdout()) {
            let mut split = output.split(' ');

            if let Some(value) = split.next() {
                self.value = value.parse()?;
                self.reading = split.collect();
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Meter {
    pub title: String,
    pub unit: String,

    max_command: String,
    value_command: String,
    frequency: u64,

    pub right: bool,
    pub bottom: bool,

    pub theme: usize,
    pub prefix: Option<String>,
    #[serde(skip_deserializing)]
    pub max_value: u64,
    #[serde(skip_deserializing)]
    pub current_value: u64,

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
            title: "RAM".to_string(),
            unit: "mb".to_string(),
            max_value: 0,
            current_value: 0,
            max_command: "echo 16014".to_string(),
            value_command: "memcheck".to_string(),
            frequency: 1,
            right: true,
            bottom: false,
            timer: None,
            value_cmd: construct_command("memcheck"),
            max_cmd: construct_command("echo 16000"),
            prefix: None,
            theme: 1,
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

    pub fn new() -> Self {
        Self::default()
    }
}

fn construct_command(command: &str) -> Option<Command> {
    let mut split = command.split_whitespace();
    let cmd = split.next()?;

    let mut command = Command::new(cmd);
    command.args(split);

    Some(command)
}
