# Onboarding Quiz Specification

> **5 questions. 5 minutes. Your starting ELO and zooid assignment depend on the result.**

## Overview

The onboarding quiz is a 5-question adaptive assessment that determines a new contributor's starting ELO score and initial zooid assignment. Questions are drawn from a 15-question bank across 5 categories, with difficulty adapting based on the first answer.

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
Q4: Any remaining category (may repeat if needed)
  |
  v
Q5: Any remaining category (may repeat if needed)
  |
  v
SCORE: Count correct answers, assign ELO and zooid
```

**Rules:**
- Q1 is always medium difficulty from a randomly selected category
- Q2-Q5 each come from a different category than the preceding question (when possible)
- No two questions from the same difficulty level appear consecutively
- All 5 categories should be represented across the 5 questions (one question per category)

---

## Question Bank

> **CUSTOMIZATION REQUIRED**: Replace the placeholder questions below with questions specific to your project. Each category should have 3 questions (Easy, Medium, Hard) for a total of 15 questions.

### Category A: {{QUIZ_CATEGORY_A_NAME}}

#### A1 -- Easy

{{QUIZ_A1_EASY}}

#### A2 -- Medium

{{QUIZ_A2_MEDIUM}}

#### A3 -- Hard

{{QUIZ_A3_HARD}}

---

### Category B: {{QUIZ_CATEGORY_B_NAME}}

#### B1 -- Easy

{{QUIZ_B1_EASY}}

#### B2 -- Medium

{{QUIZ_B2_MEDIUM}}

#### B3 -- Hard

{{QUIZ_B3_HARD}}

---

### Category C: {{QUIZ_CATEGORY_C_NAME}}

#### C1 -- Easy

{{QUIZ_C1_EASY}}

#### C2 -- Medium

{{QUIZ_C2_MEDIUM}}

#### C3 -- Hard

{{QUIZ_C3_HARD}}

---

### Category D: {{QUIZ_CATEGORY_D_NAME}}

#### D1 -- Easy

{{QUIZ_D1_EASY}}

#### D2 -- Medium

{{QUIZ_D2_MEDIUM}}

#### D3 -- Hard

{{QUIZ_D3_HARD}}

---

### Category E: {{QUIZ_CATEGORY_E_NAME}}

#### E1 -- Easy

{{QUIZ_E1_EASY}}

#### E2 -- Medium

{{QUIZ_E2_MEDIUM}}

#### E3 -- Hard

{{QUIZ_E3_HARD}}

---

## How to Write Good Quiz Questions

Each question should follow this format:

```markdown
**Question text here?**

- (a) Option A
- (b) Option B
- (c) Option C
- (d) Option D

**Correct: (X)**

*Rationale: Explain why this answer is correct and why the distractors are wrong. This makes the quiz educational, not just evaluative.*
```

### Category Design Guidelines

| Category | Should Test | Examples |
|----------|------------|---------|
| **A** | Primary language / systems knowledge | Language idioms, concurrency patterns, error handling |
| **B** | Domain-specific knowledge | Protocol behavior, data structures, core algorithms |
| **C** | Testing and quality practices | Test strategies, property testing, fuzz findings |
| **D** | Architecture and design | Design patterns, integration strategies, trade-offs |
| **E** | Formal methods or advanced topics | TLA+, invariants, safety/liveness properties |

### Difficulty Guidelines

| Level | What It Tests | Who Should Get It Right |
|-------|--------------|------------------------|
| **Easy** | Foundational knowledge, basic definitions | Everyone (including beginners) |
| **Medium** | Applied knowledge specific to the project | Experienced developers |
| **Hard** | Deep understanding requiring nuanced reasoning | Domain experts |

---

## Administration

### Presenting the Quiz

When administering the quiz to a contributor (human or AI):

1. **Select Q1**: Pick a random category, use the medium difficulty question
2. **Evaluate Q1**: If correct, remaining pool = medium + hard. If wrong, remaining pool = easy + medium.
3. **Select Q2-Q5**: One question per remaining category (ensure all 5 categories are covered across the 5 questions). Alternate between the two available difficulty levels in the pool.
4. **Score**: Count correct answers. Assign ELO per the scoring table.
5. **Assign zooid**: Per the scoring table.
6. **Record**: Add the contributor to REGISTRY.md with their starting ELO, tier, zooid, and date.

### Question Format for Delivery

Present each question as:

```
Question [N] of 5 -- [Category Name]

[Question text]

  (a) [Option A]
  (b) [Option B]
  (c) [Option C]
  (d) [Option D]
```

After the contributor answers, reveal the correct answer and rationale before moving to the next question. This makes the quiz educational, not just evaluative.

### Anti-Gaming

- Questions must be presented one at a time (no lookahead)
- The contributor cannot change a previous answer
- If a contributor is suspected of looking up answers, the quiz result stands -- their actual performance under the zooid's hard gates will reveal true competence, and ELO will adjust accordingly
- The quiz may be re-taken after 30 days if the contributor believes their initial score was unrepresentative
