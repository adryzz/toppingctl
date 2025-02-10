use clap::{ArgAction, Parser};
use device::E2x2;
mod device;

fn main() {
    let args = Property::parse();
    let api = hidapi::HidApi::new().unwrap();
    let mut device = E2x2::new(api).unwrap();

    let res = match args {
        Property::Brightness { value } => device.brightness(value as usize),
        Property::AutoStandby { state } => device.auto_standby(state),
        Property::HeadphoneGain { state } => device.headphones_gain(state),
        Property::HeadphoneOut { state } => device.headphones_out(state),
        Property::LineOut { state } => device.line_out(state),
        Property::InputMonitor { channel, state } => device.input_monitor(channel as usize, state),
        Property::InputPhantomPower { channel, state } => device.input_48v(channel as usize, state),
        Property::InputImpedance { channel, state } => device.input_inst(channel as usize, state),
        Property::InputMute { channel, state } => device.input_mute(channel as usize, state),
    };
    if let Err(e) = res {
        eprintln!("HID error: {e}");
        std::process::exit(-1);
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[derive(Debug, Copy, Clone)]
enum Property {
    
    Brightness {
        #[arg(required(true), value_parser(clap::value_parser!(u8).range(0..3)))]
        value: u8,
    },
    AutoStandby {
        #[arg(required(true), action(ArgAction::Set), value_parser(clap::builder::BoolishValueParser::new()))]
        state: bool,
    },
    HeadphoneGain {
        #[arg(required(true), action(ArgAction::Set), value_parser(clap::builder::BoolishValueParser::new()))]
        state: bool,
    },
    HeadphoneOut {
        #[arg(required(true), action(ArgAction::Set), value_parser(clap::builder::BoolishValueParser::new()))]
        state: bool,
    },
    LineOut {
        #[arg(required(true), action(ArgAction::Set), value_parser(clap::builder::BoolishValueParser::new()))]
        state: bool,
    },
    InputMonitor {
        #[arg(required(true))]
        channel: u8,
        #[arg(required(true), action(ArgAction::Set), value_parser(clap::builder::BoolishValueParser::new()))]
        state: bool,
    },
    InputPhantomPower {
        #[arg(required(true))]
        channel: u8,
        #[arg(required(true), action(ArgAction::Set), value_parser(clap::builder::BoolishValueParser::new()))]
        state: bool,
    },
    InputImpedance {
        #[arg(required(true))]
        channel: u8,
        #[arg(required(true), action(ArgAction::Set), value_parser(clap::builder::BoolishValueParser::new()))]
        state: bool,
    },
    InputMute {
        #[arg(required(true))]
        channel: u8,
        #[arg(required(true), action(ArgAction::Set), value_parser(clap::builder::BoolishValueParser::new()))]
        state: bool,
    },
}