---
created: 2026-04-19T22:30:00Z
branch: main
author: codex
sprint: planning
status: active
---

# Evaluation And Datasets

This document defines:

- the evaluation workbench the product needs
- the controls the operator must have
- the datasets we should start with
- the datasets that need extra legal review before production use

## Evaluation Workbench

The evaluation view should be easy to use during fast creative iteration.

Minimum layout:

1. Candidate strip
   - run id
   - preset name and hash
   - seed
   - artifact hashes
   - current status
2. Playback and preview panel
   - play or stop audio
   - preview MIDI note roll
   - transport position
3. Change summary panel
   - preset diff
   - live patch diff
   - prior baseline reference
4. Metrics panel
   - objective metrics
   - human scores
   - aggregate reward with visible weights
5. Action bar
   - reject
   - keep for reference
   - queue follow-up search
   - promote locally
   - request publish approval

## Required Objective Metrics

The first evaluation loop should compute:

- note density
- pitch range
- pitch entropy
- inter-onset interval variance
- rhythmic repetition score
- silence ratio
- clip peak and RMS
- loudness safety margin
- deterministic replay status
- render latency

Later metrics can add:

- harmonic novelty
- motif recurrence
- state controllability score
- state observability score
- live mutation responsiveness

## Required Human Rating Controls

Every reviewed run should support explicit 1-5 or 1-7 ratings for:

- musicality
- novelty
- controllability
- prompt alignment
- reuse potential
- live usefulness

Optional free-text fields:

- what worked
- what failed
- next experiment idea
- publish notes

## Reward Calculation Policy

The system must store:

- raw objective metrics
- raw human scores
- named reward weights
- aggregate reward

Hard rules:

- no imputed human scores
- no overwriting raw inputs
- changing weights creates a new reward record, not an in-place edit

## Evaluation Dataset Strategy

We need four dataset classes, not one giant corpus:

1. Symbolic composition data
   - for preset design, note-mapping analysis, and structure priors
2. Audio and multitrack data
   - for render evaluation, source-aware testing, and future live mixing analysis
3. Music-language data
   - for prompt alignment and caption-style evaluation
4. Interaction and preference data
   - for recommendation/feed shaping and reward learning

## Recommended Starting Dataset Stack

### Tier A: Start Immediately

These are the best initial candidates for meaningful work right now.

| Dataset | Use | Why it matters | License / use notes | Recommendation |
|---------|-----|----------------|---------------------|----------------|
| PDMX | symbolic training and preset mining | large public-domain MusicXML corpus with metadata | public-domain corpus, open release | best symbolic baseline |
| Slakh2100 | multitrack audio plus aligned MIDI | good for render evaluation and arrangement analysis | CC BY 4.0 | best multitrack starter |
| NSynth | instrument-note timbre library | good for timbre descriptors and simple synth benchmarking | CC BY 4.0 | best note-level audio starter |
| MusicNet | classical audio plus note labels | good for transcription-aware and structure-aware evaluation | freely licensed and public-domain source recordings per metadata | strong secondary starter |
| Song Describer | audio-caption evaluation | directly useful for prompt alignment and caption scoring | CC BY-SA 4.0 | best caption-eval starter |

### Tier B: Use For Research, Not Production By Default

| Dataset | Risk | Why |
|---------|------|-----|
| MAESTRO | non-commercial | CC BY-NC-SA 4.0 |
| MTG-Jamendo | non-commercial / special permission | dataset page says non-commercial research and academic use only unless separately licensed |
| MUSDB18 | academic-use gate | hosted with access approval and academic-only terms |
| MusicCaps | source-rights complexity | based on 10-second clips from AudioSet / YouTube, useful for evaluation but not my first production corpus |
| Yambda | license-policy ambiguity | dataset card shows Apache-2.0 but also says published exclusively for scientific and research purposes |

### Tier C: License Review Required Before Production Use

