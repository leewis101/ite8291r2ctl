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
        Ok(Self { dev })
    }

    pub fn mono(self, red: u8, green: u8, blue: u8, brightness: u8, save: u8) -> HidResult<()> {
        for i in 0..4 {
            let packet = [0x14, 0x00, i + 1, red, green, blue, 0x00, 0x00];
            self.dev.send_feature_report(&packet)?;
        }
        let end_packet = [0x08, 0x02, 0x01, 0x05, brightness, 0x08, 0x00, save];
        self.dev.send_feature_report(&end_packet)
    }

    pub fn breath(self, speed: u8, brightness: u8, save: u8) -> HidResult<()> {
        self.send_generic_packet()?;
        let packet = [0x08, 0x02, 0x02, speed, brightness, 0x08, 0x00, save];
        self.dev.send_feature_report(&packet)
    }

    pub fn wave(self, speed: u8, direction: u8, brightness: u8, save: u8) -> HidResult<()> {
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

    pub fn flash(self, speed: u8, direction: u8, brightness: u8, save: u8) -> HidResult<()> {
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
}
