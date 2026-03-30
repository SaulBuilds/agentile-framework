# Documentation Rules

Documentation is part of the product contract. If code changes truth, docs change with it.

## Principles

1. One source of truth per topic.
2. Link instead of duplicating.
3. Docs ship with the code that changes them.
4. Historical context is archived, not silently rewritten.
5. Starter boilerplate must not masquerade as project truth.

## Required Root Documents

### `README.md`

The root README should describe the actual project, not the framework starter.

Gate:
- if the repo still uses starter framing, fix it early
- if project-facing behavior changes, update the README in the same PR

### `SPIRIT.md`

The root spirit defines local collaboration norms and what the project is protecting.

Gate:
- every adopted project should have one
- if governance or collaboration norms change, update it explicitly
- do not let agents quietly rewrite it to bless shortcuts

### `CHANGELOG.md`

Gate: user-facing changes should update `CHANGELOG.md`.

## Module READMEs

Module READMEs are required where a module has meaningful public behavior or operational expectations.

Gate: if a module's public contract changes, update its README.

## API Documentation

Public functions, types, interfaces, endpoints, or commands should have local documentation comments or equivalent reference docs.

## Insight Artifacts

Journals, essays, and case studies are documentation when they carry durable engineering truth.

Gate:
- if a review, debugging cycle, or sprint produces novel reusable insight, capture it
- do not rely on chat logs as the only record

## Canonical Locations

| Document Type | Location |
|---------------|----------|
| Project overview | `README.md` |
| Project spirit | `SPIRIT.md` |
| Changelog | `CHANGELOG.md` |
| Sprint records | `.agentile/sprints/` |
| Journals | sprint `JOURNAL.md` and `.agentile/docs/journals/` |
| Essays | `.agentile/docs/essays/` |
| Case studies | `.agentile/docs/case_studies/` |
| Configuration | `.agentile/CONFIG.md` |
| ADRs / specs | `.agentile/` project-selected locations |

## Prohibited Patterns

| Pattern | Why It Is Wrong | Use Instead |
|---------|-----------------|-------------|
| Starter README left in place | Misleads every new contributor | Rewrite the root README early |
| Starter spirit left in place | The repo has no local intent | Rewrite `SPIRIT.md` from `SPIRIT_GUIDE.md` |
| Valuable lesson only in chat | Lost institutional memory | Journal, essay, or case study |
| Versioned doc filenames like `_v2` | Drift and confusion | Git history plus archive notes |
| Duplicate summaries | Competing truths | Link to the canonical source |

## Review Checklist

- [ ] Root README reflects the real project
- [ ] Root SPIRIT reflects current collaboration norms
- [ ] CHANGELOG updated if user-facing behavior changed
- [ ] Module docs updated where needed
- [ ] Sprint and insight artifacts reflect reality
- [ ] No duplicate or stale truth was introduced
