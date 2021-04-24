use ite8291r2_ctl::KeyboardController;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ite8921r2_ctl")]
struct Opt {
    /// Keyboard backlight effect: monocolor, breathing, wave, rainbow, flash, mix, disable
    #[structopt(short, long)]
    effect: String,

    /// Save the settings
    #[structopt(short = "S", long)]
    save: bool,

    /// Brightness of the effect: 0-5
    #[structopt(short, long, default_value = "2")]
    brightness: usize,

    /// Color of the effect
    #[structopt(short, long, required_if("effect", "monocolor"))]
    color: Option<String>,

    /// Direction of the effect: left, right
    #[structopt(
        short,
        long,
        required_if("effect", "wave"),
        required_if("effect", "flash")
    )]
    direction: Option<String>,

    /// Speed of the effect: 0-5
    #[structopt(
        short,
        long,
        required_if("effect", "breathing"),
        required_if("effect", "wave"),
        required_if("effect", "flash"),
        required_if("effect", "mix")
    )]
    speed: Option<usize>,
}

fn main() {
    let controller = KeyboardController::new().unwrap();

    let opt = Opt::from_args();

    let save = KeyboardController::parse_save(opt.save);
    let brightness = KeyboardController::parse_brightness(opt.brightness).unwrap();

    match opt.effect.as_ref() {
        "monocolor" => {
            let (r, g, b) = KeyboardController::parse_color(opt.color.unwrap()).unwrap();
            controller.mono_color(r, g, b, brightness, save).unwrap();
        }
        "breathing" => {
            let speed = KeyboardController::parse_speed(opt.speed.unwrap()).unwrap();
            controller.breathing(speed, brightness, save).unwrap();
        }
        "wave" => {
            let speed = KeyboardController::parse_speed(opt.speed.unwrap()).unwrap();
            let direction = KeyboardController::parse_direction(opt.direction.unwrap()).unwrap();
            controller.wave(speed, brightness, direction, save).unwrap();
        }
        "rainbow" => {
            controller.rainbow(brightness, save).unwrap();
        }
        "flash" => {
            let speed = KeyboardController::parse_speed(opt.speed.unwrap()).unwrap();
            let direction = KeyboardController::parse_direction(opt.direction.unwrap()).unwrap();
            controller
                .flash(speed, brightness, direction, save)
                .unwrap();
        }
        "mix" => {
            let speed = KeyboardController::parse_speed(opt.speed.unwrap()).unwrap();
            controller.mix(speed, brightness, save).unwrap();
        }
        "disable" => {
            controller.disable().unwrap();
        }
        _ => {
            println!("no such effect")
        }
    };
}
