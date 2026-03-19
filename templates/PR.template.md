## Summary

<!-- 1-3 bullet points describing what this PR does and why -->

-
-
-

## Sprint Reference

<!-- Link to the sprint WP this PR addresses -->

- **Sprint:** S-<ID>
- **Work Package:** WP-<N>
- **Feature spec:** <!-- link to FEATURE.md if applicable -->

## Changes

<!-- Describe the technical changes. Group by module if touching multiple modules. -->

### `<module-name>`
- <change description>

### `<module-name>`
- <change description>

## Test Plan

<!-- How was this change tested? What tests were added? -->

- [ ] Unit tests added/updated: `<test names>`
- [ ] Integration tests added/updated: `<test names>`
- [ ] Full workspace tests pass
- [ ] Lint clean
- [ ] Formatted
- [ ] Frontend tests pass (if applicable)
- [ ] Contract tests pass (if applicable)

### Test Count

| Suite | Before | After | Delta |
|-------|--------|-------|-------|
| Tests | <N> | <N> | +<N> |

## Documentation

<!-- What documentation was updated? -->

- [ ] Module README updated (if API changed)
- [ ] CHANGELOG entry added (if user-facing change)
- [ ] Doc comments on new public items
- [ ] N/A -- no documentation changes needed

## Security Considerations

<!-- Does this PR touch security-sensitive code? -->

- [ ] This PR does NOT touch security-sensitive code
- [ ] This PR touches security-sensitive code and requires review from: @<reviewer>

## Formal Verification

<!-- Does this PR require TLA+ or invariant test updates? -->

- [ ] N/A -- no consensus or state machine changes
- [ ] TLA+ spec updated and TLC passes
- [ ] Invariant tests updated

## Checklist

- [ ] Conventional commit messages used
- [ ] Branch name follows conventions (`feature/`, `fix/`, etc.)
- [ ] No TODOs, FIXMEs, or stubs in new code
- [ ] No hardcoded secrets or credentials
- [ ] Co-Authored-By footer included (if AI-assisted)
