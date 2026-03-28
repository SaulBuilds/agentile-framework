# DEBUGGING Workflow

> The original workflows (FEATURE, SPRINT, REVIEW) cover building and checking.
> This workflow covers understanding and fixing.

## When to Use

Use DEBUGGING when:
- A feature that should work doesn't
- The system is in an unknown state
- Multiple things appear broken at once
- You've been fixing symptoms for > 30 minutes without progress

## The 6-Step Cycle (OHVIRF)

### 1. OBSERVE -- Don't touch anything

**Gate: No code changes until observation is complete.**

Map the actual state of the system:
- What processes are running?
- What ports are in use?
- What database/service is each component connected to?
- What do the logs say? (not what you think they say)
- What does the user actually see vs what they should see?

**Deliverable**: A state snapshot written down (not in your head).

### 2. HYPOTHESIZE -- Write it down before opening any file

State your theory as a testable claim:
- BAD: "The system is broken"
- GOOD: "The API shows 0 results because `get_items` reads from the test database (port 5433), which is empty, instead of the production database (port 5432) where the data was inserted"

**Gate: The hypothesis must be falsifiable. If you can't describe how to disprove it, it's not specific enough.**

### 3. VERIFY -- Prove it with data, not code

Run commands. Check responses. Read actual values.
- Query the API
- Check the config
- Compare expected vs actual values
- Read the logs

**Gate: Either confirm the hypothesis with evidence, or disprove it and go back to step 2.**

### 4. ISOLATE -- Find the minimal reproduction

What is the smallest change that demonstrates the bug?
- "If I query port 5433 instead of 5432, the result is empty"
- "If I change the config to use the production database, it works"

**Gate: You should be able to explain the bug in one sentence with specific values.**

### 5. FIX -- Now write the code

You understand the root cause. You know what needs to change. Write the fix.

Rules:
- One fix per commit
- The commit message explains the root cause, not just the symptom
- If the fix touches more than 3 files, write a plan first

### 6. REGRESS -- Check that nothing else broke

- Run the test suite
- Check the other features
- Verify the original bug is fixed
- Check that the fix didn't introduce new issues

**Gate: Tests pass. The user confirms the fix. The state snapshot from step 1 is now correct.**

## Anti-Patterns

| Pattern | Problem | Alternative |
|---------|---------|-------------|
| Fix-and-pray | Change code, rebuild, hope it works | OBSERVE first |
| Symptom chasing | Fix the visible error without understanding why | HYPOTHESIZE the root cause |
| Shotgun debugging | Change 5 things at once | ISOLATE to one variable |
| Memory debugging | "I think it was working before..." | VERIFY with data |
| Rebuild loop | Change, build (slow), test, change, build... | Check with fast validation first |

## When to Escalate

If after two full OHVIRF cycles the bug isn't fixed:
1. Write a journal entry documenting what you tried
2. Create a sprint work package for the fix
3. Consider whether the architecture needs to change (not just the code)
