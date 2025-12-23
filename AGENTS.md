## Project Overview

A hobby project building a custom music notation system from scratch with the following goals:
- Create an intuitive notation system more inclusive of non-western music theory
- Learn sound generation from the ground up
- Build lightweight terminal UI tools (avoiding bloated commercial software)
- Eventually train neural networks to encode/decode instruments, harmony, and melody
- **Goal is to assist musicians' creative process, NOT to generate audio slop or replace musicians**

## Tech Stack

- **Language**: Rust
- **TUI Framework**: ratatui
- **Audio**: cpal, hound, fundsp (DSP building blocks)
- **Future ML**: tch-rs or burn (for neural encoding/decoding)

## Notation System Design

### Core Principles
- **12-letter system**: A B C D E F G H I J K L (no sharps/flats)
- **Reference pitch**: A1 maps to traditional C3/Do3
- **Inclusive of non-western theory**: Can represent Turkish maqam, African-American traditions, Indian ragas, Indonesian gamelan, etc.
- **Visual representation**: Color-coded notes using circle of fifths spacing

### Color Mapping System
- Notes spaced by intervals of 7 semitones
- In traditional western music theory, this is called a "perfect fifth" (like C to G)
- **In our notation**: Moving by 7 letters (we call this a "seventh" since we count semitones)
  - Example: A + 7 semitones = H
  - Example: H + 7 semitones = C (wraps around)
  - Full cycle: A → H → C → J → E → L → F → A (continues...)
- Colors spaced by 210° on hue wheel (7/12ths of 360°)
  - A = hue 0° (red)
  - H = hue 210° 
  - C = hue 420° = 60° (wraps around)
  - J = hue 270°
  - ...and so on
- Colors quantized to Catppuccin palette for readability
- Formula: `note_n_hue = (n * 210) % 360` where n is the note index (0-11)
- Then snap to nearest Catppuccin color via color distance (deltaE or RGB Euclidean)

## Sound Generation Learning Path

### Phase 1: Basics (START HERE)
1. Generate simple waveforms (sine, square, sawtooth, triangle)
2. Write waveforms to WAV files
3. Understand: samples, sample rate (44.1kHz), bit depth (16-bit)
4. Basic formula: `sample = amplitude * sin(2π * frequency * time)`

### Phase 2: Physical Modeling
1. Implement Karplus-Strong algorithm (simple plucked string model)
2. Build ADSR envelope (Attack, Decay, Sustain, Release)
3. Add basic filters (low-pass, high-pass, band-pass)

### Phase 3: Analysis
1. FFT/spectral analysis to understand existing sounds
2. Understand harmonics and timbre
3. Analyze how different instruments create their characteristic sounds

### Phase 4: Neural Encoding (FUTURE)
1. Study models like NSynth (Google Magenta)
2. Build encoder: audio sample → latent vector (captures "what makes violin sound like violin")
3. Build decoder: latent vector → audio
4. Enable: timbre transfer, instrument interpolation, harmony/melody suggestions

## Key DSP Building Blocks

- **Oscillators**: Generate waveforms at specific frequencies
- **Filters**: Shape frequency content (low-pass, high-pass, band-pass)
- **Envelopes**: Control parameters over time (ADSR)
- **Delay/Echo**: Buffer and playback samples
- **Mixers**: Combine or scale signals
- **LFOs**: Low-frequency modulation for effects (vibrato, tremolo)
- **Reverb**: Simulate acoustic spaces

## Synthesis Methods to Explore

1. **Additive**: Combine sine waves at different frequencies/amplitudes
2. **Subtractive**: Start with rich waveform, filter it down
3. **FM**: Frequency modulation (Yamaha DX7 style)
4. **Wavetable**: Interpolate between stored waveforms
5. **Physical modeling**: Simulate actual instrument physics

## Project Components (To Build)

### 1. Notation Engine
- Parser for new 12-letter system
- Converter to/from traditional notation (for import/export)
- Support for microtonal variations (Turkish, Arabic makams)

### 2. TUI Interface (ratatui)
- Visual notation display with color-coded notes
- Piano roll / timeline view
- Real-time audio preview
- Lightweight, keyboard-driven workflow

### 3. Audio Engine
- Real-time synthesis
- Multiple instrument models
- MIDI compatibility (eventually)
- Export to audio files

### 4. ML Components (Future)
- Instrument encoder/decoder
- Harmony suggestion system (based on notation + rules)
- Melody generation assistant
- All outputs should be suggestions to augment creativity, not replace it

## Development Philosophy

- **Learn by doing**: Build everything from first principles
- **Lightweight tools**: Modern TUIs can be powerful without bloat
- **Inclusive design**: System should handle diverse music traditions
- **AI as assistant**: Neural nets suggest, humans decide
- **Open exploration**: Okay to experiment and discover what works

## Next Immediate Steps

1. Set up Rust project with ratatui + hound
2. Generate a 440Hz sine wave and write to WAV file
3. Play it back to confirm audio pipeline works
4. Implement color wheel → Catppuccin quantization
5. Build basic TUI that displays the 12 notes with their colors

## Resources to Reference

- Karplus-Strong algorithm (simple string synthesis)
- ADSR envelope shaping
- NSynth paper (for future neural encoding)
- Catppuccin color palette
- Turkish maqam system, Indian raga theory (for notation design)

---

