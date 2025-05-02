use octane::utils::{generate_saw_wave, generate_sine_wave, save_samples_to_wav};
use octane::{octane_synthesize, CombineMethod, OctaneOscillatorState};

use std::thread::sleep;
use std::time::Duration;

// This example shows how Octane might be used in a real-time synthesizer application

fn main() {
    println!("Octane Synthesizer Application Example");
    println!("=====================================");

    // Create waveform tables
    let sine_wave = generate_sine_wave(1024, 512);
    let saw_wave = generate_saw_wave(1024, 512);

    // Audio settings
    let sample_rate = 44100;
    let buffer_size = 256; // Small buffer for low latency
    let duration_seconds = 10.0;
    let total_samples = (sample_rate as f32 * duration_seconds) as usize;
    let num_buffers = total_samples / buffer_size;

    // Create output buffer
    let mut output_samples = vec![0i16; total_samples];

    // Initialize oscillators
    let mut osc_a = OctaneOscillatorState::new();
    let mut osc_b = OctaneOscillatorState::new();

    // Initial frequencies
    osc_a.update_phase_delta(440.0, sample_rate); // A4
    osc_b.update_phase_delta(220.0, sample_rate); // A3

    println!("Simulating real-time audio processing...");

    // Simulate parameter changes over time
    for buffer_idx in 0..num_buffers {
        // Update parameters based on time
        let time_position = buffer_idx as f32 / num_buffers as f32;

        // Simulate frequency modulation over time
        let freq_a = 440.0 + 100.0 * (time_position * 2.0 * std::f32::consts::PI).sin();
        let freq_b = 220.0 + 50.0 * (time_position * 4.0 * std::f32::consts::PI).sin();

        osc_a.update_phase_delta(freq_a, sample_rate);
        osc_b.update_phase_delta(freq_b, sample_rate);

        // Simulate alpha parameter modulation
        let alpha = 0.5 + 0.5 * (time_position * 3.0 * std::f32::consts::PI).sin();

        // Determine combine method based on time
        let method = if time_position < 0.33 {
            CombineMethod::Fade
        } else if time_position < 0.66 {
            CombineMethod::Phase
        } else {
            CombineMethod::Mult
        };

        // Process current buffer
        let buffer_start = buffer_idx * buffer_size;
        let buffer_end = buffer_start + buffer_size;

        for i in buffer_start..buffer_end {
            if i < total_samples {
                output_samples[i] = octane_synthesize(
                    &mut osc_a,
                    &mut osc_b,
                    &sine_wave,
                    &saw_wave,
                    method,
                    alpha,
                    time_position > 0.5, // Enable sync in second half
                );
            }
        }

        // Simulate real-time processing delay
        if buffer_idx % 10 == 0 {
            println!(
                "Processing buffer {}/{} - Method: {:?}, Alpha: {:.2}",
                buffer_idx, num_buffers, method, alpha
            );
        }

        // Small sleep to simulate real-time processing
        sleep(Duration::from_millis(1));
    }

    // Save the result
    let output_file = "octane_synth_app_demo.wav";
    match save_samples_to_wav(output_file, &output_samples, sample_rate) {
        Ok(_) => println!("Successfully saved to {}", output_file),
        Err(e) => println!("Error saving WAV file: {}", e),
    }

    println!("Simulation completed!");
}
