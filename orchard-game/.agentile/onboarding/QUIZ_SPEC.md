# Onboarding Quiz Specification

> **5 questions. 5 minutes. Your starting ELO and zooid assignment depend on the result.**

## Overview

The onboarding quiz is a 5-question adaptive assessment that determines a new contributor's starting ELO score and initial zooid assignment. Questions are drawn from a question bank across multiple categories, with difficulty adapting based on the first answer.

---

## Scoring

| Correct Answers | Starting ELO | Starting Tier | Auto-Assigned Zooid |
|----------------|-------------|---------------|-------------------|
| 0 | 0 | Novice | TECH_WRITER |
| 1 | 200 | Novice | TECH_WRITER |
| 2 | 400 | Novice | DEVELOPER (guided, all PRs need Expert review) |
| 3 | 600 | Novice | DEVELOPER |
| 4 | 800 | Journeyman | QA_ENGINEER or DEVELOPER (contributor's choice) |
| 5 | 1000 | Journeyman | Any Journeyman-tier zooid (contributor's choice) |

Maximum starting ELO from the quiz is 1000. Expert (1200+) and Master (1600+) tiers must be earned through contributions tracked by the ELO system.

---

## Adaptive Flow

```
START
  |
  v
Q1: Medium difficulty, random category
  |
  +-- CORRECT --> Q2-Q5 drawn from MEDIUM + HARD pool
  |
  +-- WRONG ----> Q2-Q5 drawn from EASY + MEDIUM pool
  |
  v
Q2: Different category than Q1
  |
  v
Q3: Different category than Q1 and Q2
  |
  v
Q4: Any remaining category
  |
  v
Q5: Any remaining category
  |
  v
SCORE: Count correct answers, assign ELO and zooid
```

---

## Question Categories

Customize these categories for your project. Each category should have Easy, Medium, and Hard questions.

### Recommended Categories

1. **Primary Language / Systems Programming** -- Language idioms, concurrency, error handling
2. **Domain Knowledge** -- Your project's core domain (blockchain, ML, web, etc.)
3. **Testing & Quality** -- TDD, property testing, fuzzing, regression testing
4. **Architecture & Design** -- ADRs, patterns, interface design, trade-offs
5. **Formal Methods** -- TLA+, model checking, invariants, state machines

### Question Format

Each question should have:
- 4 options (a-d)
- One correct answer
- A rationale explaining why the answer is correct
- A connection to the project's actual codebase or methodology

### Example Question

```
Question [N] of 5 -- [Category Name]

[Question text]

  (a) [Option A]
  (b) [Option B]
  (c) [Option C]
  (d) [Option D]

Correct: (X)
Rationale: [Why this is the right answer and why it matters for the project]
```

---

## Writing Your Question Bank

1. Write 3 questions per category (Easy, Medium, Hard) = 15 questions minimum
2. Questions should test understanding, not memorization
3. Wrong answers should be plausible (common misconceptions)
4. Rationales should be educational -- the quiz teaches, not just evaluates
5. Include questions that relate to your project's actual patterns and decisions

---

## Administration

### Presenting the Quiz

1. **Select Q1**: Pick a random category, use the medium difficulty question
2. **Evaluate Q1**: If correct, remaining pool = medium + hard. If wrong, pool = easy + medium.
3. **Select Q2-Q5**: One question per remaining category. Alternate difficulty levels.
4. **Score**: Count correct answers. Assign ELO per the scoring table.
5. **Assign zooid**: Per the scoring table.
6. **Record**: Add the contributor to `zooids/REGISTRY.md`.

### Anti-Gaming

- Questions must be presented one at a time
- The contributor cannot change a previous answer
- The quiz may be re-taken after 30 days
