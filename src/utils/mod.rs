//! Utility functions for Octane oscillator examples

/// Generate a sine wave table
pub fn generate_sine_wave(size: usize, amplitude: i16) -> Vec<i16> {
    let mut wave = vec![0; size];
    for (i, sample) in wave.iter_mut().enumerate() {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (size as f32);
        *sample = (angle.sin() * amplitude as f32) as i16;
    }
    wave
}

/// Generate a sawtooth wave table
pub fn generate_saw_wave(size: usize, amplitude: i16) -> Vec<i16> {
    let mut wave = vec![0; size];
    for (i, sample) in wave.iter_mut().enumerate() {
        // Sawtooth from -amplitude to amplitude
        *sample = ((2 * i as i32 * amplitude as i32) / size as i32 - amplitude as i32) as i16;
    }
    wave
}

/// Generate a triangle wave table
pub fn generate_triangle_wave(size: usize, amplitude: i16) -> Vec<i16> {
    let mut wave = vec![0; size];
    let half_size = size / 2;

    for (i, sample) in wave.iter_mut().enumerate() {
        if i < half_size {
            // Rising part
            *sample = ((4 * i as i32 * amplitude as i32) / size as i32 - amplitude as i32) as i16;
        } else {
            // Falling part
            *sample = ((4 * (size - i) as i32 * amplitude as i32) / size as i32 - amplitude as i32)
                as i16;
        }
    }
    wave
}

/// Generate a square wave table
pub fn generate_square_wave(size: usize, amplitude: i16) -> Vec<i16> {
    let mut wave = vec![0; size];
    let half_size = size / 2;

    for (i, sample) in wave.iter_mut().enumerate() {
        *sample = if i < half_size { amplitude } else { -amplitude };
    }
    wave
}

/// Helper function to save samples to a WAV file
pub fn save_samples_to_wav(
    filename: &str,
    samples: &[i16],
    sample_rate: u32,
) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename)?;

    // WAV header (44 bytes)
    let data_size = (samples.len() * 2) as u32; // 16-bit samples = 2 bytes per sample
    let file_size = data_size + 36; // Total file size - 8 (for RIFF header)

    // RIFF header
    file.write_all(b"RIFF")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(b"WAVE")?;

    // fmt chunk
    file.write_all(b"fmt ")?;
    file.write_all(&[16, 0, 0, 0])?; // Chunk size: 16
    file.write_all(&[1, 0])?; // Format: 1 (PCM)
    file.write_all(&[1, 0])?; // Channels: 1 (mono)
    file.write_all(&sample_rate.to_le_bytes())?; // Sample rate
    let byte_rate = sample_rate * 2; // Sample rate * bytes per sample * channels
    file.write_all(&byte_rate.to_le_bytes())?;
    file.write_all(&[2, 0])?; // Block align: 2 bytes
    file.write_all(&[16, 0])?; // Bits per sample: 16

    // data chunk
    file.write_all(b"data")?;
    file.write_all(&data_size.to_le_bytes())?;

    // Write sample data
    for sample in samples {
        file.write_all(&sample.to_le_bytes())?;
    }

    Ok(())
}
