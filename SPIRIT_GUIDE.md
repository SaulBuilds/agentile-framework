# SPIRIT Guide

This guide exists so every adopted project writes its own root `SPIRIT.md` early, before the repo accumulates momentum in the wrong direction.

`SPIRIT.md` is not branding copy. It is the local constitution for how humans and agents should work together in this repository.

## When To Create Or Rewrite `SPIRIT.md`

Create or rewrite `SPIRIT.md` when:
- the project is adopting Agentile for the first time
- the repo still contains the starter spirit instead of project-specific intent
- the project changes phase in a meaningful way, such as greenfield to hardening or prototype to pre-prod
- reviews reveal repeated confusion about standards, boundaries, or counterpart expectations

If `SPIRIT.md` is missing, stale, or generic, fixing that comes before feature work.

## What `SPIRIT.md` Should Do

A good `SPIRIT.md` should:
- explain what this project is trying to protect
- state how humans and agents are expected to collaborate
- define a few non-negotiables that matter here
- explain how the spirit can be changed safely
- stay short enough that every contributor will actually read it

A good spirit is specific, honest, and enforceable.

## Drafting Loop

1. Read `README.md`, `.agentile/AGENT_ENTRY.md`, `.agentile/CONFIG.md`, and `.agentile/rules/CORE_RULES.md`.
2. Inspect the actual repo state: healthy, retrofit, failing CI, debugging, hardening, or pre-prod.
3. Draft `SPIRIT.md` using `.agentile/templates/SPIRIT.template.md`.
4. Make sure the spirit fits the actual project rather than the framework starter.
5. Before the first protected commit, set up governance for `SPIRIT.md` using `.agentile/docs/SPIRIT_PROTECTION.md`.

## Required Sections

Keep the document short, but cover these topics:

1. Mission
2. What we protect
3. Human-agent collaboration norms
4. Non-negotiable boundaries
5. Reflection and journaling commitments
6. Change control for the spirit itself

## Protecting The Spirit From Drift

The spirit must be protected from two kinds of failure:

1. External erosion: people editing the spirit casually to bless shortcuts.
2. Internal erosion: agents rewriting the spirit to justify whatever they already want to do.

Use these checks:
- do not let `SPIRIT.md` grow into a vague manifesto
- every hard claim in the spirit should map to a workflow, rule, or review habit
- changes to `SPIRIT.md` should include a rationale tied to observed behavior, not vibes
- if a spirit change lowers a quality bar, require explicit human review
- keep a short changelog section inside `SPIRIT.md` so drift is visible

## Protection Before First Commit

At minimum, protect:
- `SPIRIT.md`
- `SPIRIT_GUIDE.md`
- `.agentile/AGENT_ENTRY.md`
- `.agentile/rules/`
- `.agentile/workflows/`

Recommended first layer:
- protected default branch
- CODEOWNERS on spirit and governance files
- required reviews
- signed commits or signed tags where available

Optional stronger layer:
- store a signed hash of `SPIRIT.md`
- mirror approvals through an off-chain or on-chain multisig if the project is governance-heavy

See [`.agentile/docs/SPIRIT_PROTECTION.md`](.agentile/docs/SPIRIT_PROTECTION.md) for the concrete options.

## Minimal Checklist

Before coding begins in a new project:
- [ ] `SPIRIT.md` exists at the repo root
- [ ] `SPIRIT.md` reflects the actual project, not the starter
- [ ] `README.md` reflects the actual project, not the starter
- [ ] protection for spirit and governance files is planned or configured
- [ ] agents know to route through `.agentile/AGENT_ENTRY.md`
