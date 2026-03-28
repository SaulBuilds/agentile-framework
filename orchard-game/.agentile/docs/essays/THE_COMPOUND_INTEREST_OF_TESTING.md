# The Compound Interest of Testing

*A retrospective on why test investment compounds and what each category of test actually protects.*

---

## The Pattern Tax

Every test follows a pattern. Once the pattern exists, each new test costs approximately 30 seconds to write. The first test in a category costs 10 minutes (design the pattern, set up the helpers, figure out the assertions). The 50th test costs 30 seconds (copy the pattern, change the input, change the assertion).

This is compound interest. The pattern is the principal. Each test is interest earned. The more tests you write, the cheaper the next test becomes, because the pattern handles the boilerplate.

## What Different Test Categories Actually Protect

### Unit Tests -- Prevent Regressions

These are the "does the function do what it says" tests. They catch regressions when code is modified. They are cheap to write, fast to run, and they form the baseline.

But unit tests do not catch *interactions*. A unit test verifies that `Service.process()` returns the right value. It does not verify that `Service.process()` calls `Logger.record_success()` after completion.

### Adversarial Tests -- Prevent Exploits

These are the "what happens when someone tries to break it" tests. Designed by thinking like an attacker:

- What if the input contains null bytes?
- What if the value overflows?
- What if you send concurrent requests?
- What if you use the system after it has been locked?

Each adversarial test documents an attack surface. Even if the test passes trivially, the test's *existence* is documentation. It says: "someone thought about this attack and verified the system handles it."

### Property Tests -- Prevent Classes of Bugs

Property-based tests generate thousands of random inputs and verify invariants hold for all of them. One property test replaces hundreds of hand-written examples. They are expensive to design (you have to think about the *property*, not the *example*) but they cover enormous input spaces.

### Integration Tests -- Prevent System Failures

These test the full pipeline: create -> process -> verify. They test cross-module interactions that no unit test covers.

### Formal Specifications -- Prevent Design Flaws

Tests verify *behavior*. Specs verify *properties*. A specification can explore millions of states and verify that no sequence of operations can produce an invalid outcome. No test suite can cover millions of scenarios. The model checker does it exhaustively.

## The Real Cost of Not Testing

Each critical failure is prevented by multiple layers: a unit test, an adversarial test, an integration test, AND a formal invariant. Defense in depth. If one layer misses the bug, another catches it.

## What This Teaches

### 1. Tests Are Documentation

The adversarial test suite is the best documentation of a system's security posture. It explicitly lists every attack surface considered. An auditor reading the adversarial tests knows exactly what has been tested and -- by omission -- what has not.

### 2. Formal Specs Find Different Bugs Than Tests

Tests verify specific scenarios. Specs verify all scenarios. Specs find design principle violations that no test catches because the violation requires a specific sequence that no test exercises.

### 3. The First Test Is Expensive, The Rest Are Cheap

The pattern tax pays off exponentially. This is why velocity increases over a session -- from 30 tests/hour at the start to 85 tests/hour by the end.

### 4. Real Backends Break Fake Tests

Swapping a fake backend for a real one will break tests that unknowingly depend on the fake. This is the Wittgenstein loophole applied to testing: the tests follow the *pattern* of testing the service, but they are actually testing the *fake backend*.

## The Compound Interest

Each test you write makes every future test more valuable, because the system's verified surface area grows, and each new test extends it in a dimension that all previous tests benefit from.

---

*The interest compounds. The next test is cheaper than the last. This is why you test: not because you might be wrong today, but because you will be wrong tomorrow, and the tests will catch it before the users do.*
