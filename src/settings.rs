use std::{process::Command, time::Instant};

use anyhow::{anyhow, Context, Result};
use directories_next::ProjectDirs;
use serde::Deserialize;

#[allow(unused_macros)]
macro_rules! logit {
    ($($arg:tt)*) => {
        use std::fs::OpenOptions;
        use std::io::Write;
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("/tmp/logit.txt") {
                file.write_all(format!($($arg)*).as_bytes());
        }
    };
}

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
    pub fn update(&mut self) {
        if self.timer.is_none() || self.timer.unwrap().elapsed().as_secs() > self.frequency {
            self.timer = Some(Instant::now());
            let mut cmd = construct_command(self.command.to_string()).unwrap();

            self.value = cmd.get_stdout().parse().unwrap()
        }
    }

    pub fn init(&mut self) {
        let output = construct_command(self.command.to_string())
            .unwrap()
            .get_stdout();

        let mut split = output.split(' ');

        self.value = split.next().unwrap().parse().unwrap();
        self.reading = split.collect::<String>();
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
            value_cmd: construct_command("memcheck".to_string()),
            max_cmd: construct_command("echo 16000".to_string()),
        }
    }
}

impl Meter {
    pub fn update(&mut self) {
        if self.timer.is_none() || self.timer.unwrap().elapsed().as_secs() > self.frequency {
            self.timer = Some(Instant::now());
            let mut cmd = construct_command(self.value_command.to_string()).unwrap();

            self.current_value = cmd.get_stdout().parse().unwrap()
        }
    }

    pub fn init(&mut self) {
        let mut cmd = construct_command(self.max_command.to_string()).unwrap();

        self.max_value = cmd.get_stdout().parse().unwrap();
    }

    pub fn new() -> Self {
        Self::default()
    }
}

fn construct_command(command: String) -> Option<Command> {
    let mut split = command
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .into_iter();

    let cmd = split.next()?;
    let args = split.collect::<Vec<String>>();
    let mut command = Command::new(cmd);

    if args.is_empty() {
        Some(command)
    } else {
        command.args(args);
        Some(command)
    }
}
