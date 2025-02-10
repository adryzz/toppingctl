use device::E2x2;
use clap::Parser;
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
    Brightness{value: u8},
    AutoStandby{state: bool},
    HeadphoneGain{state: bool},
    HeadphoneOut{state: bool},
    LineOut{state: bool},
    InputMonitor{channel: u8, state: bool},
    InputPhantomPower{channel: u8, state: bool},
    InputImpedance{channel: u8, state: bool},
    InputMute{channel: u8, state: bool},
}