# Spirit Protection

`SPIRIT.md` only matters if the repository treats it as real governance rather than decorative prose.

This document describes practical ways to protect:
- `SPIRIT.md`
- `SPIRIT_GUIDE.md`
- `.agentile/AGENT_ENTRY.md`
- `.agentile/rules/`
- `.agentile/workflows/`

## Recommended Minimum: Off-Chain Git Governance

For most projects, start here.

1. Protect the default branch.
2. Require pull requests for governance file changes.
3. Use `CODEOWNERS` so spirit and rule changes need explicit approval.
4. Require at least two approvals for governance changes where the hosting platform supports it.
5. Prefer signed commits or signed tags for governance updates.
6. Require the PR description to explain why the spirit changed and what observed behavior motivated it.

Use the starter snippet in [`../templates/CODEOWNERS.spirit.template`](../templates/CODEOWNERS.spirit.template).

## Stronger Off-Chain Pattern

When spirit changes are rare and high-signal:

1. Store a checksum of `SPIRIT.md` in a signed release tag or governance note.
2. Require a matching changelog entry in `SPIRIT.md`.
3. Mirror the approved hash into release notes or an audit log.

Example:

```bash
sha256sum SPIRIT.md
git tag -s spirit-v1 <commit>
```

## On-Chain Or Multisig Pattern

Use this only if the project already operates with real governance or release controls.

Options:
- store a hash of `SPIRIT.md` or a governance manifest in an on-chain registry
- require a multisig wallet or signer set to approve governance updates before release
- tie release promotion to a manifest signed by the required parties

This is most useful when:
- the repo controls protocol logic, treasury logic, or regulated workflows
- governance decisions already happen through an existing multisig
- releases must be attestable outside Git hosting

## What To Protect First

If you only have time for one pass, protect these paths:

```text
/SPIRIT.md
/SPIRIT_GUIDE.md
/.agentile/AGENT_ENTRY.md
/.agentile/rules/
/.agentile/workflows/
```

## Review Questions For A Spirit Change

Ask:
- what concrete failure, ambiguity, or drift triggered this change
- does this tighten clarity or merely add prose
- does this lower a gate; if so, who approved that tradeoff
- is the change specific enough to enforce

If the answer is fuzzy, the spirit change probably is too.

## Note

This is an engineering governance guide, not legal, financial, or security compliance advice.
