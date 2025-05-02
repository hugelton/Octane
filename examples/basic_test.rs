use octane::utils::{
    generate_saw_wave, generate_sine_wave, generate_square_wave, generate_triangle_wave,
    save_samples_to_wav,
};
use octane::{octane_synthesize, CombineMethod, OctaneOscillatorState};

fn main() {
    // Create waveform tables
    let sine_wave = generate_sine_wave(1024, 512);
    let saw_wave = generate_saw_wave(1024, 512);
    let triangle_wave = generate_triangle_wave(1024, 512);
    let square_wave = generate_square_wave(1024, 512);

    println!("Octane Oscillator Test");
    println!("=====================");

    // Test parameters
    let sample_rate = 44100;
    let duration_seconds = 5.0;
    let num_samples = (sample_rate as f32 * duration_seconds) as usize;

    // Create a buffer for output samples
    let mut output_samples = vec![0i16; num_samples];

    // Test all combine methods
    let methods = [
        (CombineMethod::Fade, "fade"),
        (CombineMethod::Phase, "phase"),
        (CombineMethod::Mult, "mult"),
    ];

    for (method, name) in methods.iter() {
        // Initialize oscillators
        let mut osc_a = OctaneOscillatorState::new();
        let mut osc_b = OctaneOscillatorState::new();

        // Set frequencies (A4 = 440Hz, A3 = 220Hz)
        osc_a.update_phase_delta(440.0, sample_rate);
        osc_b.update_phase_delta(220.0, sample_rate);

        // Generate samples
        println!("Generating samples with {} method...", name);
        for i in 0..num_samples {
            output_samples[i] = octane_synthesize(
                &mut osc_a, &mut osc_b, &sine_wave, &saw_wave, *method, 0.5,   // 50% mix
                false, // No sync
            );
        }

        // Save to WAV file
        let output_file = format!("octane_{}_test.wav", name);
        match save_samples_to_wav(&output_file, &output_samples, sample_rate) {
            Ok(_) => println!("Successfully saved to {}", output_file),
            Err(e) => println!("Error saving WAV file: {}", e),
        }
    }

    // Test with different waveform combinations
    let waveform_pairs = [
        (&sine_wave, &triangle_wave, "sine_triangle"),
        (&sine_wave, &square_wave, "sine_square"),
        (&triangle_wave, &saw_wave, "triangle_saw"),
    ];

    for (wave_a, wave_b, name) in waveform_pairs.iter() {
        // Initialize oscillators
        let mut osc_a = OctaneOscillatorState::new();
        let mut osc_b = OctaneOscillatorState::new();

        // Set frequencies
        osc_a.update_phase_delta(440.0, sample_rate);
        osc_b.update_phase_delta(220.0, sample_rate);

        // Generate samples with Mult method
        println!("Generating {} waveform combination...", name);
        for i in 0..num_samples {
            output_samples[i] = octane_synthesize(
                &mut osc_a,
                &mut osc_b,
                wave_a,
                wave_b,
                CombineMethod::Mult,
                0.7,  // 70% mix
                true, // With sync
            );
        }

        // Save to WAV file
        let output_file = format!("octane_{}_test.wav", name);
        match save_samples_to_wav(&output_file, &output_samples, sample_rate) {
            Ok(_) => println!("Successfully saved to {}", output_file),
            Err(e) => println!("Error saving WAV file: {}", e),
        }
    }

    println!("Test completed. Check the generated WAV files.");
    println!("Files are saved in the project directory.");
}
