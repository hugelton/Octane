//! Octane oscillator library for digital synthesis

/// Parameters for oscillator processing
pub struct OctaneProcessParams {
    pub osc_a_freq: f32,
    pub osc_b_freq: f32,
    pub method: CombineMethod,
    pub alpha: f32,
    pub sync: bool,
    pub sample_rate: u32,
}

/// Combine methods for the Octane
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CombineMethod {
    Fade,  // Cross-fade between oscillators
    Phase, // Phase distortion/modulation
    Mult,  // Multiplication (ring modulation)
           // Additional methods can be added here
}

/// Oscillator state structure for Octane
pub struct OctaneOscillatorState {
    pub phase: u32,
    pub phase_delta: u32,
}

impl Default for OctaneOscillatorState {
    fn default() -> Self {
        Self::new()
    }
}

impl OctaneOscillatorState {
    pub fn new() -> Self {
        Self {
            phase: 0,
            phase_delta: 0,
        }
    }

    /// Update phase increment based on frequency
    pub fn update_phase_delta(&mut self, frequency: f32, sample_rate: u32) {
        self.phase_delta = ((frequency * (1u64 << 32) as f32) / sample_rate as f32) as u32;
    }

    /// Advance the oscillator phase
    pub fn advance_phase(&mut self) {
        self.phase = self.phase.wrapping_add(self.phase_delta);
    }

    /// Get the current LUT index from phase
    pub fn get_lut_index(&self) -> usize {
        ((self.phase >> 22) & 0x3FF) as usize
    }
}

/// Main synthesis function for Octane oscillators
pub fn octane_synthesize(
    osc_a_state: &mut OctaneOscillatorState,
    osc_b_state: &mut OctaneOscillatorState,
    osc_a_lut: &[i16],     // Oscillator A lookup table
    osc_b_lut: &[i16],     // Oscillator B lookup table
    method: CombineMethod, // Combination method
    alpha: f32,            // Mix ratio (0.0-1.0)
    sync: bool,            // Sync oscillator A to B
) -> i16 {
    // Save previous phase for sync
    let prev_phase_b = osc_b_state.phase;

    // Advance phases
    osc_a_state.advance_phase();
    osc_b_state.advance_phase();

    // Handle sync
    if sync && (prev_phase_b > osc_b_state.phase) {
        osc_a_state.phase = 0;
    }

    // Get waveform indices
    let index_a = osc_a_state.get_lut_index();
    let index_b = osc_b_state.get_lut_index();

    // Get waveform values
    let sample_a = osc_a_lut[index_a];
    let sample_b = osc_b_lut[index_b];

    // Process based on combine method
    let result = match method {
        CombineMethod::Fade => {
            // Cross-fade implementation
            let scale = 1024;
            let alpha_fixed = (alpha * 1024.0) as i32;
            ((sample_a as i32 * (scale - alpha_fixed) + sample_b as i32 * alpha_fixed) >> 10) as i16
        }

        CombineMethod::Phase => {
            // Phase distortion implementation
            let phase_offset = sample_b as i32 * 12;
            let phase_offset = (phase_offset * (alpha * 1024.0) as i32) >> 10;
            let phase_offset = phase_offset / 32;
            let final_index = ((index_a as i32 + phase_offset) & 0x3FF) as usize;

            // Safety check for index bounds
            if final_index < osc_a_lut.len() {
                osc_a_lut[final_index]
            } else {
                0
            }
        }

        CombineMethod::Mult => {
            // Ring modulation implementation
            let modulated = (sample_a as i32 * sample_b as i32) >> 9;

            // Dry/wet mix
            let alpha_fixed = (alpha * 1024.0) as i32;
            let result = (sample_a as i32 * (1024 - alpha_fixed) + modulated * alpha_fixed) >> 10;

            // DC offset removal simplified
            result as i16
        }

        // Handle any additional methods that might be added later
        #[allow(unreachable_patterns)]
        _ => {
            // Default fallback for future methods
            ((sample_a as i32 + sample_b as i32) >> 1) as i16
        }
    };

    // Amplify and clamp the result
    (result as i32 * 64).clamp(-32768, 32767) as i16
}

/// Convenient wrapper function for Octane oscillator processing
pub fn octane_process(osc_a_lut: &[i16], osc_b_lut: &[i16], params: &OctaneProcessParams) -> i16 {
    // Initialize oscillator states
    let mut osc_a = OctaneOscillatorState::new();
    let mut osc_b = OctaneOscillatorState::new();

    // Set phase increments based on frequencies
    osc_a.update_phase_delta(params.osc_a_freq, params.sample_rate);
    osc_b.update_phase_delta(params.osc_b_freq, params.sample_rate);

    // Run synthesis
    octane_synthesize(
        &mut osc_a,
        &mut osc_b,
        osc_a_lut,
        osc_b_lut,
        params.method,
        params.alpha,
        params.sync,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillator_phase_increment() {
        let mut osc = OctaneOscillatorState::new();
        osc.update_phase_delta(440.0, 44100);

        let initial_phase = osc.phase;
        osc.advance_phase();

        assert_ne!(initial_phase, osc.phase);
        assert_eq!(osc.phase, osc.phase_delta);
    }

    #[test]
    fn test_fade_combine_method() {
        let mut osc_a = OctaneOscillatorState::new();
        let mut osc_b = OctaneOscillatorState::new();

        // Simple test waveforms
        let wave_a = vec![512i16; 1024]; // Constant value
        let wave_b = vec![-512i16; 1024]; // Constant value

        // With alpha = 0.5, should get average of the two values
        let result = octane_synthesize(
            &mut osc_a,
            &mut osc_b,
            &wave_a,
            &wave_b,
            CombineMethod::Fade,
            0.5,
            false,
        );

        // Should be close to zero (might not be exactly due to fixed-point math)
        assert!(result.abs() < 10);
    }
}
pub mod utils;
