use hidapi;

pub struct KeyboardController {
    dev: hidapi::HidDevice,
}

type HidResult<T> = Result<T, hidapi::HidError>;

impl KeyboardController {
    const VENDOR_ID: u16 = 0x048d;
    const PRODUCT_ID: u16 = 0xce00;

    pub fn new() -> HidResult<Self> {
        let api = hidapi::HidApi::new()?;
        let dev = api.open(Self::VENDOR_ID, Self::PRODUCT_ID)?;
        Ok(KeyboardController { dev })
    }

    pub fn mono_color(
        self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        save: u8,
    ) -> HidResult<()> {
        for i in 0..4 {
            let packet = [0x14, 0x00, i + 1, red, green, blue, 0x00, 0x00];
            self.dev.send_feature_report(&packet)?;
        }
        let end_packet = [0x08, 0x02, 0x01, 0x05, brightness, 0x08, 0x00, save];
        self.dev.send_feature_report(&end_packet)
    }

    pub fn breathing(self, speed: u8, brightness: u8, save: u8) -> HidResult<()> {
        self.send_generic_packet()?;
        let packet = [0x08, 0x02, 0x02, speed, brightness, 0x08, 0x00, save];
        self.dev.send_feature_report(&packet)
    }

    pub fn wave(self, speed: u8, brightness: u8, direction: u8, save: u8) -> HidResult<()> {
        self.send_generic_packet()?;
        let packet = [0x08, 0x02, 0x03, speed, brightness, 0x08, direction, save];
        self.dev.send_feature_report(&packet)
    }

    pub fn rainbow(self, brightness: u8, save: u8) -> HidResult<()> {
        let rainbow_colors = [
            [0xff, 0x00, 0x00],
            [0x00, 0xb4, 0x00],
            [0x00, 0x00, 0xff],
            [0xff, 0x00, 0xff],
        ];
        self.send_color_packet(&rainbow_colors)?;
        let packet = [0x08, 0x02, 0x05, 0x05, brightness, 0x08, 0x00, save];
        self.dev.send_feature_report(&packet)
    }

    pub fn flash(self, speed: u8, brightness: u8, direction: u8, save: u8) -> HidResult<()> {
        self.send_generic_packet()?;
        let packet = [0x08, 0x02, 0x12, speed, brightness, 0x08, direction, save];
        self.dev.send_feature_report(&packet)
    }

    pub fn mix(self, speed: u8, brightness: u8, save: u8) -> HidResult<()> {
        self.send_generic_packet()?;
        let packet = [0x08, 0x02, 0x13, speed, brightness, 0x08, 0x00, save];
        self.dev.send_feature_report(&packet)
    }

    pub fn disable(self) -> HidResult<()> {
        let packet = [0x08, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.dev.send_feature_report(&packet)
    }

    fn send_generic_packet(&self) -> HidResult<()> {
        let generic_colors = [
            [0xff, 0x00, 0x00],
            [0xff, 0x5a, 0x00],
            [0xff, 0xb4, 0x00],
            [0x00, 0xb4, 0x00],
            [0x00, 0x00, 0xff],
            [0x00, 0xb4, 0xff],
            [0xff, 0x00, 0xff],
        ];
        self.send_color_packet(&generic_colors)
    }

    fn send_color_packet(&self, colors: &[[u8; 3]]) -> HidResult<()> {
        for (i, rgb) in colors.iter().enumerate() {
            let packet = [0x14, 0x00, i as u8 + 1, rgb[0], rgb[1], rgb[2], 0x00, 0x00];
            self.dev.send_feature_report(&packet)?;
        }
        Ok(())
    }

    pub fn parse_color(color: String) -> Result<(u8, u8, u8), ColorParseError> {
        let rgb = match color.to_lowercase().as_ref() {
            "red" => (0xff, 0x00, 0x00),
            "green" => (0x00, 0xff, 0x00),
            "blue" => (0x00, 0x00, 0xff),
            _ => {
                if color.starts_with("#") && color.len() == 7 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&color[1..3], 16),
                        u8::from_str_radix(&color[3..5], 16),
                        u8::from_str_radix(&color[5..7], 16),
                    ) {
                        return Ok((r, g, b));
                    }
                }
                return Err(ColorParseError);
            }
        };
        Ok(rgb)
    }

    pub fn parse_brightness(value: usize) -> Result<u8, BrightnessParseError> {
        let brightness = [0x00, 0x08, 0x16, 0x24, 0x32];
        if value > brightness.len() {
            return Err(BrightnessParseError);
        }
        return Ok(brightness[value]);
    }

    pub fn parse_speed(value: usize) -> Result<u8, SpeedParseError> {
        let speeds = [0x0a, 0x07, 0x05, 0x03, 0x01];
        if value > speeds.len() {
            return Err(SpeedParseError);
        }
        return Ok(speeds[value]);
    }

    pub fn parse_direction(s: String) -> Result<u8, DirectionParseError> {
        match s.as_ref() {
            "left" => Ok(0x01),
            "right" => Ok(0x02),
            _ => Err(DirectionParseError),
        }
    }

    pub fn parse_save(t: bool) -> u8 {
        if t {
            0x01
        } else {
            0x00
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ColorParseError;

#[derive(Debug, PartialEq)]
pub struct BrightnessParseError;

#[derive(Debug, PartialEq)]
pub struct SpeedParseError;

#[derive(Debug, PartialEq)]
pub struct DirectionParseError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        assert_eq!(
            KeyboardController::parse_color(String::from("red")),
            Ok((0xff, 0x00, 0x00))
        );
        assert_eq!(
            KeyboardController::parse_color(String::from("RED")),
            Ok((0xff, 0x00, 0x00))
        );
        assert_eq!(
            KeyboardController::parse_color(String::from("rainbow")),
            Err(ColorParseError)
        );
        assert_eq!(
            KeyboardController::parse_color(String::from("#ffd700")),
            Ok((0xff, 0xd7, 0x00))
        );
        assert_eq!(
            KeyboardController::parse_color(String::from("#fff")),
            Err(ColorParseError)
        );
        assert_eq!(
            KeyboardController::parse_color(String::from("#zzzzzz")),
            Err(ColorParseError)
        );
    }
}
