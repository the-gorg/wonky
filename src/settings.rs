use serde::{Deserialize, Serialize};
use std::{process::Command, string, time::Instant};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Conf {
    pub meters: Vec<Meter>,
    indicators: Vec<Indicator>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Indicator {
    title: String,
    command: String,
    frequency: u64,
    left: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meter {
    pub title: String,
    pub unit: String,

    max_command: String,
    value_command: String,
    frequency: u64,

    left: bool,
    bottom: bool,

    #[serde(skip_serializing, skip_deserializing)]
    pub max_value: u64,
    #[serde(skip_serializing, skip_deserializing)]
    pub current_value: u64,

    #[serde(skip_serializing, skip_deserializing)]
    max_cmd: Option<Command>,
    #[serde(skip_serializing, skip_deserializing)]
    value_cmd: Option<Command>,
    #[serde(skip_serializing, skip_deserializing)]
    timer: Option<Instant>,
}

pub trait CommandExt {
    fn get_stdout(&mut self) -> String;
}

impl CommandExt for Command {
    fn get_stdout(&mut self) -> String {
        let output = self.output().expect("oops").stdout.to_owned();

        std::str::from_utf8(&output)
            .expect("berp")
            .trim()
            .to_string()
    }
}

impl Meter {
    pub fn update(&mut self) {
        if self.timer.is_none()
            || self.timer.unwrap().elapsed().as_secs() > self.frequency
        {
            self.timer = Some(Instant::now());
            let mut cmd =
                Self::construct_command(self.value_command.to_string())
                    .unwrap();
            self.current_value = cmd.get_stdout().parse().unwrap()
        }
    }

    pub fn init(&mut self) {
        let mut cmd =
            Self::construct_command(self.max_command.to_string()).unwrap();

        self.max_value = cmd.get_stdout().parse().unwrap();
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

        if args.len() == 0 {
            Some(command)
        } else {
            command.args(args);
            Some(command)
        }
    }

    pub fn new() -> Self {
        Self {
            title: "RAM".to_string(),
            unit: "mb".to_string(),
            max_value: 0,
            current_value: 0,
            max_command: "echo 16014".to_string(),
            value_command: "memcheck".to_string(),
            frequency: 1,
            left: true,
            bottom: false,
            timer: None,
            value_cmd: Self::construct_command("memcheck".to_string()),
            max_cmd: Self::construct_command("echo 16000".to_string()),
        }
    }
}
