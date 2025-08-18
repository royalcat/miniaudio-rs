use std::ptr::null;

use super::biquad_filtering::Biquad;
use super::Filter;
use crate::base::{Error, Format};
use crate::frames::{Frames, FramesMut};
use miniaudio_sys as sys;

/// Configuration for a second order peaking filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct Peak2Config(sys::ma_peak2_config);

impl Peak2Config {
    #[inline]
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        gain_db: f64,
        q: f64,
        frequency: f64,
    ) -> Peak2Config {
        Peak2Config(unsafe {
            sys::ma_peak2_config_init(format as _, channels, sample_rate, gain_db, q, frequency)
        })
    }

    #[inline]
    pub fn format(&self) -> Format {
        Format::from_c(self.0.format)
    }

    #[inline]
    pub fn set_format(&mut self, format: Format) {
        self.0.format = format as _;
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    #[inline]
    pub fn set_channels(&mut self, channels: u32) {
        self.0.channels = channels;
    }

    #[inline]
    pub fn sample_rate(&self) -> u32 {
        self.0.sampleRate
    }

    #[inline]
    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.0.sampleRate = sample_rate;
    }

    #[inline]
    pub fn frequency(&self) -> f64 {
        self.0.frequency
    }

    #[inline]
    pub fn set_frequency(&mut self, frequency: f64) {
        self.0.frequency = frequency;
    }

    #[inline]
    pub fn gain_db(&self) -> f64 {
        self.0.gainDB
    }

    #[inline]
    pub fn set_gain_db(&mut self, gain_db: f64) {
        self.0.gainDB = gain_db;
    }

    #[inline]
    pub fn q(&self) -> f64 {
        self.0.q
    }

    #[inline]
    pub fn set_q(&mut self, q: f64) {
        self.0.q = q;
    }
}

/// Second order peaking filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct Peak2(sys::ma_peak2);

impl Peak2 {
    #[inline]
    pub fn new(config: &Peak2Config) -> Result<Peak2, Error> {
        let mut peak2 = std::mem::MaybeUninit::<Peak2>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_peak2_init(
                config as *const Peak2Config as *const _,
                null(),
                peak2.as_mut_ptr() as *mut _,
            ))?;
            Ok(peak2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &Peak2Config) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_peak2_reinit(config as *const Peak2Config as *const _, &mut self.0)
        })
    }

    #[inline]
    pub fn bq(&self) -> &Biquad {
        unsafe { &*(&self.0.bq as *const sys::ma_biquad as *const Biquad) }
    }

    #[inline]
    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_peak2_get_latency(&self.0 as *const _ as *mut _) }
    }
}

impl Filter for Peak2 {
    #[inline]
    fn process_pcm_frames(&mut self, output: &mut FramesMut, input: &Frames) -> Result<(), Error> {
        super::ensure_frames_compat(output, input)?;

        Error::from_c_result(unsafe {
            sys::ma_peak2_process_pcm_frames(
                &mut self.0 as *mut _,
                output.as_mut_ptr() as *mut _,
                input.as_ptr() as *const _,
                output.frame_count() as u64,
            )
        })
    }
}
