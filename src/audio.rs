use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::CODEC_TYPE_NULL;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use hound;

use std::fs::File;
use std::io::Cursor;

use crate::FitzgeraldError;
use crate::Result; //custom in lib.rs

/// Container for raw audio samples and associated metadata.
///
/// Attributes:
///     samples: 1D vector of interleaved audio samples (e.g., [L, R, L, R])
///     sample_rate: The num of samples per s
///     channels: The num of audio channels (1 for mono, 2 for stereo).
pub struct AudioData {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: usize,
}

impl AudioData {
    /// Converts interleaved multi-channel data into a single mono signal.
    ///
    /// Averages all channels at each time step into a single floating-point value.
    /// Returns a new Vec<f32> containing the mono samples.
    pub fn to_mono(&self) -> Vec<f32> {
        if self.channels == 1 {
            return self.samples.clone();
        }

        //  iter thru samples in chunks of size 'channels'
        // average them into a single value.
        self.samples
            .chunks_exact(self.channels)
            .map(|chunk| chunk.iter().sum::<f32>() / self.channels as f32)
            .collect()
    }
}

fn decode_mss(mss: MediaSourceStream) -> Result<AudioData> {
    let hint = Hint::new();
    let probed = symphonia::default::get_probe().format(
        &hint,
        mss,
        &Default::default(),
        &Default::default(),
    )?;
    let mut format_reader = probed.format;

    let track = format_reader
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("No supported audio tacks found");

    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
    let channels = track.codec_params.channels.map(|c| c.count()).unwrap_or(2);

    //make the decoder
    let decoder_opts = Default::default();
    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;

    let mut all_samples: Vec<f32> = Vec::new();

    //Decode loop
    loop {
        // Grab the next packet of compressed data
        let packet = match format_reader.next_packet() {
            Ok(packet) => packet,
            // If we reach the end of the file, break out of the loop cleanly
            Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(e) => return Err(FitzgeraldError::Symphonia(e)),
        };

        //skip if not part of track
        if packet.track_id() != track_id {
            continue;
        }

        //decode packet to audio
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // normalize to f32.
                let mut sample_buf =
                    SampleBuffer::<f32>::new(audio_buf.capacity() as u64, *audio_buf.spec());
                sample_buf.copy_interleaved_ref(audio_buf);

                all_samples.extend_from_slice(sample_buf.samples());
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(FitzgeraldError::Symphonia(e)),
        }
    }

    //everything is good
    Ok(AudioData {
        samples: all_samples,
        sample_rate,
        channels,
    })
}

pub fn load_audio_from_bytes(bytes: Vec<u8>) -> Result<AudioData> {
    let cursor = Cursor::new(bytes);
    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());
    decode_mss(mss)
}

/// Decodes an audio file from the given path into an AudioData struct.
///
/// This function supports multiple formats (MP3, WAV, FLAC, etc.) by probing
/// the file headers and decoding the first available audio track into 32-bit floats.
///
/// Args:
///     path: A string slice representing the path to the audio file.
///
/// Returns:
///     A Result containing AudioData on success, or a SymphoniaError on failure.
pub fn load_audio(path: &str) -> Result<AudioData> {
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    decode_mss(mss)
}

/// Encodes and saves audio samples to a WAV file.
///
/// Args:
///     path: The destination file path.
///     audio: A reference to the AudioData struct to be saved.
///
/// Returns:
///     A Result indicating success (Ok) or a Hound error (Err).
pub fn save_wav(path: &str, audio: &AudioData) -> Result<()> {
    let spec = hound::WavSpec {
        channels: audio.channels as u16,
        sample_rate: audio.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float, // We are using f32
    };

    let mut writer = hound::WavWriter::create(path, spec)?;
    for &sample in &audio.samples {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;
    Ok(())
}

pub fn play_audio(audio: &AudioData) -> Result<cpal::Stream> {
    #[cfg(target_arch = "wasm32")]
    {
        // forcefully resume audio context
        let audio_ctx = web_sys::AudioContext::new().map_err(|e| {
            FitzgeraldError::ValidationError(format!("AudioContext::new failed: {:?}", e))
        })?;
        let _ = audio_ctx.resume();
    }

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| FitzgeraldError::ValidationError("no output device found".into()))?;

    let config = device.default_output_config().map_err(|e| {
        FitzgeraldError::ValidationError(format!("can't get default output config: {}", e))
    })?;

    let _device_sample_rate = config.sample_rate();
    let device_channels = config.channels() as usize;
    let audio_channels = audio.channels;

    let samples = audio.samples.clone();
    let num_audio_frames = samples.len() / audio_channels;
    let mut frame_index = 0;

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for device_frame in data.chunks_mut(device_channels) {
                    for (ch, out_sample) in device_frame.iter_mut().enumerate() {
                        if frame_index < num_audio_frames {
                            let src_ch = ch % audio_channels;
                            *out_sample = samples[frame_index * audio_channels + src_ch];
                        } else {
                            *out_sample = 0.0;
                        }
                    }
                    if frame_index < num_audio_frames {
                        frame_index += 1;
                    }
                }
            },
            |err| log::error!("playback error: {}", err),
            None,
        )
        .map_err(|e| FitzgeraldError::ValidationError(e.to_string()))?;

    stream
        .play()
        .map_err(|e| FitzgeraldError::ValidationError(e.to_string()))?;

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_audio_io_roundtrip() {
        let sample_rate = 44100;
        let freq = 440.0;
        let duration = 1.0;
        let num_samples = (sample_rate as f32 * duration) as usize;

        let mut samples = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            samples.push((2.0 * PI * freq * t).sin());
        }

        let original_audio = AudioData {
            samples,
            sample_rate,
            channels: 1,
        };

        let test_file = "integration_test_temp.wav";

        save_wav(test_file, &original_audio).expect("Failed to save WAV");

        let loaded_audio = load_audio(test_file).expect("Failed to load WAV");

        assert_eq!(loaded_audio.sample_rate, original_audio.sample_rate);
        assert_eq!(loaded_audio.channels, original_audio.channels);
        assert!(
            (loaded_audio.samples.len() as i32 - original_audio.samples.len() as i32).abs() < 100
        );

        let _ = std::fs::remove_file(test_file);
    }
}
