# Case Study: The Persistence of Mock Implementations

**Why temporary code becomes permanent, and how framework-level discipline prevents it.**

---

## The $440 Million Mock

On August 1, 2012, Knight Capital Group deployed new trading software to eight servers. Seven received the update. The eighth still contained "Power Peg" -- a test program from 2003 designed to move stock prices artificially to verify other algorithms in a controlled environment.

A reused flag bit accidentally reactivated this dormant test code in the live environment. Knight executed over 4 million trades in 154 stocks in roughly 30 minutes. The loss: **$440 million.** Knight Capital was acquired a year later.

Power Peg was a mock. It was nine years old. Nobody knew it was there.

---

## The Numbers: How Long "Temporary" Code Lives

| What | How Long It Survives | Source |
|------|---------------------|--------|
| TODO comments (median) | **246 days** | Morlion, .NET repo study |
| TODO comments (mean) | **528 days** (~1.5 years) | Morlion |
| Low-quality TODOs vs good | **3.6x longer** to resolve | ACM TOSEM 2024 |
| Self-admitted technical debt | **25-60% never removed** | Potdar & Shihab, IEEE |
| Feature flags | **73% never removed** | FlagShark, 500+ enterprise teams |
| Technical debt as % of tech estate | **20-40%** | McKinsey |

The pattern is consistent: temporary code survives far longer than developers expect.

---

## Why This Happens: Six Mechanisms

### 1. Context Loss
The developer who wrote the mock leaves, changes teams, or forgets. The mock becomes indistinguishable from production code.

### 2. "It Works" Inertia
If a mock produces outputs that appear correct, there is no functional pressure to replace it. The UI renders. The tests pass. Users do not complain.

### 3. Missing Definition of Done
Teams without explicit criteria for "no mock code in production" systematically allow stubs through.

### 4. Mock Drift
When mocks and real services evolve independently, they diverge. Tests pass against the mock but would fail against production. The mock's behavior becomes the de facto specification.

### 5. Flag Rot
Feature flags intended to guard incomplete implementations are never removed. 73% become permanent.

### 6. Coupling Amplification
The need to mock indicates tight coupling. Rather than fixing the coupling, teams add more mocks. The cost of removing the mock grows faster than the cost of keeping it.

---

## How to Prevent It: Framework-Level Solutions

### Pattern 1: Data-Source-First Development
Before writing any frontend component, define the command's return type AND its data source. If the source is "seed data" or "mock," the work is incomplete.

### Pattern 2: The Mock Registry
Every mock gets registered in a file that the build system can check. Development builds include the mock flag. Release builds do NOT. If any mock exists, the release build fails.

### Pattern 3: Acceptance Criteria Must Name the Data Source
Sprint WP acceptance criteria should include explicit data source requirements, not just "renders data."

### Pattern 4: Strangler Fig for Existing Mocks
Do not rip out all mocks at once. Write the real implementation alongside the mock, add a feature flag, run both paths in CI, remove the mock once the real path passes all tests.

### Pattern 5: The Mock Budget
Allow a fixed number of mocks per sprint, tracked explicitly. Each mock must have a replacement WP in the next sprint.

### Pattern 6: Formal Specs as Mock Detectors
A formal spec describes the real state machine. If the implementation returns hardcoded data, it cannot satisfy the spec's invariants. The spec is the contract between the UI and the backend -- mocks violate it by definition.

---

## The Lesson

Rules prevent mocks at the moment of creation. They do not prevent the CONDITIONS that make mocks the path of least resistance.

What prevents those conditions:

1. **Acceptance criteria that name the data source**
2. **A mock registry that blocks release builds**
3. **Integration tests that use real data sources**
4. **Mock budgets per sprint with replacement WPs**
5. **Formal specs as contracts**

The $440 million lesson from Knight Capital: mock code does not announce itself. It looks like production code. It works like production code. Until it does not.

---

*References:*
- Knight Capital: Henrico Dolfing case study, SEC filings
- TODO lifespans: Peter Morlion (.NET study), ACM TOSEM 2024
- Feature flag rot: FlagShark (500+ enterprise teams)
- Technical debt economics: McKinsey Digital 2020-2024
- Strangler Fig: Martin Fowler, Shopify Engineering
