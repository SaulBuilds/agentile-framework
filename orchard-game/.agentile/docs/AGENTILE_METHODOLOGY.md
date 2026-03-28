# The Agentile Methodology: A Framework for Human-AI Software Engineering

### An Essay on What Worked, What Broke, and What the Next Generation of Developers Should Know

---

## Preface

The Agentile methodology -- the fusion of AI-agent workflows with agile sprint discipline -- did not start as a methodology. It started as a mess. It became a methodology because the mess kept recurring in the same shapes, and the solutions kept hardening into rules.

This essay describes those rules and the failures that created them.

---

## Part I: The Problem

### What Goes Wrong When AI Agents Write Code

AI coding agents share a pathology: they want to write code immediately. Given a task, they produce code. Given a question, they produce code. Given silence, they produce code. This is not a bug. It is what they were trained to do.

The consequences are predictable:

1. **Context drift.** The agent forgets what it already built. It reimplements modules that exist. It contradicts decisions made three prompts ago.

2. **Documentation debt.** The agent produces code but not the documentation that explains why the code exists. Three weeks later, neither the human nor the agent remembers the reasoning.

3. **Test amnesia.** The agent writes features without tests, or writes tests that pass by accident.

4. **Sprint sprawl.** Without structured planning, work expands to fill available context. A "quick fix" becomes a refactor. A refactor becomes an architecture change.

5. **The stub problem.** Under pressure to show progress, the agent produces mock implementations, TODO comments, and placeholder functions. These are the cockroaches of software engineering: they survive every cleanup attempt and reproduce in the dark.

---

## Part II: The Rules That Emerged

### Rule 1: Plan Before You Code

The most important file in a repository using Agentile is `AGENT_ENTRY.md`. It describes the architecture, the commands, the conventions, and the prohibitions. The critical insight: **the first thing an AI agent does in a session determines the quality of everything that follows.**

### Rule 2: No Stubs, No TODOs, No Mocks

Early in development, agents produce functions that return hardcoded values, methods that print "TODO: implement," and test files that assert `true`. The cleanup cost is enormous. The rule is simple: if a feature cannot be fully implemented, it should not be started.

### Rule 3: Test Count Is the Metric

Every sprint records the test count. This creates a ratchet: the count can only go up. If a sprint starts at 1,704 tests and ends at 1,700, four tests were deleted, and that requires an explanation.

### Rule 4: Audits Are Dated Directories

Audit findings get lost across sprints. Timestamped audit directories with numbered documents create a clear paper trail. Never modify an audit after publication.

### Rule 5: The Sprint File Is the Source of Truth

When everything else is confusing, the sprint file must be clear. It is a dashboard, not a plan. It contains: which sprints are active, their status, and a table of work packages.

---

## Part III: Key Insights

### Sprint Modes: Launch vs. Hardening

Different phases demand different definitions of "done":

**Launch mode**: "Done" means the user can interact with it. Test the happy path. Ship fast.
**Hardening mode**: "Done" means an adversary cannot break it. Test the adversarial path. Take time.

### The Journal Habit

Journals are more honest than ADRs because they are written before decisions are finalized, in natural language, without a formal review process. They capture uncertainty. They capture "I was wrong about this."

### The Framework as External Memory

The `.agentile/` directory is not supplementary documentation. It is the agent's brain between sessions. CURRENT.md is working memory. DAILY.md is episodic memory. CONFIG.md is semantic memory. Without these files, the agent spends 10-15 minutes re-deriving context. With them, 30 seconds.

### Human Pushback Is the Highest-Value Quality Gate

The agent will optimize for "tests pass." The human must demand "feature works." This asymmetry is not a bug -- it is the fundamental dynamic of human-AI collaboration.

---

## Part IV: Recommendations

### If You Are Starting a New Project with AI

1. **Write your AGENT_ENTRY.md before you write any code.** Every minute spent on this file saves an hour of confused agent behavior later.

2. **Use `.agentile/sprints/CURRENT.md` from day one.** Even if your first sprint is "Sprint 1: Hello World."

3. **Enforce the test ratchet.** Record test counts in every daily log. Never merge code that reduces the count.

4. **Separate planning, auditing, and operations.** Use `.agentile/sprints/` for planning, `.agentile/audits/` for findings.

5. **Timestamp everything.** Documents without timestamps become indistinguishable across phases.

### If You Are an AI Agent Reading This

Your human is counting on you to maintain context across sessions. The way you do this is not by remembering -- you cannot. The way you do this is by reading the right files at the start of every session and writing the right files at the end.

Read `CURRENT.md` first. Always.
Write `DAILY.md` last. Always.
Everything in between is the work.

---

## Conclusion

The question is not whether AI can build software. The question is whether you, the human reading this, are willing to impose the discipline that turns AI-generated code into production-grade systems.

The framework exists. The evidence exists.

Use it.

---

*The Agentile methodology was developed through empirical observation of AI-assisted software development, refined across 35+ sprints and 1,100+ story points of delivered work.*
