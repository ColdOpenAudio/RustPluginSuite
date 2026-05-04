# PFGA — Pitch/Frequency Gate Amp with Null/Noise-Floor Validation

## Purpose

PFGA unifies three control/verification layers into one operational device and one test framework:

1. **Frequency Gate Layer** (band-aware reduction/opening)
2. **Pitch Gate Amp Layer** (pitch-confidence-aware preservation/emphasis)
3. **Null/Noise-Floor Layer** (truth-check and residue validation)

The locked capture decision is:

- **Resample capture is the working method** for iterative validation.
- **Master export is final confirmation only**.

## Canonical Signal Flow

```text
Input
→ DC removal
→ input trim
→ input scope
→ pitch detector
→ frequency analyzer
→ amplitude tracker
→ noise-floor learner
→ pitch confidence logic
→ frequency gate logic
→ pitch-aware amp logic
→ lookahead alignment
→ gain shaping
→ output scope
→ null/noise validation
→ output
```

## Core Questions PFGA Must Answer

For each frame/window/block, PFGA evaluates:

1. What pitch or note is present?
2. Which frequency areas contain useful energy?
3. What part of the signal is noise, silence, residue, or unwanted background?

Then it decides whether to open, reduce, amplify, or stay neutral.

## Layer Contract

### 1) Frequency Gate Layer

**Detect:**

- low/mid/high band energy
- per-band RMS
- per-band peak
- per-band learned floor

**Decide:**

- open/hold/close state per band
- reduction depth per band
- attack/release smoothing behavior

### 2) Pitch Gate Amp Layer

**Detect:**

- dominant frequency estimate
- MIDI note estimate
- cents deviation
- pitch stability
- pitch confidence
- pitched vs unpitched class

**Decide:**

- preserve when pitched confidence is strong
- reduce when unpitched/noisy dominates
- optionally emphasize harmonic region around stable pitch
- scale gain behavior by confidence

### 3) Null/Noise-Floor Layer

**Compare:**

- dry input
- wet output
- resample print
- bypass/neutral output
- learned floor model
- rejected residue bus (if exposed)

**Answer:**

- bypass parity status
- neutral-mode transparency status
- null residue expected vs unexpected
- useful-vs-garbage gain outcome

## Device Modes

1. **Clean Frequency Gate**
   - band cleanup for hiss/rumble/room tone.
2. **Pitch Gate**
   - pitch-confidence-driven opening/preservation.
3. **Pitch Amp**
   - pitch-aware emphasis without equal noise lift.
4. **Hybrid PFGA**
   - full combined mode (default advanced mode).
5. **Null Check Mode**
   - disable creative shaping and validate transparency.
6. **Noise Learn Mode**
   - learn silence/room/hiss bed for floor model.

## Ableton Capture Layout

```text
01_REF_SOURCE
02_PF_GATE_AMP_TEST
03_DRY_CAPTURE
04_WET_CAPTURE
05_RESAMPLE_PRINT
06_ROLLING_MASTER
07_NULL_TEST
08_NOISE_FLOOR_CAPTURE
09_MASTER_EXPORT_REFERENCE
```

## Required Test Material

- digital silence
- room tone
- pink noise
- white noise
- sine sweep
- low sine tone
- mid sine tone
- high sine tone
- bass phrase
- vocal phrase
- drum transient loop
- full mix loop
- noisy problem sample
- pitch-bending sample
- resampled/transposed sample

## Combined Test Protocol

For each source, capture and archive:

1. `DRY_CAPTURE`
2. `WET_CAPTURE`
3. `RESAMPLE_PRINT`
4. `NULL_TEST` result
5. `NOISE_FLOOR_CAPTURE` (when applicable)

## Validation Rules

1. **Bypass:** wet must match dry.
2. **Neutral mode:** enabled/no-op must still match dry closely.
3. **Frequency gate:** only intended bands reduce.
4. **Pitch gate:** stable pitch opens more reliably than unstable noise.
5. **Pitch amp:** useful pitched signal can rise without equal noise rise.
6. **Noise floor:** learned floor reduces predictably without deleting musical content.
7. **Null test:** unexpected residue means unwanted coloration or phase error.
8. **Resample:** print must match monitored playback.
9. **Master export:** final export must match approved resample print.

## Terminology

- Product-facing name: **Pitch/Frequency Gate Amp**
- Internal shorthand: **PFGA**
- Expanded technical name: **Pitch/Frequency Gate Amp with Null/Noise-Floor Validation**
