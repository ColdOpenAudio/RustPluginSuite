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
## Build Maintenance And Execution Discipline (Codex Brief Addendum)

The build must maintain architectural discipline as it approaches technical implementation.

Do not let PFGA become an unstructured pile of features.

The project must remain modular, testable, real-time safe, uninstall-safe, and expansion-ready.

## Non-Negotiable Build Maintenance Rules

1. Keep DSP separate from UI.
2. Keep rolling capture separate from DSP.
3. Keep installer/uninstaller separate from plugin runtime.
4. Keep file writing off the audio thread.
5. Keep presets, captures, config, and logs separated.
6. Keep all destructive actions manifest-based.
7. Keep capture behavior deterministic.
8. Keep every advanced feature disableable.
9. Keep every heavy feature lazy-loaded or inactive until used.
10. Keep the first version smaller than the final vision but structurally ready for expansion.

## Required Project Boundaries

### PFGA Core

Only audio processing, analysis, state, and parameter logic.

### PFGA Plugin

NIH-plug wrapper, parameters, host integration, UI bridge.

### Rolling Master Engine

Circular buffers, capture jobs, metadata, background writing, archive logic.

### Standalone App

Full capture browser, diagnostics, installer/uninstaller access, extended routing.

### Installer

Install, verify, repair, manifest creation.

### Uninstaller

Dry run, safe removal, preservation rules, purge confirmation.

### Tests

DSP tests, rolling master tests, installer/uninstaller tests, integration tests.

## Implementation Priority Lock

### Phase 1

- PFGA core audio path
- Bypass
- Input/output gain
- DC removal
- Basic frequency gate
- Basic pitch confidence
- Lookahead
- Wet/dry
- Input/output meters
- Basic rolling 15-second capture
- NIH-plug VST3/CLAP build
- Standalone shell
- Installer manifest
- Dry-run uninstaller

### Phase 2

- Full pitch/frequency amp logic
- Noise-floor learning
- Null validation
- Rolling 8-bar capture
- Pre/post capture
- Metadata snapshots
- Installer repair
- Verify install
- Plugin-only uninstall

### Phase 3

- Continuous archive
- A/B capture
- Error capture
- Full diagnostic capture
- Capture library
- Advanced visualizers
- Full purge workflow
- Cross-platform packaging polish

## Scope Control Rule

If a feature risks destabilizing Phase 1, it must be scaffolded but not completed.

Scaffold means:

- define the interface
- reserve the parameter/state location
- document the behavior
- add a disabled placeholder
- add a test stub

Do not force every future feature into the first working build.

## Performance Maintenance Rules

The build must maintain real-time safety.

### Audio thread may

- process audio
- update lightweight meters
- write to real-time-safe circular buffers
- read atomic parameter values
- emit lightweight event flags

### Audio thread must not

- allocate memory repeatedly
- write files
- open dialogs
- scan folders
- hash files
- serialize JSON
- call installer logic
- block on locks
- wait for background threads

## Rolling Master Maintenance Rules

Rolling Master must remain optional.

### Default

- enabled only when requested
- 15-second buffer available
- continuous archive off
- error capture limited to clipping and overload

### Advanced

- 8-bar capture
- pre/post capture
- A/B capture
- dry/wet pair
- diagnostic capture
- continuous archive

Continuous archive must never activate silently.

## Installer/Uninstaller Maintenance Rules

The plugin runtime must not own uninstall logic.

### Installer tool owns

- install paths
- manifests
- hashes
- support folders
- repair
- verification

### Uninstaller owns

- dry-run plan
- removal plan
- preservation rules
- purge confirmation
- uninstall logs

The uninstaller must preserve by default:

- presets
- config
- rolling master captures
- exported WAV files
- project files
- DAW sessions

## Quality Gate Before Codex Implementation

Codex should not proceed past scaffolding unless these questions are answered in code:

1. Can the PFGA core run without the UI?
2. Can the rolling master engine be disabled completely?
3. Can the plugin build without the standalone app?
4. Can the standalone app build without installer logic?
5. Can the installer run dry-run without copying files?
6. Can the uninstaller run dry-run without deleting files?
7. Can tests verify bypass, neutral mode, and null behavior?
8. Can a capture be committed without blocking the audio thread?
9. Can user captures survive uninstall by default?
10. Can every installed file be traced to the manifest?

## Paste-Ready Codex Prompt Addendum

Maintain strict modular architecture.

Do not combine DSP, UI, rolling capture, file writing, installer, or uninstaller logic into one tangled module.

PFGA core must remain usable as a standalone DSP library.

The plugin wrapper must only expose PFGA through NIH-plug.

The rolling master engine must use real-time-safe circular buffering and background file writing.

The installer and uninstaller must be separate tools from the plugin runtime.

The uninstaller must be manifest-based, dry-run capable, and preservation-first.

As implementation approaches, prioritize a stable Phase 1 over feature saturation.

Phase 1 must prove:

- PFGA core signal path
- bypass
- DC removal
- basic frequency gate
- basic pitch confidence
- lookahead
- wet/dry
- 15-second rolling capture
- metadata stub
- NIH-plug VST3/CLAP build
- standalone shell
- installer manifest creation
- dry-run uninstall

Phase 2 and Phase 3 features may be scaffolded, but they must not destabilize Phase 1.

Every advanced feature must have:

- a module boundary
- a disabled default state
- a test stub
- a documented expansion path
- no blocking behavior on the audio thread

Acceptance is not based on feature count.

Acceptance is based on:

- clean architecture
- deterministic audio behavior
- real-time safety
- safe installation
- safe uninstallation
- capture reproducibility
- preservation of user data
- clear expansion path

## Groupchat Version (Condensed)

Add one more constraint before this goes into implementation: the build needs to maintain architectural discipline as it approaches technical execution.

Do not let PFGA become one tangled feature pile.

Keep the system split cleanly:

- PFGA Core: DSP, pitch detection, frequency analysis, gate logic, amp logic, lookahead, null/noise validation.
- PFGA Plugin: NIH-plug wrapper, parameters, host integration, UI bridge.
- Rolling Master Engine: Circular buffer, capture modes, metadata, background file writing, archive logic.
- Standalone App: Capture browser, diagnostics, expanded routing, installer/uninstaller access.
- Installer: Install, verify, repair, manifest creation.
- Uninstaller: Dry run, safe removal, preservation rules, purge confirmation, uninstall logs.

Rules:

- DSP stays separate from UI.
- Rolling capture stays separate from DSP.
- Installer/uninstaller stays separate from plugin runtime.
- File writing never happens on the audio thread.
- Continuous archive stays off by default.
- Presets, config, rolling master captures, exported WAV files, project files, and DAW sessions are preserved by default.
- Every destructive action must be manifest-based.
- Every advanced feature must be disableable.
- Every heavy feature must stay inactive until used.

Phase 1 should prove:

- PFGA core signal path
- Bypass
- DC removal
- Basic frequency gate
- Basic pitch confidence
- Lookahead
- Wet/dry
- Basic meters
- 15-second rolling capture
- Metadata stub
- NIH-plug VST3/CLAP build
- Standalone shell
- Installer manifest creation
- Dry-run uninstall

Phase 2 and Phase 3 features can be scaffolded, but they should not destabilize Phase 1.

Acceptance is not feature count. Acceptance is clean architecture, deterministic audio behavior, real-time safety, safe installation, safe uninstallation, capture reproducibility, preserved user data, and a clear expansion path.
