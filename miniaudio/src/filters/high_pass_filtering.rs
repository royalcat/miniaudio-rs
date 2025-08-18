use std::ptr::null;

use super::biquad_filtering::Biquad;
use super::Filter;
use crate::base::{Error, Format};
use crate::frames::{Frames, FramesMut};
use miniaudio_sys as sys;

/// Configuration for a first order high-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF1Config(sys::ma_hpf1_config);

impl HPF1Config {
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: f64,
    ) -> HPF1Config {
        HPF1Config(unsafe {
            sys::ma_hpf1_config_init(format as _, channels, sample_rate, cutoff_frequency)
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
    pub fn cutoff_frequency(&self) -> f64 {
        self.0.cutoffFrequency
    }

    #[inline]
    pub fn set_cutoff_frequency(&mut self, cutoff_frequency: f64) {
        self.0.cutoffFrequency = cutoff_frequency;
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

/// Configuration for a second order high-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF2Config(sys::ma_hpf1_config);

impl HPF2Config {
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: f64,
        q: f64,
    ) -> HPF2Config {
        HPF2Config(unsafe {
            sys::ma_hpf2_config_init(format as _, channels, sample_rate, cutoff_frequency, q)
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
    pub fn cutoff_frequency(&self) -> f64 {
        self.0.cutoffFrequency
    }

    #[inline]
    pub fn set_cutoff_frequency(&mut self, cutoff_frequency: f64) {
        self.0.cutoffFrequency = cutoff_frequency;
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

/// First order high-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF1(sys::ma_hpf1);

impl HPF1 {
    pub fn new(config: &HPF1Config) -> Result<HPF1, Error> {
        let mut hpf1 = std::mem::MaybeUninit::<HPF1>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_hpf1_init(
                config as *const HPF1Config as *const _,
                null(),
                hpf1.as_mut_ptr() as *mut _,
            ))?;
            Ok(hpf1.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &HPF1Config) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_hpf1_reinit(config as *const HPF1Config as *const _, &mut self.0)
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_hpf1_get_latency(&self.0 as *const _ as *mut _) }
    }
}

impl Filter for HPF1 {
    #[inline]
    fn process_pcm_frames(&mut self, output: &mut FramesMut, input: &Frames) -> Result<(), Error> {
        super::ensure_frames_compat(output, input)?;

        Error::from_c_result(unsafe {
            sys::ma_hpf1_process_pcm_frames(
                &mut self.0 as *mut _,
                output.as_mut_ptr() as *mut _,
                input.as_ptr() as *const _,
                output.frame_count() as u64,
            )
        })
    }
}

/// Second order high-pass filter.
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF2(sys::ma_hpf2);

impl HPF2 {
    pub fn new(config: &HPF2Config) -> Result<HPF2, Error> {
        let mut hpf2 = std::mem::MaybeUninit::<HPF2>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_hpf2_init(
                config as *const HPF2Config as *const _,
                null(),
                hpf2.as_mut_ptr() as *mut _,
            ))?;
            Ok(hpf2.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &HPF2Config) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_hpf2_reinit(config as *const HPF2Config as *const _, &mut self.0)
        })
    }

    pub fn biquad(&self) -> &Biquad {
        unsafe { (&self.0.bq as *const _ as *const Biquad).as_ref().unwrap() }
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_hpf2_get_latency(&self.0 as *const _ as *mut _) }
    }
}

impl Filter for HPF2 {
    #[inline]
    fn process_pcm_frames(&mut self, output: &mut FramesMut, input: &Frames) -> Result<(), Error> {
        super::ensure_frames_compat(output, input)?;

        Error::from_c_result(unsafe {
            sys::ma_hpf2_process_pcm_frames(
                &mut self.0 as *mut _,
                output.as_mut_ptr() as *mut _,
                input.as_ptr() as *const _,
                output.frame_count() as u64,
            )
        })
    }
}

/// Configuration for a high-pass filter with configurable order (up to 8)
#[repr(transparent)]
#[derive(Clone)]
pub struct HPFConfig(sys::ma_hpf_config);

impl HPFConfig {
    /// If order is set to 0, this will be treated as a passthrough (no filtering will be applied).
    pub fn new(
        format: Format,
        channels: u32,
        sample_rate: u32,
        cutoff_frequency: f64,
        order: u32,
    ) -> HPFConfig {
        HPFConfig(unsafe {
            sys::ma_hpf_config_init(format as _, channels, sample_rate, cutoff_frequency, order)
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
    pub fn cutoff_frequency(&self) -> f64 {
        self.0.cutoffFrequency
    }

    #[inline]
    pub fn set_cutoff_frequency(&mut self, cutoff_frequency: f64) {
        self.0.cutoffFrequency = cutoff_frequency;
    }

    #[inline]
    pub fn order(&self) -> u32 {
        self.0.order
    }

    #[inline]
    pub fn set_order(&mut self, order: u32) {
        self.0.order = order;
    }
}

/// High-pass filter with configurable order (up to 8)
#[repr(transparent)]
#[derive(Clone)]
pub struct HPF(sys::ma_hpf);

impl HPF {
    pub fn new(config: &HPFConfig) -> Result<HPF, Error> {
        let mut hpf = std::mem::MaybeUninit::<HPF>::uninit();
        unsafe {
            Error::from_c_result(sys::ma_hpf_init(
                config as *const HPFConfig as *const _,
                null(),
                hpf.as_mut_ptr() as *mut _,
            ))?;
            Ok(hpf.assume_init())
        }
    }

    pub fn reinit(&mut self, config: &HPFConfig) -> Result<(), Error> {
        Error::from_c_result(unsafe {
            sys::ma_hpf_reinit(config as *const HPFConfig as *const _, &mut self.0)
        })
    }

    pub fn latency(&self) -> u32 {
        unsafe { sys::ma_hpf_get_latency(&self.0 as *const _ as *mut _) }
    }
}

impl Filter for HPF {
    #[inline]
    fn process_pcm_frames(&mut self, output: &mut FramesMut, input: &Frames) -> Result<(), Error> {
        super::ensure_frames_compat(output, input)?;

        Error::from_c_result(unsafe {
            sys::ma_hpf_process_pcm_frames(
                &mut self.0 as *mut _,
                output.as_mut_ptr() as *mut _,
                input.as_ptr() as *const _,
                output.frame_count() as u64,
            )
        })
    }
}
