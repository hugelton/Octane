# Octane

Octane is a Rust implementation of the core compund oscillator engine used in the Eurorack module K102E (https://hugelton.com/k102e.html).

## Overview

This library extracts the compund oscillator component from the K102E oscillator and makes it reusable in other projects. It functions as a compound oscillator that combines two oscillators by accepting lookup tables (LUT) and pitch information.

## Synthesis Methods

Octane provides three different compund methods:

1. **Fade**: Cross-fades between the outputs of two oscillators. The mixing ratio can be controlled using the α parameter.
2. **Phase**: Uses the output of one oscillator to modulate the phase of the other. This generates waveforms with complex harmonic structures.
3. **Mult**: Multiplies the outputs of two oscillators (ring modulation). This creates metallic sounds with non-linear harmonic structures.

Oscillator sync functionality is also implemented, allowing for richer sound design possibilities.

## Examples

The project includes two sample implementations:

- **basic_test**: Tests all synthesis methods and various waveform combinations, outputting them as WAV files. Static.
- **synth_app**: Performs real-time synthesis, generating audio with parameters that change over time.

## Usage

This library can be incorporated into projects requiring waveform synthesis, such as digital audio synthesizers, music applications, and sound design tools.

## License

Released under the MIT License. See the LICENSE file for details.
