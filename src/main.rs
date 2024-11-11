use device::E2x2;

mod device;

fn main() {
    let api = hidapi::HidApi::new().unwrap();
    let mut device = E2x2::new(api).unwrap();
    device.input_48v(0, false);
    std::thread::sleep(std::time::Duration::from_secs(2));
    device.input_48v(0, true);
}
