# Included Sub-Operators and Functional Value

This suite currently includes five sub-operators, each with a distinct role and runtime behavior.

| Sub-Operator | Plugin ID | Category | Functional Value | Runtime Signal |
|---|---|---|---|---|
| Oscilloscope | `oscilloscope` | Analyzer | Time-domain waveform visibility for fast signal inspection. | Sets `operators.oscilloscope.enabled=true` |
| Stereoscope | `stereoscope` | Analyzer | Stereo image inspection and channel relationship visibility. | Sets `operators.stereoscope.enabled=true` |
| Frequency Gate | `frequency_gate` | Dynamics | Frequency-selective gating path for cleanup/control workflows. | Sets `operators.frequency_gate.enabled=true` |
| Pitch/Frequency Gate Amp (PFGA) | `pfga` | Dynamics | Hybrid gate/amp path with locked validation semantics for production checks. | Sets `enabled`, `mode=hybrid`, `validation_layer=null_noise_floor`, `capture_policy=resample_working_master_final` |
| Bass Go Brrr | `bass_go_brrr` | Bass Enhancement | Dedicated bass enhancement stage for low-end emphasis use-cases. | Sets `operators.bass_go_brrr.enabled=true` |

## Verification in Code

- `default_operator_suite()` guarantees all supported sub-operators are included in one canonical registration path.
- Unit tests verify registration uniqueness, descriptor export, and runtime context writes from each operator.
