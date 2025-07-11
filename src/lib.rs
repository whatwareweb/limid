mod synth;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use web_sys::console;

use cpal::{SizedSample, FromSample};

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // provides better error messages in debug
    // disabled in release mode as to not bloat the file size
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Handle(Stream);

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn tone() -> Handle {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("failed to find output device");
    let config = device.default_output_config().unwrap();
    Handle(match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        _ => panic!("sample format unsupported"),
    })
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Stream
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // produce fm sin wave (wow fancy)
        let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        synth::fm(sample_clock, sample_rate, 440.0)
    };

    #[cfg(feature = "native")]
    let err_fn = |err| eprintln!("{}", &format!("an error occurred on stream: {}", err));

    #[cfg(not(feature = "native"))]
    let err_fn = |err| console::error_1(&format!("an error occurred on stream: {}", err).into());


    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _| write_data(data, channels, &mut next_value),
            err_fn,
            None,
        )
        .unwrap();
    stream.play().unwrap();
    stream
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: SizedSample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}