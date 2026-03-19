# Documentation Rules

> Documentation is a first-class deliverable. Stale docs are worse than no docs.

---

## Principles

1. **One source of truth per topic.** If a topic is documented in two places, one of them is wrong.
2. **Link, don't duplicate.** Reference the authoritative document instead of copying content.
3. **Docs ship with code.** Documentation updates are part of the same PR as the code change.
4. **Stale docs are archived, not deleted.** Move outdated docs to `archive/` with a date prefix.

---

## Required Documentation

### Crate/Module READMEs

**GATE:** Every crate or module in the workspace must have a `README.md` at its root. Use the `CRATE_README.template.md` template.

Required sections:
- **Purpose** -- one paragraph explaining what the crate does and why it exists
- **Modules** -- list of top-level modules with one-line descriptions
- **Public API** -- key types, traits, and functions with brief usage examples
- **Tests** -- how to run tests for this crate
- **Dependencies** -- notable external dependencies and why they are used

### CHANGELOG

**GATE:** Every PR that introduces a user-facing change (new feature, behavior change, bug fix, deprecation) must add an entry to `CHANGELOG.md` in the repository root.

Format:
```markdown
## [Unreleased]

### Added
- Feature description -- Sprint WP-X.Y

### Changed
- Change description

### Fixed
- Fix description

### Removed
- Removal description
```

Entries reference the sprint WP where applicable.

### API Documentation

**GATE:** Every public function, struct, enum, and trait must have a documentation comment.

Minimum requirements:
- One-line summary
- Parameter descriptions for non-obvious parameters
- Return value description
- Errors section if the function returns a Result type
- Panics section if the function can panic (and explain why it will not in practice)

---

## Document Locations

| Document Type | Location | Governance |
|---------------|----------|------------|
| Project README | `README.md` (repo root) | Updated by any contributor |
| Crate READMEs | `<crate>/README.md` | Updated when crate API changes |
| Architecture decisions | `.agentile/planset/` | ADR template, append-only |
| API guides | `docs/guides/` | Updated with API changes |
| Technical deep-dives | `docs/technical/` | Updated with architecture changes |
| Sprint records | `.agentile/sprints/` | Immutable after sprint close |
| Audit reports | `.agentile/audits/` | Immutable after creation |
| Configuration reference | `.agentile/CONFIG.md` | Single source of truth |
| Formal specs | `.agentile/formal/` | TLA+ files with README |

---

## Archiving

When a document becomes outdated:

1. Move it to `archive/` with a date prefix: `archive/YYYY-MM-description.md`
2. Add a one-line note at the top: `> Archived on YYYY-MM-DD. Superseded by <new-path>.`
3. Update any references that pointed to the old location.

**DO NOT** delete documentation. Historical context has value.

**DO NOT** create versioned filenames like `roadmap_v2.md`. Use git for versioning. Use `archive/` for superseded documents.

---

## Prohibited Patterns

| Pattern | Why It Is Wrong | What to Do Instead |
|---------|-----------------|---------------------|
| `*_PROGRESS.md` | Ephemeral status; use sprint DAILY.md | Update sprint `DAILY.md` |
| `*_COMPLETION.md` | One-time artifact; archive immediately | Write sprint `REPORT.md` |
| `*_SUMMARY.md` | Duplicates other docs | Link to the source of truth |
| `*_PLAN.md` at repo root | Sprawl; use sprint planning | Create sprint in `.agentile/sprints/` |
| `*_v2.md` | Version confusion | Use git, archive the old version |
| README in every directory | Noise; most add no value | Only where a README is genuinely needed |

---

## Review Checklist

Before merging any PR, verify:

- [ ] New public items have doc comments
- [ ] Crate README is current if the crate's API changed
- [ ] CHANGELOG has an entry if the change is user-facing
- [ ] No duplicate documentation was created
- [ ] Links point to existing files (no broken references)
- [ ] Archived docs have date prefixes and supersession notes