**Remember**: This is a learning journey. Start simple, build incrementally, and enjoy discovering how sound works! AGENTS.md - Custom Music Notation System

## Project Overview

A hobby project building a custom music notation system from scratch with the following goals:
- Create an intuitive notation system more inclusive of non-western music theory
- Learn sound generation from the ground up
- Build lightweight terminal UI tools (avoiding bloated commercial software)
- Eventually train neural networks to encode/decode instruments, harmony, and melody
- **Goal is to assist musicians' creative process, NOT to generate audio slop or replace musicians**

## Tech Stack

- **Language**: Rust
- **TUI Framework**: ratatui
- **Audio**: cpal, hound, fundsp (DSP building blocks)
- **Future ML**: tch-rs or burn (for neural encoding/decoding)

## Notation System Design

### Core Principles
- **12-letter system**: A B C D E F G H I J K L (no sharps/flats)
- **Reference pitch**: A1 maps to traditional C3/Do3
- **Inclusive of non-western theory**: Can represent Turkish maqam, African-American traditions, Indian ragas, Indonesian gamelan, etc.
- **Visual representation**: Color-coded notes using circle of fifths spacing

### Color Mapping System
- Notes spaced by intervals of 7 semitones
- In traditional western music theory, this is called a "perfect fifth" (like C to G)
- **In our notation**: Moving by 7 letters (we call this a "seventh" since we count semitones)
  - Example: A + 7 semitones = H
  - Example: H + 7 semitones = C (wraps around)
  - Full cycle: A → H → C → J → E → L → F → A (continues...)
- Colors spaced by 210° on hue wheel (7/12ths of 360°)
  - A = hue 0° (red)
  - H = hue 210° 
  - C = hue 420° = 60° (wraps around)
  - J = hue 270°
  - ...and so on
- Colors quantized to Catppuccin palette for readability
- Formula: `note_n_hue = (n * 210) % 360` where n is the note index (0-11)
- Then snap to nearest Catppuccin color via color distance (deltaE or RGB Euclidean)

## Sound Generation Learning Path

### Phase 1: Basics (START HERE)
1. Generate simple waveforms (sine, square, sawtooth, triangle)
2. Write waveforms to WAV files
3. Understand: samples, sample rate (44.1kHz), bit depth (16-bit)
4. Basic formula: `sample = amplitude * sin(2π * frequency * time)`

### Phase 2: Physical Modeling
1. Implement Karplus-Strong algorithm (simple plucked string model)
2. Build ADSR envelope (Attack, Decay, Sustain, Release)
3. Add basic filters (low-pass, high-pass, band-pass)

### Phase 3: Analysis
1. FFT/spectral analysis to understand existing sounds
2. Understand harmonics and timbre
3. Analyze how different instruments create their characteristic sounds

### Phase 4: Neural Encoding (FUTURE)
1. Study models like NSynth (Google Magenta)
2. Build encoder: audio sample → latent vector (captures "what makes violin sound like violin")
3. Build decoder: latent vector → audio
4. Enable: timbre transfer, instrument interpolation, harmony/melody suggestions

## Key DSP Building Blocks

- **Oscillators**: Generate waveforms at specific frequencies
- **Filters**: Shape frequency content (low-pass, high-pass, band-pass)
- **Envelopes**: Control parameters over time (ADSR)
- **Delay/Echo**: Buffer and playback samples
- **Mixers**: Combine or scale signals
- **LFOs**: Low-frequency modulation for effects (vibrato, tremolo)
- **Reverb**: Simulate acoustic spaces

## Synthesis Methods to Explore

1. **Additive**: Combine sine waves at different frequencies/amplitudes
2. **Subtractive**: Start with rich waveform, filter it down
3. **FM**: Frequency modulation (Yamaha DX7 style)
4. **Wavetable**: Interpolate between stored waveforms
5. **Physical modeling**: Simulate actual instrument physics

## Project Components (To Build)

### 1. Notation Engine
- Parser for new 12-letter system
- Converter to/from traditional notation (for import/export)
- Support for microtonal variations (Turkish, Arabic makams)

### 2. TUI Interface (ratatui)
- Visual notation display with color-coded notes
- Piano roll / timeline view
- Real-time audio preview
- Lightweight, keyboard-driven workflow

### 3. Audio Engine
- Real-time synthesis
- Multiple instrument models
- MIDI compatibility (eventually)
- Export to audio files

### 4. ML Components (Future)
- Instrument encoder/decoder
- Harmony suggestion system (based on notation + rules)
- Melody generation assistant
- All outputs should be suggestions to augment creativity, not replace it

## Development Philosophy

- **Learn by doing**: Build everything from first principles
- **Lightweight tools**: Modern TUIs can be powerful without bloat
- **Inclusive design**: System should handle diverse music traditions
- **AI as assistant**: Neural nets suggest, humans decide
- **Open exploration**: Okay to experiment and discover what works

## Next Immediate Steps

1. Set up Rust project with ratatui + hound
2. Generate a 440Hz sine wave and write to WAV file
3. Play it back to confirm audio pipeline works
4. Implement color wheel → Catppuccin quantization
5. Build basic TUI that displays the 12 notes with their colors

## Resources to Reference

- Karplus-Strong algorithm (simple string synthesis)
- ADSR envelope shaping
- NSynth paper (for future neural encoding)
- Catppuccin color palette
- Turkish maqam system, Indian raga theory (for notation design)

