use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, StreamConfig};

pub fn get_default_output_device() -> Result<(Device, StreamConfig), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let supported_config = device.default_output_config().expect("no supported config");
    let config = supported_config.config();
    Ok((device, config))
}
