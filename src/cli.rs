use std::fmt;
use std::str::FromStr;

use clap::{Parser, ValueEnum};
use thiserror::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    /// keyboard backlight effect
    #[arg(short, long, value_enum)]
    pub effect: Effects,
    /// save settings
    #[arg(short = 'S', long, default_value_t = false)]
    pub save: bool,
    /// keyboard backlight brightness
    #[arg(short,
        long,
        value_parser = BrightnessParser,
        default_value = "3",
    )]
    pub brightness: Brightness,
    /// keybarod backlight color hex code, e.g. #ff0000
    #[arg(
        short,
        long,
        value_parser = Color::from_str,
        required = false,
        required_if_eq("effect", "mono"),
    )]
    pub color: Option<Color>,
    /// keybarod backlight moving direction
    #[arg(
        short,
        long,
        value_enum,
        required = false,
        required_if_eq("effect", "wave"),
        required_if_eq("effect", "flash")
    )]
    pub direction: Option<Direction>,
    /// keybarod backlight moving speed
    #[arg(
        short,
        long,
        value_parser = SpeedParser,
        required = false,
        required_if_eq("effect", "breath"),
        required_if_eq("effect", "wave"),
        required_if_eq("effect", "flash"),
        required_if_eq("effect", "mix"),
    )]
    pub speed: Option<Speed>,
}

#[derive(Copy, Clone, ValueEnum)]
pub enum Effects {
    Mono,
    Breath,
    Wave,
    Rainbow,
    Flash,
    Mix,
    Disable,
}

#[derive(Clone, Copy)]
pub struct Brightness {
    index: usize,
}

impl Brightness {
    const VALUES: &'static [u8] = &[0x00, 0x08, 0x16, 0x24, 0x32];

    pub fn value(&self) -> u8 {
        Self::VALUES[self.index]
    }
}

impl fmt::Display for Brightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.value())
    }
}

#[derive(Clone)]
struct BrightnessParser;

impl clap::builder::TypedValueParser for BrightnessParser {
    type Value = Brightness;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u8).range(0i64..Brightness::VALUES.len() as i64);
        let val = inner.parse_ref(cmd, arg, value)?;
        Ok(Brightness {
            index: val as usize,
        })
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::builder::PossibleValue> + '_>> {
        Some(Box::new(Brightness::VALUES.iter().enumerate().map(
            |(i, _)| clap::builder::PossibleValue::new(i.to_string()),
        )))
    }
}

#[derive(Clone, Copy)]
pub struct Speed {
    index: usize,
}

impl Speed {
    const VALUES: &'static [u8] = &[0x0a, 0x07, 0x05, 0x03, 0x01];

    pub fn value(&self) -> u8 {
        Self::VALUES[self.index]
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.value())
    }
}

#[derive(Clone)]
struct SpeedParser;

impl clap::builder::TypedValueParser for SpeedParser {
    type Value = Speed;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u8).range(1i64..=Speed::VALUES.len() as i64);
        let val = inner.parse_ref(cmd, arg, value)?;
        Ok(Speed {
            index: (val - 1) as usize,
        })
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::builder::PossibleValue> + '_>> {
        Some(Box::new(Speed::VALUES.iter().enumerate().map(|(i, _)| {
            clap::builder::PossibleValue::new((i + 1).to_string())
        })))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

#[derive(Error, Debug)]
#[error("invalid color")]
pub struct InvalidColor;

impl std::str::FromStr for Color {
    type Err = InvalidColor;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = s.to_lowercase();
        if color.starts_with("#") {
            if color.len() != 7 {
                return Err(InvalidColor);
            }
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&color[1..3], 16),
                u8::from_str_radix(&color[3..5], 16),
                u8::from_str_radix(&color[5..7], 16),
            ) {
                return Ok(Color(r, g, b));
            } else {
                return Err(InvalidColor);
            }
        }
        match color.as_str() {
            "red" => Ok(Color(0xff, 0x00, 0x00)),
            "green" => Ok(Color(0x00, 0xff, 0x00)),
            "blue" => Ok(Color(0x00, 0x00, 0xff)),
            _ => Err(InvalidColor),
        }
    }
}

#[derive(Copy, Clone, ValueEnum)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn value(&self) -> u8 {
        match self {
            Self::Left => 0x01,
            Self::Right => 0x02,
        }
    }
}
