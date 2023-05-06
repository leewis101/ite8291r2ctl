use clap::Parser;

use cli::Effects;
use kbd::KeyboardController;

mod cli;
mod kbd;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let controller = KeyboardController::new()?;
    let args = cli::Arg::parse();
    match args.effect {
        Effects::Mono => {
            let rgb = args.color.unwrap();
            controller.mono(
                rgb.0,
                rgb.1,
                rgb.2,
                args.brightness.value(),
                args.save as u8,
            )?;
        }
        Effects::Breath => {
            controller.breath(
                args.speed.unwrap().value(),
                args.brightness.value(),
                args.save as u8,
            )?;
        }
        Effects::Wave => {
            controller.wave(
                args.speed.unwrap().value(),
                args.direction.unwrap().value(),
                args.brightness.value(),
                args.save as u8,
            )?;
        }
        Effects::Rainbow => {
            controller.rainbow(args.brightness.value(), args.save as u8)?;
        }
        Effects::Flash => {
            controller.flash(
                args.speed.unwrap().value(),
                args.direction.unwrap().value(),
                args.brightness.value(),
                args.save as u8,
            )?;
        }
        Effects::Mix => {
            controller.mix(
                args.speed.unwrap().value(),
                args.brightness.value(),
                args.save as u8,
            )?;
        }
        Effects::Disable => {
            controller.disable()?;
        }
    }
    Ok(())
}
