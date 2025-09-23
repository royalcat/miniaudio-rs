use std::{thread::sleep, time::Duration};

const DEVICE_FORMAT: miniaudio::Format = miniaudio::Format::F32;
const DEVICE_CHANNELS: u32 = 1;
const DEVICE_SAMPLE_RATE: u32 = miniaudio::StandardSampleRate::Hz48000 as u32;
const SUBBUFFER_LEN: usize = 8192;
const SUBBUFFER_COUNT: usize = 16;

pub fn main() {
    let (rb_send, rb_recv) = miniaudio::ring_buffer::<f32>(SUBBUFFER_LEN, SUBBUFFER_COUNT)
        .expect("Failed to create ring buffer");

    let mut capture_config = miniaudio::DeviceConfig::new(miniaudio::DeviceType::Capture);
    capture_config.capture_mut().set_format(DEVICE_FORMAT);
    capture_config.capture_mut().set_channels(DEVICE_CHANNELS);
    capture_config.set_sample_rate(DEVICE_SAMPLE_RATE);
    // capture_config.set_no_clip(true);

    capture_config.set_data_callback(move |_, _, input| {
        let samples = input.as_samples::<f32>();

        let mut written_count = 0;
        while written_count < samples.len() {
            written_count += rb_send.write(&samples[written_count..]);
        }
    });

    let mut playback_config = miniaudio::DeviceConfig::new(miniaudio::DeviceType::Playback);
    playback_config.playback_mut().set_format(DEVICE_FORMAT);
    playback_config.playback_mut().set_channels(DEVICE_CHANNELS);
    playback_config.set_sample_rate(DEVICE_SAMPLE_RATE);

    // playback_config.set_no_clip(true);

    playback_config.set_data_callback(move |_, output, _| {
        let samples = output.as_samples_mut::<f32>();
        // Here we try reading at most 8 subbuffers to attempt to read enough samples to
        // fill the playback output buffer. We don't allow infinite attempts because we can't be
        // sure how long that would take.
        let mut read_count = 0;
        let mut attempts = 0;
        while read_count < samples.len() && attempts < 10 {
            read_count += rb_recv.read(&mut samples[read_count..]);
            attempts += 1;
        }

        // If we didn't read enough samples, fill the rest with silence.
        if read_count < samples.len() {
            let remaining = &mut samples[read_count..];
            remaining.fill(0.0);
        }
    });

    let capture_device =
        miniaudio::Device::new(None, &capture_config).expect("Failed to create capture device");

    let playback_device =
        miniaudio::Device::new(None, &playback_config).expect("Failed to create playback device");

    capture_device
        .start()
        .expect("Failed to start capture device");
    // sleep(Duration::from_millis(20));
    playback_device
        .start()
        .expect("Failed to start playback device");

    sleep(Duration::from_secs(15));

    capture_device
        .stop()
        .expect("Failed to stop capture device");
    playback_device
        .stop()
        .expect("Failed to stop playback device");
}