| Dataset | Risk | Why |
|---------|------|-----|
| Lakh MIDI Dataset | provenance / attribution complexity | dataset is CC BY 4.0, but files were scraped from public web sources and individual attribution is not consistently recoverable |
| JamendoMaxCaps and related large caption corpora | downstream license inheritance | useful, but derived from track collections with varying rights and should be reviewed before product use |

## Dataset Procurement Cheat Sheet

Procure these in order:

1. PDMX
   - purpose: symbolic baseline and preset mining
   - what to store: release URL, DOI, checksum, subset choice, transformation script hash
   - target use class: `production_allowed` after registry review
2. Slakh2100
   - purpose: multitrack render evaluation and arrangement experiments
   - what to store: dataset version, checksum, subset split, MIDI-to-audio pairing manifest
   - target use class: `production_allowed`
3. NSynth
   - purpose: note-level timbre analysis, simple renderer regression tests, sound descriptor grounding
   - what to store: instrument metadata, split choice, sample-rate normalization script hash
   - target use class: `production_allowed`
4. MusicNet
   - purpose: note-aligned audio validation and classical structure evaluation
   - what to store: metadata provenance file, split config, resampling transform hash
   - target use class: `license_review_required` until track-level provenance import is confirmed in registry
5. Song Describer
   - purpose: prompt alignment and caption-style evaluation
   - what to store: caption version, audio license file, filtering rules, attribution strategy
   - target use class: `license_review_required` because of share-alike obligations

## Internal Dataset Registry Fields

Every dataset record must include:

- `dataset_id`
- `display_name`
- `source_url`
- `citation`
- `license_name`
- `commercial_use_status`
- `redistribution_status`
- `approved_use_class`
- `checksum_manifest`
- `local_storage_path`
- `dataset_version`
- `split_policy`
- `transform_pipeline_hash`
- `parent_datasets`
- `operator_approval_id`
- `notes`

## Human Feedback Dataset We Must Build Ourselves

External datasets will not cover our actual product loop.

We need an internal feedback corpus with:

- operator prompt
- preset hash
- seed
- session context
- artifact hashes
- objective metrics
- human ratings
- free-text critique
- next-step decision
- whether the run was promoted, rejected, or published

This becomes the most valuable corpus for the adaptation layer because it reflects our actual tool surface and desired taste.

## Recommended First Internal Collection Plan

Phase 1:

- 100-250 reviewed runs from the demo preset family
- 2-3 operators
- one rating pass per run

Phase 2:

- 1,000+ reviewed runs
- A/B comparisons between candidate pairs
- prompt families for mood, density, energy, and texture

Phase 3:

- preference modeling and search-policy tuning
- only after the data registry and approval trail are stable

## Minimum Data We Need Before Adaptation

Do not start adaptation or reward-model tuning until we have:

1. at least one approved symbolic dataset
2. at least one approved multitrack or note-level audio dataset
3. at least one caption/evaluation dataset
4. at least 100 internal reviewed runs
5. a working run-manifest schema
6. a dataset registry with policy enforcement

## Easy-To-Use Operator Controls

The operator must be able to:

- compare two or more candidates side by side
- listen without leaving the evaluation screen
- see preset and parameter diffs in plain language
- mark a run for follow-up without publishing it
- export reviewed runs as CSV or JSONL
- filter by prompt family, preset family, score band, and date

## Research Basis

- PDMX:
  - https://zenodo.org/records/14984509
- Slakh2100:
  - https://www.slakh.com/
- NSynth:
  - https://magenta.tensorflow.org/datasets/nsynth/
- MusicNet:
  - https://zenodo.org/records/5120004
- Song Describer:
  - https://zenodo.org/records/10072001
- MAESTRO:
  - https://magenta.withgoogle.com/datasets/maestro
- MTG-Jamendo:
  - https://mtg.github.io/mtg-jamendo-dataset/
- Lakh MIDI Dataset:
  - https://colinraffel.com/projects/lmd/
- MUSDB18:
  - https://sigsep.github.io/datasets/musdb.html
- MusicCaps:
  - https://huggingface.co/datasets/google/MusicCaps
- Yambda:
  - https://huggingface.co/datasets/yandex/yambda
