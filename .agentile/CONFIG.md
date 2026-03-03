# CONFIG.md — Project Configuration

> Edit this file to match your project. The agent reads this to understand tooling and conventions.

## Project Identity
```yaml
project_name: ""
description: ""
version: "0.1.0"
repository: ""
owner: ""
```

## Tech Stack
```yaml
language: ""           # e.g., TypeScript, Python, Rust, Go
framework: ""          # e.g., Next.js, Express, FastAPI, Actix
runtime: ""            # e.g., Node 20, Python 3.12, Rust nightly
package_manager: ""    # e.g., npm, yarn, pnpm, pip, cargo
```

## Testing
```yaml
test_runner: ""        # e.g., jest, vitest, pytest, cargo test
bdd_framework: ""      # e.g., cucumber-js, behave, cucumber-rs
coverage_target: 90    # Minimum % coverage before a feature is considered done
coverage_tool: ""      # e.g., c8, istanbul, coverage.py
```

## CI/CD
```yaml
ci_platform: ""        # e.g., GitHub Actions, GitLab CI, CircleCI
deployment_target: ""  # e.g., Vercel, AWS, Docker, bare metal
containerized: false   # true/false
```

## Documentation
```yaml
doc_format: "markdown" # markdown, jsdoc, sphinx, rustdoc
api_docs: false        # Generate API documentation?
changelog: true        # Maintain CHANGELOG.md?
```

## Agent Preferences
```yaml
verbosity: "concise"         # concise | detailed | minimal
report_frequency: "per-task" # per-task | per-sprint | on-request
auto_commit: false           # Should the agent commit automatically?
branch_per_feature: true     # Create a branch for each feature?
ask_before_refactor: false   # Ask human before refactoring?
```

## Custom Rules
```yaml
# Add project-specific rules the agent must follow
custom_rules:
  # - "All API routes must have rate limiting"
  # - "Use snake_case for database columns"
  # - "Every component must have a Storybook story"
```
